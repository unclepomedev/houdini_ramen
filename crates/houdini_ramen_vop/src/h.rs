#[derive(Debug, Clone)]
pub struct VopHair {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHair {
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

    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_s_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_s<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("s".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_hair_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_haircolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("haircolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_speccolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("speccolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_hair_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_width<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("width".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bump_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
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
    pub fn with_haircolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "haircolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_haircolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "haircolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHair {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hair"
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

pub trait VopHairOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
    /// Output pin: "Displacement Amount"
    fn out_amount(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("amount".to_string()),
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHairOutputs for VopHair {}
impl VopHairOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHair> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHairInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_roundness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn displacement_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn face_forward_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_dispn(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHairInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHair> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn compute_roundness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_roundness")
    }
    fn displacement_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("displacement_amount")
    }
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("displacenml1")
    }
    fn face_forward_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("face_forward_N")
    }
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lighting1")
    }
    fn normalize_dispn(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_dispN")
    }
    fn s_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_global")
    }
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_input")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopHairdiff {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHairdiff {
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

    pub fn set_hair_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_nn<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("nN".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_diffuse_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_kd<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Kd".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_diffuse_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_diff<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diff".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_kd(mut self, val: f32) -> Self {
        self.params.insert(
            "Kd".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Kd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHairdiff {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hairdiff"
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

pub trait VopHairdiffOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
    /// Output pin: "Illumination"
    fn out_illum(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("illum".to_string()),
        }
    }
    /// Output pin: "F"
    fn out_f(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("f".to_string()),
        }
    }
}

impl VopHairdiffOutputs for VopHairdiff {}
impl VopHairdiffOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHairdiff> {}

#[derive(Debug, Clone)]
pub struct VopHairmodel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHairmodel {
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

    pub fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_direction_from_eye_to_surface_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
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
    pub fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("I".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tip_direction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tip<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tip".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_conserve_energy_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_conserveenergy<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("conserveenergy".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_fake_rounding_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rounding_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rounding_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_primary_reflection_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_primary_component_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_label<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_label".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_secondary_reflection_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflect_lights_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_refl_lights<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_refl_lights".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflect_objects_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_refl_objs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_refl_objs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_intensity_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_quality_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_qual<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_qual".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_size_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_shift_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_secondary_component_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_label<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_label".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_glint_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_g_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("g_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_glint_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_g_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("g_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_glint_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_g_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("g_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_transmission_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmit_lights_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_trans_lights<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_trans_lights".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmit_objects_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_trans_objs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_trans_objs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_quality_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_qual<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_qual".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_component_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(32),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_label<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_label".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(33),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opac_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opac_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(34),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opac_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opac_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_fake_caustics_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(35),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_min_shadow_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(36),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_min<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_min".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_max_shadow_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(37),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_max<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_max".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_r_int(mut self, val: f32) -> Self {
        self.params.insert(
            "r_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_size(mut self, val: f32) -> Self {
        self.params.insert(
            "r_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "r_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_int(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_qual(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_qual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_g_int(mut self, val: f32) -> Self {
        self.params.insert(
            "g_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_g_size(mut self, val: f32) -> Self {
        self.params.insert(
            "g_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_g_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "g_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_int(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_qual(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_qual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opac_int(mut self, val: f32) -> Self {
        self.params.insert(
            "opac_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opac_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_min(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_min".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_min".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_max(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_max".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_max".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "trt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_trt_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tt_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opac_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opac_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opac_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_label(mut self, val: &str) -> Self {
        self.params.insert(
            "r_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_label(mut self, val: &str) -> Self {
        self.params.insert(
            "trt_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_label(mut self, val: &str) -> Self {
        self.params.insert(
            "tt_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_conserveenergy(mut self, val: bool) -> Self {
        self.params.insert(
            "conserveenergy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_conserveenergy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conserveenergy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rounding_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "rounding_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rounding_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rounding_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "r_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_r_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_refl_lights(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_refl_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_refl_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_refl_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_refl_objs(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_refl_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_refl_objs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_refl_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_trans_lights(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_trans_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_trans_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_trans_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_trans_objs(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_trans_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_trans_objs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_trans_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fake_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHairmodel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hairmodel"
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

pub trait VopHairmodelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Cf"
    fn out_cf(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Cf".to_string()),
        }
    }
    /// Output pin: "Of"
    fn out_of(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Of".to_string()),
        }
    }
    /// Output pin: "F"
    fn out_f(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("F".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHairmodelOutputs for VopHairmodel {}
impl VopHairmodelOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHairmodel> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHairmodelInnerExt {
    fn cf(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn of(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bsdf_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn computelighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn constant3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn energy_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if_conserve_energy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn isshadow1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opacity_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrhair_r1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrhair_trt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrhair_tt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnet3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnet4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn trt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn trt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHairmodelInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHairmodel> {
    fn cf(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Cf")
    }
    fn of(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Of")
    }
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add2")
    }
    fn and1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and1")
    }
    fn bsdf_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bsdf_inputs")
    }
    fn computelighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("computelighting1")
    }
    fn constant3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("constant3")
    }
    fn energy_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("energy_inputs")
    }
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global1")
    }
    fn if3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if3")
    }
    fn if_conserve_energy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if_conserve_energy")
    }
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("inline1")
    }
    fn isshadow1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("isshadow1")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply2")
    }
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply3")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn opacity_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opacity_inputs")
    }
    fn pbrhair_r1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrhair_r1")
    }
    fn pbrhair_trt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrhair_trt1")
    }
    fn pbrhair_tt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrhair_tt1")
    }
    fn r_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("r_clr")
    }
    fn r_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("r_inputs")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn subinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput3")
    }
    fn subinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput4")
    }
    fn subnet3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subnet3")
    }
    fn subnet4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subnet4")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn trt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("trt_clr")
    }
    fn trt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("trt_inputs")
    }
    fn tt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tt_clr")
    }
    fn tt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tt_inputs")
    }
    fn twoway1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway1")
    }
    fn twoway2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway2")
    }
    fn twoway3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway3")
    }
}

#[derive(Debug, Clone)]
pub struct VopHairnormal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHairnormal {
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

    pub fn set_incident_vector_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_ni<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("nI".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_hair_tangent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_tang<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tang".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_s_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_ss<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ss".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_do_rounding_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_bow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bow".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_bow(mut self, val: bool) -> Self {
        self.params.insert(
            "bow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHairnormal {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hairnormal"
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

pub trait VopHairnormalOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Hair Normal"
    fn out_hairnn(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("hairNn".to_string()),
        }
    }
}

impl VopHairnormalOutputs for VopHairnormal {}
impl VopHairnormalOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHairnormal> {}

#[derive(Debug, Clone)]
pub struct VopHairshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHairshader {
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

    pub fn set_cd_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_cd<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Cd".to_string()),
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
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_uv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_point_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_usepointcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("usePointColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_kd<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Kd".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_root_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rootdiffusecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rootDiffuseColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_userootdiffusemap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useRootDiffuseMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_baserootcolormap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("baseRootColorMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_colorrootmapfilter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("colorRootMapFilter".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_colormapwidth<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("colorMapWidth".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_colormapintensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("colorMapIntensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separate_tip_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipdiffuseseparate<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipDiffuseSeparate".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tip_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipdiffusecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipDiffuseColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_map_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_usetipdiffusemap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useTipDiffuseMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_map_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipcolormap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipColorMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipcolormapfilter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipColorMapFilter".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipcolormapwidth<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipColorMapWidth".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_intensity_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipcolormapintensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipColorMapIntensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_root_to_tip_blend_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuseblendramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseBlendRamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_white_hairs_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_usewhitehairs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useWhiteHairs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_whitehaircolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("whitehaircolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_guard_hairs_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_useguardhairs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useGuardHairs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_intensity_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_spec_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("spec_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_map_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_usespecintensitymap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useSpecIntensityMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_intensity_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_baseintensitymap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("baseIntensityMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_intensity_map_filter_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_intensitymapfilter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("intensityMapFilter".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_intensity_map_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_intensitymapwidth<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("intensityMapWidth".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_root_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rootspeccolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rootSpecColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_map_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_userootspecmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useRootSpecMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_map_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rootspecmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rootSpecMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(32),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rootspecmapfilter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rootSpecMapFilter".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(33),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rootmapwidth<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rootMapWidth".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_intensity_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(34),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_colormapintensity1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("colorMapIntensity1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separate_tip_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(35),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipspecseparate<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipSpecSeparate".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tip_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(36),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipspeccolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipSpecColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_map_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(37),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_usetipspecmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useTipSpecMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_map_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(38),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_basetipspecmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("baseTipSpecMap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(39),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipspecmapfilter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipSpecMapFilter".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(40),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipspecmapwidth<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipSpecMapWidth".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_intensity_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(41),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tipspecmapintensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tipSpecMapIntensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_root_to_tip_blend_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(42),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specblendramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specBlendRamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(43),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_intensity_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(44),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_size_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(45),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_shift_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(46),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_diffuse_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(47),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tinttransmissionwithprimary<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tintTransmissionWithPrimary".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(48),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(49),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opac_color<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opac_color".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_ramp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(50),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opacityramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opacityramp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_white_hair_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(51),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_whitehair<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("whitehair".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_guard_hair_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(52),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_guardhair<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("guardhair".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_53_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(53),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_hairid<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hairid".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(54),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_guardhairtintcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("guardhairtintcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_attribute_alpha_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(55),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_alpha<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Alpha".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_source_values_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(56),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandsaturationramp<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandSaturationRamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_source_values_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(57),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandintensityramp<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandIntensityRamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_seed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(58),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandsaturationseed<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandSaturationSeed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_seed_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(59),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandintensityseed<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandIntensitySeed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_randomize_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(60),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandsaturation<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandSaturation".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_randomize_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(61),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandintensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandIntensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_source_values_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(62),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specrandintensityramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specRandIntensityRamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_seed_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(63),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specrandintensityseed<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specRandIntensitySeed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_randomize_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(64),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specrandintensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specRandIntensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_range_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(65),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandsaturationrange<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandSaturationRange".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_range_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(66),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandintensityrange<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandIntensityRange".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_range_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(67),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specrandintensityrange<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specRandIntensityRange".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_source_values_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(68),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandhueramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandHueRamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_seed_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(69),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandhueseed<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandHueSeed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_randomize_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(70),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandhue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandHue".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_range_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(71),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffuserandhuerange<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseRandHueRange".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_72_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(72),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_id<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("id".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(73),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(74),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_transparent_shadows_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(75),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_min_shadow_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(76),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_min<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_min".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_max_shadow_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(77),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_max<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_max".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_hair_id_seed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(78),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_hairidseed<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hairidseed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_cd_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(79),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_displaycolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("displayColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_80_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(80),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_st<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("st".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_kd(mut self, val: f32) -> Self {
        self.params.insert(
            "Kd".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Kd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_size(mut self, val: f32) -> Self {
        self.params.insert(
            "r_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "r_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colormapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "colorMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colormapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colormapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "colorMapIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colormapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorMapIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipcolormapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "tipColorMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipcolormapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipcolormapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tipColorMapIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipcolormapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMapIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spec_int(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_intensitymapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "intensityMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensitymapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intensityMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootmapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "rootMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rootmapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colormapintensity1(mut self, val: f32) -> Self {
        self.params.insert(
            "colorMapIntensity1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colormapintensity1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorMapIntensity1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipspecmapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "tipSpecMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipspecmapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecMapWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipspecmapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tipSpecMapIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipspecmapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecMapIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_int(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_min(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_min".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_min".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_max(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_max".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_max".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hairidseed(mut self, val: f32) -> Self {
        self.params.insert(
            "hairidseed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hairidseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hairidseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_tex{}_2", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_tex_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}_2", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}_2", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}_2", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}_2", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}_2", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_clamping_mode_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}_2", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}_2", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_envscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_envscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_diff_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_spec_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_spec_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_level(mut self, val: f32) -> Self {
        self.params.insert(
            "level".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_level_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "level".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuselevel(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuselevel".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuselevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuselevel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specularlevel(mut self, val: f32) -> Self {
        self.params.insert(
            "specularlevel".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specularlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specularlevel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_volumelevel(mut self, val: f32) -> Self {
        self.params.insert(
            "volumelevel".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumelevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumelevel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_direct_samples(mut self, val: f32) -> Self {
        self.params.insert(
            "direct_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_direct_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indirect_samples(mut self, val: f32) -> Self {
        self.params.insert(
            "indirect_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_indirect_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nlights(mut self, val: f32) -> Self {
        self.params.insert(
            "nlights".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nlights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nlights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nddispersion(mut self, val: f32) -> Self {
        self.params.insert(
            "nddispersion".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nddispersion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nddispersion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ndpriority(mut self, val: f32) -> Self {
        self.params.insert(
            "ndpriority".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ndpriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ndpriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ndior(mut self, val: f32) -> Self {
        self.params.insert(
            "ndior".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ndior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ndior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayopacity(mut self, val: f32) -> Self {
        self.params.insert(
            "displayOpacity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displayopacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayOpacity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandhuerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diffuseRandHueRange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diffuserandhuerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHueRange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandsaturationrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diffuseRandSaturationRange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diffuserandsaturationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturationRange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandintensityrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diffuseRandIntensityRange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diffuserandintensityrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensityRange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specrandintensityrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "specRandIntensityRange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_specrandintensityrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensityRange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_shinyrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_shinyrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue_range".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat_range".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int_range".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int_range".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_st(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "st".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_st_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "st".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootdiffusecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rootDiffuseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rootdiffusecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootDiffuseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipdiffusecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tipDiffuseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tipdiffusecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipDiffuseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_whitehaircolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "whitehaircolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_whitehaircolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "whitehaircolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guardhairtintcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guardhairtintcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guardhairtintcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guardhairtintcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootspeccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rootSpecColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rootspeccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootSpecColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipspeccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tipSpecColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tipspeccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tt_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opac_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opac_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opac_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_emit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_emit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_envrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_envrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_root_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_diff_root_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_root_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_root_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_tip_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_diff_tip_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_tip_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_tip_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_root_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_spec_root_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_root_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_root_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_tip_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_spec_tip_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_tip_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_tip_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacityexport(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opacityExport".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opacityexport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacityExport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_direct(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indirect(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ce(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ce".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ce".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_direct_emission(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_emission".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_emission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_all_emission(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "all_emission".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_all_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "all_emission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_all(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "all".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_all_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "all".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indirect_emission(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_emission".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_emission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_direct_noshadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_noshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_noshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_noshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_direct_shadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_shadow".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_shadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indirect_noshadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_noshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_noshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_noshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indirect_shadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_shadow".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_shadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_absorption(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "absorption".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absorption_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "absorption".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "uv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandhueseed(mut self, val: i32) -> Self {
        self.params.insert(
            "diffuseRandHueSeed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_diffuserandhueseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHueSeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandsaturationseed(mut self, val: i32) -> Self {
        self.params.insert(
            "diffuseRandSaturationSeed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_diffuserandsaturationseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturationSeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandintensityseed(mut self, val: i32) -> Self {
        self.params.insert(
            "diffuseRandIntensitySeed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_diffuserandintensityseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensitySeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specrandintensityseed(mut self, val: i32) -> Self {
        self.params.insert(
            "specRandIntensitySeed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_specrandintensityseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensitySeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_speclayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_speclayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_opacitylayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_opacitylayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumplayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_bumplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_normallayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_normallayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_whitehair(mut self, val: i32) -> Self {
        self.params.insert(
            "whitehair".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_whitehair_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "whitehair".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guardhair(mut self, val: i32) -> Self {
        self.params.insert(
            "guardhair".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guardhair_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guardhair".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hairid(mut self, val: i32) -> Self {
        self.params.insert(
            "hairid".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hairid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hairid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_id(mut self, val: i32) -> Self {
        self.params.insert(
            "id".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_id_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "id".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuseblendramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "diffuseBlendRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuseblendramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseBlendRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specblendramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "specBlendRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_specblendramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specBlendRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandhueramp(
        mut self,
        val: Vec<houdini_ramen_core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "diffuseRandHueRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuserandhueramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHueRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandsaturationramp(
        mut self,
        val: Vec<houdini_ramen_core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "diffuseRandSaturationRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuserandsaturationramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturationRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandintensityramp(
        mut self,
        val: Vec<houdini_ramen_core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "diffuseRandIntensityRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuserandintensityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensityRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specrandintensityramp(
        mut self,
        val: Vec<houdini_ramen_core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "specRandIntensityRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_specrandintensityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensityRamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacityramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "opacityramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_opacityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacityramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_baserootcolormap(mut self, val: &str) -> Self {
        self.params.insert(
            "baseRootColorMap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_baserootcolormap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseRootColorMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colorrootmapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "colorRootMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colorrootmapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorRootMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipcolormap(mut self, val: &str) -> Self {
        self.params.insert(
            "tipColorMap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipcolormap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipcolormapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "tipColorMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipcolormapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_baseintensitymap(mut self, val: &str) -> Self {
        self.params.insert(
            "baseIntensityMap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_baseintensitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseIntensityMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_intensitymapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "intensityMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_intensitymapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intensityMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootspecmap(mut self, val: &str) -> Self {
        self.params.insert(
            "rootSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootspecmapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "rootSpecMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootspecmapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootSpecMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basetipspecmap(mut self, val: &str) -> Self {
        self.params.insert(
            "baseTipSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basetipspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseTipSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipspecmapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "tipSpecMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipspecmapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecMapFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_specmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_opacitymap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_opacitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumpmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumpmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumptype(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumpbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumpbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_normalmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_normalmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_normalbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_normalbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_envrotorder(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_envrotorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_map(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_map".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_map".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_blend_ramp(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_blend_ramp".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_blend_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_blend_ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_map(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_map".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_map".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_blend_ramp(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_blend_ramp".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_blend_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_blend_ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_alpha_blend_ramp(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_alpha_blend_ramp".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_alpha_blend_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_alpha_blend_ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepointcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "usePointColor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepointcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usePointColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_userootdiffusemap(mut self, val: bool) -> Self {
        self.params.insert(
            "useRootDiffuseMap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userootdiffusemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useRootDiffuseMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipdiffuseseparate(mut self, val: bool) -> Self {
        self.params.insert(
            "tipDiffuseSeparate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tipdiffuseseparate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipDiffuseSeparate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetipdiffusemap(mut self, val: bool) -> Self {
        self.params.insert(
            "useTipDiffuseMap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetipdiffusemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useTipDiffuseMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usewhitehairs(mut self, val: bool) -> Self {
        self.params.insert(
            "useWhiteHairs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usewhitehairs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useWhiteHairs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useguardhairs(mut self, val: bool) -> Self {
        self.params.insert(
            "useGuardHairs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useguardhairs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useGuardHairs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usespecintensitymap(mut self, val: bool) -> Self {
        self.params.insert(
            "useSpecIntensityMap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usespecintensitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useSpecIntensityMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_userootspecmap(mut self, val: bool) -> Self {
        self.params.insert(
            "useRootSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userootspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useRootSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipspecseparate(mut self, val: bool) -> Self {
        self.params.insert(
            "tipSpecSeparate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tipspecseparate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecSeparate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetipspecmap(mut self, val: bool) -> Self {
        self.params.insert(
            "useTipSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetipspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useTipSpecMap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tinttransmissionwithprimary(mut self, val: bool) -> Self {
        self.params.insert(
            "tintTransmissionWithPrimary".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tinttransmissionwithprimary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tintTransmissionWithPrimary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandhue(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuseRandHue".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuserandhue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandsaturation(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuseRandSaturation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuserandsaturation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuserandintensity(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuseRandIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuserandintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specrandintensity(mut self, val: bool) -> Self {
        self.params.insert(
            "specRandIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_specrandintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fake_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_light(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_light_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_cutout(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_cutout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_adjustshiny(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_adjustshiny_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_bumpinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_bumpinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_use_diff_map(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_use_diff_map".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_use_diff_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_use_diff_map".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_use_spec_map(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_use_spec_map".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_use_spec_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_use_spec_map".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHairshader {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hairshader"
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

pub trait VopHairshaderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "surface"
    fn out_surface(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("surface".to_string()),
        }
    }
    /// Output pin: "Opacity Export"
    fn out_opacityexport(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("opacityExport".to_string()),
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHairshaderOutputs for VopHairshader {}
impl VopHairshaderOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHairshader> {}

#[derive(Debug, Clone)]
pub struct VopHairshadercore {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHairshadercore {
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

    pub fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_direction_from_eye_to_surface_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
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
    pub fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("I".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tip_direction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tip<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tip".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_conserve_energy_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_conserveenergy<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("conserveenergy".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_fake_rounding_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rounding_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rounding_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_primary_reflection_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_primary_component_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_r_label<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("r_label".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_secondary_reflection_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflect_lights_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_refl_lights<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_refl_lights".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflect_objects_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_refl_objs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_refl_objs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_intensity_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_quality_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_qual<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_qual".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_size_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_shift_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_secondary_component_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trt_label<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trt_label".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_glint_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_g_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("g_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_glint_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_g_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("g_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_glint_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_g_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("g_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_transmission_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmit_lights_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_trans_lights<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_trans_lights".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmit_objects_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_trans_objs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_trans_objs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_quality_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_qual<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_qual".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_component_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(32),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tt_label<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tt_label".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(33),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opac_int<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opac_int".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(34),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opac_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opac_clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_fake_caustics_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(35),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_min_shadow_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(36),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_min<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_min".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_max_shadow_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(37),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fake_max<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fake_max".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_r_int(mut self, val: f32) -> Self {
        self.params.insert(
            "r_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_size(mut self, val: f32) -> Self {
        self.params.insert(
            "r_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "r_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_int(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_qual(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_qual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_g_int(mut self, val: f32) -> Self {
        self.params.insert(
            "g_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_g_size(mut self, val: f32) -> Self {
        self.params.insert(
            "g_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_g_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "g_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_int(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_qual(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_qual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_qual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opac_int(mut self, val: f32) -> Self {
        self.params.insert(
            "opac_int".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opac_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_min(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_min".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_min".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_max(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_max".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_max".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "trt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_trt_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tt_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opac_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opac_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opac_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_label(mut self, val: &str) -> Self {
        self.params.insert(
            "r_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_label(mut self, val: &str) -> Self {
        self.params.insert(
            "trt_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_label(mut self, val: &str) -> Self {
        self.params.insert(
            "tt_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_conserveenergy(mut self, val: bool) -> Self {
        self.params.insert(
            "conserveenergy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_conserveenergy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conserveenergy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rounding_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "rounding_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rounding_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rounding_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "r_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_r_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_refl_lights(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_refl_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_refl_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_refl_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trt_refl_objs(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_refl_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_refl_objs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_refl_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_trans_lights(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_trans_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_trans_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_trans_lights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tt_trans_objs(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_trans_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_trans_objs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_trans_objs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fake_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fake_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHairshadercore {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hairshadercore"
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

pub trait VopHairshadercoreOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "layer"
    fn out_layer(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("layer".to_string()),
        }
    }
    /// Output pin: "F"
    fn out_f(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("F".to_string()),
        }
    }
    /// Output pin: "Of"
    fn out_of(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Of".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHairshadercoreOutputs for VopHairshadercore {}
impl VopHairshadercoreOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHairshadercore>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHairshadercoreInnerExt {
    fn of(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bsdf_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn constant3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn energy_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if_conserve_energy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn isshadow1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn layerpack1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opacity_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrhair_r1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrhair_trt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrhair_tt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnet3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnet4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn trt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn trt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHairshadercoreInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopHairshadercore>
{
    fn of(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Of")
    }
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add2")
    }
    fn and1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and1")
    }
    fn bsdf_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bsdf_inputs")
    }
    fn constant3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("constant3")
    }
    fn energy_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("energy_inputs")
    }
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global1")
    }
    fn if3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if3")
    }
    fn if_conserve_energy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if_conserve_energy")
    }
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("inline1")
    }
    fn isshadow1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("isshadow1")
    }
    fn layerpack1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("layerpack1")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply2")
    }
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply3")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn opacity_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opacity_inputs")
    }
    fn pbrhair_r1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrhair_r1")
    }
    fn pbrhair_trt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrhair_trt1")
    }
    fn pbrhair_tt1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrhair_tt1")
    }
    fn r_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("r_clr")
    }
    fn r_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("r_inputs")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn subinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput3")
    }
    fn subinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput4")
    }
    fn subnet3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subnet3")
    }
    fn subnet4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subnet4")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn trt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("trt_clr")
    }
    fn trt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("trt_inputs")
    }
    fn tt_clr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tt_clr")
    }
    fn tt_inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tt_inputs")
    }
    fn twoway1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway1")
    }
    fn twoway2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway2")
    }
    fn twoway3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway3")
    }
}

#[derive(Debug, Clone)]
pub struct VopHairspec {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHairspec {
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

    pub fn set_hair_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_nn<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("nN".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_incident_vector_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_ni<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("nI".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tangent_vector_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_nt<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("nT".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_intensity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ks<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Ks".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_spec<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("spec".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_ks(mut self, val: f32) -> Self {
        self.params.insert(
            "Ks".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHairspec {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hairspec"
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

pub trait VopHairspecOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
    /// Output pin: "Illumination"
    fn out_illum(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("illum".to_string()),
        }
    }
    /// Output pin: "F"
    fn out_f(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("f".to_string()),
        }
    }
}

impl VopHairspecOutputs for VopHairspec {}
impl VopHairspecOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHairspec> {}

#[derive(Debug, Clone)]
pub struct VopHalo {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHalo {
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

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_position_of_eye_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_eye<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Eye".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_cf<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Cf".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_of<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Of".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_alpha_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_af<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Af".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fog_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fog_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fdensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fdensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fog_point_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fptsize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fptsize".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_fdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "fdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fptsize(mut self, val: f32) -> Self {
        self.params.insert(
            "fptsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fptsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fptsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHalo {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "halo"
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

pub trait VopHaloOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
    /// Output pin: "Output Alpha"
    fn out_alpha(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("alpha".to_string()),
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHaloOutputs for VopHalo {}
impl VopHaloOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHalo> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHaloInnerExt {
    fn af_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn af_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cf_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cf_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn eye_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn eye_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn of_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn of_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn apply_density(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn calculate_point_size(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_radius(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement_for_alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dist_eye_to_surface(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fog_times_of(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mix_fog_with_cf(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn spacechg1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn spacechg2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHaloInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHalo> {
    fn af_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Af_global")
    }
    fn af_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Af_input")
    }
    fn cf_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Cf_global")
    }
    fn cf_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Cf_input")
    }
    fn eye_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Eye_global")
    }
    fn eye_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Eye_input")
    }
    fn of_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Of_global")
    }
    fn of_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Of_input")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn apply_density(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("apply_density")
    }
    fn calculate_point_size(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("calculate_point_size")
    }
    fn clamp_radius(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_radius")
    }
    fn complement_for_alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement_for_alpha")
    }
    fn compute_alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_alpha")
    }
    fn dist_eye_to_surface(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dist_eye_to_surface")
    }
    fn fog_times_of(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fog_times_Of")
    }
    fn mix_fog_with_cf(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mix_fog_with_Cf")
    }
    fn spacechg1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("spacechg1")
    }
    fn spacechg2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("spacechg2")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopHasinput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHasinput {
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

    pub fn set_input_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_inputidx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("inputidx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_inputidx(mut self, val: i32) -> Self {
        self.params.insert(
            "inputidx".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_inputidx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputidx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHasinput {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hasinput"
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

pub trait VopHasinputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Input Does Or Does Not Exist"
    fn out_exists(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("exists".to_string()),
        }
    }
}

impl VopHasinputOutputs for VopHasinput {}
impl VopHasinputOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHasinput> {}

#[derive(Debug, Clone)]
pub struct VopHaskey {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHaskey {
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

    pub fn set_dictionary_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_dict<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dict".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_key<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_signature(mut self, val: f32) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key(mut self, val: &str) -> Self {
        self.params.insert(
            "key".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHaskey {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "haskey"
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

pub trait VopHaskeyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Exists"
    fn out_exists(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("exists".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHaskeyOutputs for VopHaskey {}
impl VopHaskeyOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHaskey> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHaskeyInnerExt {
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn snippet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHaskeyInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHaskey> {
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("const1")
    }
    fn snippet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("snippet1")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopHasmetadata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHasmetadata {
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

    pub fn set_input_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_input_index<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input_index".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_metadata_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_meta_name<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("meta_name".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_input_index(mut self, val: i32) -> Self {
        self.params.insert(
            "input_index".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input_index".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_meta_name(mut self, val: &str) -> Self {
        self.params.insert(
            "meta_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_meta_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "meta_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHasmetadata {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hasmetadata"
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

pub trait VopHasmetadataOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Metadata Exists"
    fn out_meta_exists(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("meta_exists".to_string()),
        }
    }
}

impl VopHasmetadataOutputs for VopHasmetadata {}
impl VopHasmetadataOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHasmetadata> {}

#[derive(Debug, Clone)]
pub struct VopHextile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHextile {
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

    pub fn set_uv_0_0_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_uv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_size_1_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_0_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_offset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopHextile {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hextile"
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

pub trait VopHextileOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Random ID"
    fn out_id(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("id".to_string()),
        }
    }
    /// Output pin: "Computed UV Coordinates"
    fn out_hexuv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("hexuv".to_string()),
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHextileOutputs for VopHextile {}
impl VopHextileOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHextile> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHextileInnerExt {
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHextileInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHextile> {
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected1")
    }
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("inline1")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn subinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput2")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvcoords1")
    }
}

#[derive(Debug, Clone)]
pub struct VopHilownoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHilownoise {
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

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_amp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("amp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_chip_amount_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_chip<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("chip".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_chip(mut self, val: f32) -> Self {
        self.params.insert(
            "chip".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_chip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHilownoise {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hilownoise"
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

pub trait VopHilownoiseOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Positional Frequency"
    fn out_freqp(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("freqP".to_string()),
        }
    }
    /// Output pin: "Noise Amount"
    fn out_amount(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("amount".to_string()),
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHilownoiseOutputs for VopHilownoise {}
impl VopHilownoiseOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHilownoise> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHilownoiseInnerExt {
    fn bumpscale_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn correction_factor(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn denting_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn highfreq_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn low_freq_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_noises(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn chip_multiply(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_bump(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_high_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_low_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn maddconst1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply_bumpscale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn noise_squared(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHilownoiseInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHilownoise> {
    fn bumpscale_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("BumpScale_Noise")
    }
    fn correction_factor(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Correction_Factor")
    }
    fn denting_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Denting_Noise")
    }
    fn highfreq_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("HighFreq_Holes")
    }
    fn low_freq_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Low_Freq_Holes")
    }
    fn p_freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_freq")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn add_noises(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_noises")
    }
    fn chip_multiply(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("chip_multiply")
    }
    fn clamp_bump(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_bump")
    }
    fn clamp_high_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_high_holes")
    }
    fn clamp_low_holes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_low_holes")
    }
    fn maddconst1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("maddconst1")
    }
    fn multiply_bumpscale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply_bumpscale")
    }
    fn noise_squared(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("noise_squared")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmatxgetcomponRow {
    Row1 = 0,
    Row2 = 1,
    Row3 = 2,
    Row4 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmatxgetcomponCol {
    Column1 = 0,
    Column2 = 1,
    Column3 = 2,
    Column4 = 3,
}

#[derive(Debug, Clone)]
pub struct VopHmatxgetcompon {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmatxgetcompon {
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

    pub fn set_input_matrix_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hmatx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hmatx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hmatx(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "hmatx".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_hmatx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hmatx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hmatx_um(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "hmatx_um".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_hmatx_um_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hmatx_um".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_row(mut self, val: VopHmatxgetcomponRow) -> Self {
        self.params.insert(
            "row".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_row_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "row".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_col(mut self, val: VopHmatxgetcomponCol) -> Self {
        self.params.insert(
            "col".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_col_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "col".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmatxgetcompon {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmatxgetcompon"
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

pub trait VopHmatxgetcomponOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Component"
    fn out_fval(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval".to_string()),
        }
    }
}

impl VopHmatxgetcomponOutputs for VopHmatxgetcompon {}
impl VopHmatxgetcomponOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmatxgetcompon>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmatxsetcomponRow {
    Row1 = 0,
    Row2 = 1,
    Row3 = 2,
    Row4 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmatxsetcomponCol {
    Column1 = 0,
    Column2 = 1,
    Column3 = 2,
    Column4 = 3,
}

#[derive(Debug, Clone)]
pub struct VopHmatxsetcompon {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmatxsetcompon {
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

    pub fn set_input_matrix_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hmatx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hmatx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_fval<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fval".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hmatx(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "hmatx".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_hmatx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hmatx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_row(mut self, val: VopHmatxsetcomponRow) -> Self {
        self.params.insert(
            "row".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_row_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "row".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_col(mut self, val: VopHmatxsetcomponCol) -> Self {
        self.params.insert(
            "col".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_col_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "col".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmatxsetcompon {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmatxsetcompon"
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

pub trait VopHmatxsetcomponOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Matrix"
    fn out_newhmatx(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("newhmatx".to_string()),
        }
    }
}

impl VopHmatxsetcomponOutputs for VopHmatxsetcompon {}
impl VopHmatxsetcomponOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmatxsetcompon>
{
}

#[derive(Debug, Clone)]
pub struct VopHmatxtofloat {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmatxtofloat {
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

    pub fn set_input_matrix_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hmatx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hmatx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hmatx(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "hmatx".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_hmatx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hmatx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmatxtofloat {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmatxtofloat"
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

pub trait VopHmatxtofloatOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Row 1 Column 1"
    fn out_fval11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval11".to_string()),
        }
    }
    /// Output pin: "Row 1 Column 2"
    fn out_fval12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval12".to_string()),
        }
    }
    /// Output pin: "Row 1 Column 3"
    fn out_fval13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval13".to_string()),
        }
    }
    /// Output pin: "Row 1 Column 4"
    fn out_fval14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval14".to_string()),
        }
    }
    /// Output pin: "Row 2 Column 1"
    fn out_fval21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval21".to_string()),
        }
    }
    /// Output pin: "Row 2 Column 2"
    fn out_fval22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval22".to_string()),
        }
    }
    /// Output pin: "Row 2 Column 3"
    fn out_fval23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval23".to_string()),
        }
    }
    /// Output pin: "Row 2 Column 4"
    fn out_fval24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval24".to_string()),
        }
    }
    /// Output pin: "Row 3 Column 1"
    fn out_fval31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval31".to_string()),
        }
    }
    /// Output pin: "Row 3 Column 2"
    fn out_fval32(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval32".to_string()),
        }
    }
    /// Output pin: "Row 3 Column 3"
    fn out_fval33(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval33".to_string()),
        }
    }
    /// Output pin: "Row 3 Column 4"
    fn out_fval34(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval34".to_string()),
        }
    }
    /// Output pin: "Row 4 Column 1"
    fn out_fval41(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval41".to_string()),
        }
    }
    /// Output pin: "Row 4 Column 2"
    fn out_fval42(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval42".to_string()),
        }
    }
    /// Output pin: "Row 4 Column 3"
    fn out_fval43(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval43".to_string()),
        }
    }
    /// Output pin: "Row 4 Column 4"
    fn out_fval44(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval44".to_string()),
        }
    }
}

impl VopHmatxtofloatOutputs for VopHmatxtofloat {}
impl VopHmatxtofloatOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmatxtofloat> {}

#[derive(Debug, Clone)]
pub struct VopHmatxtohvec {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmatxtohvec {
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

    pub fn set_input_matrix4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_matx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("matx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmatxtohvec {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmatxtohvec"
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

pub trait VopHmatxtohvecOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Row 1"
    fn out_row1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("row1".to_string()),
        }
    }
    /// Output pin: "Row 2"
    fn out_row2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("row2".to_string()),
        }
    }
    /// Output pin: "Row 3"
    fn out_row3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("row3".to_string()),
        }
    }
    /// Output pin: "Row 4"
    fn out_row4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("row4".to_string()),
        }
    }
}

impl VopHmatxtohvecOutputs for VopHmatxtohvec {}
impl VopHmatxtohvecOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmatxtohvec> {}

#[derive(Debug, Clone)]
pub struct VopHmtlxbias {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxbias {
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

    pub fn set_in_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_in<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_bias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_in(mut self, val: f32) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxbias {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxbias"
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

pub trait VopHmtlxbiasOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxbiasOutputs for VopHmtlxbias {}
impl VopHmtlxbiasOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxbias> {}

#[derive(Debug, Clone)]
pub struct VopHmtlxcheckers2d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcheckers2d {
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

    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_texcoord<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("texcoord".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_freq(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_texcoord(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_texcoord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcheckers2d {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcheckers2d"
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

pub trait VopHmtlxcheckers2dOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxcheckers2dOutputs for VopHmtlxcheckers2d {}
impl VopHmtlxcheckers2dOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcheckers2d>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxcircles2d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcircles2d {
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

    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_circle_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_inner_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_inner_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("inner_scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_mid_point_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_mid_point<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("mid_point".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_row_shift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_row_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("row_shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_randomize_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rand_value<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rand_value".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_randomize_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rand_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rand_scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_seed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_seed<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("seed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_texcoord<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("texcoord".to_string()),
            (out.node_id, out.pin),
        );
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
    pub fn with_inner_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "inner_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inner_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inner_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_row_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "row_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_row_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "row_shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rand_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "rand_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rand_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rand_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "seed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_freq(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mid_point(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "mid_point".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_mid_point_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mid_point".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_texcoord(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_texcoord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rand_value(mut self, val: bool) -> Self {
        self.params.insert(
            "rand_value".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rand_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rand_value".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcircles2d {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcircles2d"
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

pub trait VopHmtlxcircles2dOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxcircles2dOutputs for VopHmtlxcircles2d {}
impl VopHmtlxcircles2dOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcircles2d>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxcolorcorrect {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcolorcorrect {
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

    pub fn set_input_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_in<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_hue_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hue".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_saturation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_saturation<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("saturation".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gamma_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_gamma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gamma".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_lift_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_lift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("lift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_gain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_contrast_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_contrast<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("contrast".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_contrast_pivot_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_contrastpivot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("contrastpivot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_exposure_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_exposure<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("exposure".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_saturation(mut self, val: f32) -> Self {
        self.params.insert(
            "saturation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_saturation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "saturation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lift(mut self, val: f32) -> Self {
        self.params.insert(
            "lift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lift".to_string(),
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
    pub fn with_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "contrast".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrast".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contrastpivot(mut self, val: f32) -> Self {
        self.params.insert(
            "contrastpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrastpivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrastpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "exposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_in(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcolorcorrect {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcolorcorrect"
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

pub trait VopHmtlxcolorcorrectOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
    /// Output pin: "outr"
    fn out_outr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("outr".to_string()),
        }
    }
    /// Output pin: "outg"
    fn out_outg(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("outg".to_string()),
        }
    }
    /// Output pin: "outb"
    fn out_outb(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("outb".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHmtlxcolorcorrectOutputs for VopHmtlxcolorcorrect {}
impl VopHmtlxcolorcorrectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcolorcorrect>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHmtlxcolorcorrectInnerExt {
    fn contrast(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn exposure(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn exposurepwr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn gain(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn gamma(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn hsvadjust(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn liftadd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn liftmult(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn liftsubtract(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3c1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm2hue(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn saturation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHmtlxcolorcorrectInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopHmtlxcolorcorrect>
{
    fn contrast(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("contrast")
    }
    fn exposure(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("exposure")
    }
    fn exposurepwr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("exposurepwr")
    }
    fn gain(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("gain")
    }
    fn gamma(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("gamma")
    }
    fn hsvadjust(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("hsvadjust")
    }
    fn liftadd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("liftadd")
    }
    fn liftmult(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("liftmult")
    }
    fn liftsubtract(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("liftsubtract")
    }
    fn mtlxseparate3c1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mtlxseparate3c1")
    }
    fn parm2hue(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm2hue")
    }
    fn saturation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("saturation")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopHmtlxcomplement {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcomplement {
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

    pub fn set_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_value<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_value(mut self, val: f32) -> Self {
        self.params.insert(
            "value".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shader_rendercontextname(mut self, val: &str) -> Self {
        self.params.insert(
            "shader_rendercontextname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shader_rendercontextname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shader_rendercontextname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcomplement {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcomplement"
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

pub trait VopHmtlxcomplementOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "complement"
    fn out_complement(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("complement".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHmtlxcomplementOutputs for VopHmtlxcomplement {}
impl VopHmtlxcomplementOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcomplement>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHmtlxcomplementInnerExt {
    fn mtlxadd1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxmultiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHmtlxcomplementInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopHmtlxcomplement>
{
    fn mtlxadd1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mtlxadd1")
    }
    fn mtlxmultiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mtlxmultiply1")
    }
    fn subnetinput(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subnetinput")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopHmtlxcomplementcolor {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcomplementcolor {
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

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_in<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_in(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcomplementcolor {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcomplementcolor"
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

pub trait VopHmtlxcomplementcolorOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxcomplementcolorOutputs for VopHmtlxcomplementcolor {}
impl VopHmtlxcomplementcolorOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcomplementcolor>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxcubicrampc {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcubicrampc {
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

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_input(mut self, val: f32) -> Self {
        self.params.insert(
            "input".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcubicrampc {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcubicrampc"
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

pub trait VopHmtlxcubicrampcOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHmtlxcubicrampcOutputs for VopHmtlxcubicrampc {}
impl VopHmtlxcubicrampcOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcubicrampc>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHmtlxcubicrampcInnerExt {
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHmtlxcubicrampcInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopHmtlxcubicrampc>
{
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invlinear")
    }
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numkeys")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uniformramp")
    }
}

#[derive(Debug, Clone)]
pub struct VopHmtlxcubicrampf {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxcubicrampf {
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

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_input(mut self, val: f32) -> Self {
        self.params.insert(
            "input".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxcubicrampf {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxcubicrampf"
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

pub trait VopHmtlxcubicrampfOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHmtlxcubicrampfOutputs for VopHmtlxcubicrampf {}
impl VopHmtlxcubicrampfOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxcubicrampf>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHmtlxcubicrampfInnerExt {
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHmtlxcubicrampfInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopHmtlxcubicrampf>
{
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invlinear")
    }
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numkeys")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uniformramp")
    }
}

#[derive(Debug, Clone)]
pub struct VopHmtlxfacingratio {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxfacingratio {
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
    pub fn set_input_name_in1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_in2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_bias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_gain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gamma_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_gamma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gamma".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_invert_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_invert<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("invert".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_face_forward_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_forward<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("forward".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_in1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "in1".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_in1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_in2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "in2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_in2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in2".to_string(),
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
    pub fn with_forward(mut self, val: bool) -> Self {
        self.params.insert(
            "forward".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forward_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forward".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxfacingratio {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxfacingratio"
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

pub trait VopHmtlxfacingratioOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxfacingratioOutputs for VopHmtlxfacingratio {}
impl VopHmtlxfacingratioOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxfacingratio>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxgain {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxgain {
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

    pub fn set_in_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_in<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_gain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_in(mut self, val: f32) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxgain {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxgain"
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

pub trait VopHmtlxgainOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxgainOutputs for VopHmtlxgain {}
impl VopHmtlxgainOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxgain> {}

#[derive(Debug, Clone)]
pub struct VopHmtlxhcatmullrom {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxhcatmullrom {
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

    pub fn set_t_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_t<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("t".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tension_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_tension<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tension".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_key0<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key0".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_t(mut self, val: f32) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
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
    pub fn with_tension(mut self, val: f32) -> Self {
        self.params.insert(
            "tension".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tension".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key0(mut self, val: f32) -> Self {
        self.params.insert(
            "key0".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key1(mut self, val: f32) -> Self {
        self.params.insert(
            "key1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key2(mut self, val: f32) -> Self {
        self.params.insert(
            "key2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key3(mut self, val: f32) -> Self {
        self.params.insert(
            "key3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key0_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key0_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key0_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key0_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key1_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key1_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key1_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key1_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key2_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key2_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key2_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key2_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key3_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key3_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key3_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key3_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxhcatmullrom {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxhcatmullrom"
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

pub trait VopHmtlxhcatmullromOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxhcatmullromOutputs for VopHmtlxhcatmullrom {}
impl VopHmtlxhcatmullromOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxhcatmullrom>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxhinvlinear {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxhinvlinear {
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

    pub fn set_ramp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_ramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ramp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_num_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_num<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("num".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_key0<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key0".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key4<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key4".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key5<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key5".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key6_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key6<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key6".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key7_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key7<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key7".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key8_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key8<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key8".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_key9_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_key9<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key9".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_ramp(mut self, val: f32) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_num(mut self, val: f32) -> Self {
        self.params.insert(
            "num".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_num_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "num".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key0(mut self, val: f32) -> Self {
        self.params.insert(
            "key0".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key1(mut self, val: f32) -> Self {
        self.params.insert(
            "key1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key2(mut self, val: f32) -> Self {
        self.params.insert(
            "key2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key3(mut self, val: f32) -> Self {
        self.params.insert(
            "key3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key4(mut self, val: f32) -> Self {
        self.params.insert(
            "key4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key5(mut self, val: f32) -> Self {
        self.params.insert(
            "key5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key6(mut self, val: f32) -> Self {
        self.params.insert(
            "key6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key7(mut self, val: f32) -> Self {
        self.params.insert(
            "key7".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key8(mut self, val: f32) -> Self {
        self.params.insert(
            "key8".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_key9(mut self, val: f32) -> Self {
        self.params.insert(
            "key9".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key9".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxhinvlinear {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxhinvlinear"
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

pub trait VopHmtlxhinvlinearOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxhinvlinearOutputs for VopHmtlxhinvlinear {}
impl VopHmtlxhinvlinearOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxhinvlinear>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxhuniformcubic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxhuniformcubic {
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

    pub fn set_ramp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_ramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ramp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_num_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_num<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("num".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_value0<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value0".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value4<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value4".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value5<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value5".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_6_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value6<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value6".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_7_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value7<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value7".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_8_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value8<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value8".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value_9_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value9<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value9".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_ramp(mut self, val: f32) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_num(mut self, val: f32) -> Self {
        self.params.insert(
            "num".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_num_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "num".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value0(mut self, val: f32) -> Self {
        self.params.insert(
            "value0".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value1(mut self, val: f32) -> Self {
        self.params.insert(
            "value1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value2(mut self, val: f32) -> Self {
        self.params.insert(
            "value2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value3(mut self, val: f32) -> Self {
        self.params.insert(
            "value3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value4(mut self, val: f32) -> Self {
        self.params.insert(
            "value4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value5(mut self, val: f32) -> Self {
        self.params.insert(
            "value5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value6(mut self, val: f32) -> Self {
        self.params.insert(
            "value6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value7(mut self, val: f32) -> Self {
        self.params.insert(
            "value7".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value8(mut self, val: f32) -> Self {
        self.params.insert(
            "value8".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value9(mut self, val: f32) -> Self {
        self.params.insert(
            "value9".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value9".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value0_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value0_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value0_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value0_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value1_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value1_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value1_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value1_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value2_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value2_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value2_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value2_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value3_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value3_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value3_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value3_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value4_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value4_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value4_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value4_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value5_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value5_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value5_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value5_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value6_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value6_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value6_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value6_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value7_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value7_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value7_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value7_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value8_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value8_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value8_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value8_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value9_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value9_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value9_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value9_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxhuniformcubic {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxhuniformcubic"
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

pub trait VopHmtlxhuniformcubicOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxhuniformcubicOutputs for VopHmtlxhuniformcubic {}
impl VopHmtlxhuniformcubicOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxhuniformcubic>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxhuniformramp {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxhuniformramp {
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

    pub fn set_ramp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_ramp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ramp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_num_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_num<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("num".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_value0<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value0".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value4<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value4".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value5<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value5".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value6_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value6<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value6".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value7_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value7<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value7".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value8_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value8<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value8".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_value9_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_value9<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("value9".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_ramp(mut self, val: f32) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_num(mut self, val: f32) -> Self {
        self.params.insert(
            "num".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_num_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "num".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value0(mut self, val: f32) -> Self {
        self.params.insert(
            "value0".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value1(mut self, val: f32) -> Self {
        self.params.insert(
            "value1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value2(mut self, val: f32) -> Self {
        self.params.insert(
            "value2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value3(mut self, val: f32) -> Self {
        self.params.insert(
            "value3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value4(mut self, val: f32) -> Self {
        self.params.insert(
            "value4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value5(mut self, val: f32) -> Self {
        self.params.insert(
            "value5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value6(mut self, val: f32) -> Self {
        self.params.insert(
            "value6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value7(mut self, val: f32) -> Self {
        self.params.insert(
            "value7".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value8(mut self, val: f32) -> Self {
        self.params.insert(
            "value8".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value9(mut self, val: f32) -> Self {
        self.params.insert(
            "value9".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value9".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value0_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value0_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value0_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value0_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value1_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value1_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value1_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value1_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value2_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value2_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value2_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value2_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value3_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value3_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value3_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value3_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value4_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value4_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value4_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value4_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value5_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value5_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value5_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value5_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value6_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value6_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value6_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value6_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value7_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value7_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value7_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value7_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value8_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value8_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value8_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value8_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_value9_color3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value9_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value9_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value9_color3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxhuniformramp {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxhuniformramp"
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

pub trait VopHmtlxhuniformrampOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
}

impl VopHmtlxhuniformrampOutputs for VopHmtlxhuniformramp {}
impl VopHmtlxhuniformrampOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxhuniformramp>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmtlxpbrtexturesetProjection {
    Uv = 0,
    /// Tri-Planar
    TriMinusPlanar = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmtlxpbrtexturesetTriplanarUpaxis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmtlxpbrtexturesetDefaultValuesMode {
    Fallback = 0,
    Multiply = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHmtlxpbrtexturesetBumpStyle {
    Height = 0,
    Normal = 1,
}

#[derive(Debug, Clone)]
pub struct VopHmtlxpbrtextureset {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxpbrtextureset {
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

    pub fn set_image_filter_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_filtertype<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("filtertype".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_base_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_base_color_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("base_color_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_metalness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_metalness_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("metalness_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specular_color_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specular_color_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specular_roughness_file<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specular_roughness_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_coat_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_coat_color_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("coat_color_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_coat_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_coat_roughness_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("coat_roughness_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_subsurface_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_subsurface_color_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("subsurface_color_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_subsurface_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_subsurface_scale_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("subsurface_scale_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_emission_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_emission_color_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("emission_color_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_occlusion_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_occlusion_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("occlusion_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opacity_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opacity_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bump_style_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_bump_style<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bump_style".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bump_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_bump_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bump_scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bump_normal_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_bump_normal_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bump_normal_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bump_height_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_bump_height_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bump_height_file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_projection_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_projection<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("projection".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_texcoord<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("texcoord".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tri_planar_blend_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_triplanar_blend<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("triplanar_blend".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tri_planar_up_axis_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_triplanar_upaxis<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("triplanar_upaxis".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tri_planar_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_position_triplanar<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("position_triplanar".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tri_planar_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_normal_triplanar<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("normal_triplanar".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_default_values_mode_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_default_values_mode<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("default_values_mode".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_base_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_base_color_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("base_color_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_metalness_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_metalness_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("metalness_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specular_color_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specular_color_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_roughness_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specular_roughness_default<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specular_roughness_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_coat_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_coat_color_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("coat_color_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_coat_roughness_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_coat_roughness_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("coat_roughness_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_subsurface_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_subsurface_color_default<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("subsurface_color_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_subsurface_scale_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_subsurface_scale_default<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("subsurface_scale_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_emission_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_emission_color_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("emission_color_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_occlusion_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(32),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_occlusion_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("occlusion_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(33),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opacity_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opacity_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(34),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_normal_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("normal_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tangent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(35),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tangent_default<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tangent_default".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_triplanar_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "triplanar_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_triplanar_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triplanar_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_metalness_default(mut self, val: f32) -> Self {
        self.params.insert(
            "metalness_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_metalness_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metalness_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specular_roughness_default(mut self, val: f32) -> Self {
        self.params.insert(
            "specular_roughness_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specular_roughness_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specular_roughness_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coat_roughness_default(mut self, val: f32) -> Self {
        self.params.insert(
            "coat_roughness_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coat_roughness_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_roughness_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsurface_scale_default(mut self, val: f32) -> Self {
        self.params.insert(
            "subsurface_scale_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subsurface_scale_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_scale_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_occlusion_default(mut self, val: f32) -> Self {
        self.params.insert(
            "occlusion_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_occlusion_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlusion_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bump_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "bump_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bump_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bump_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_texcoord(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_texcoord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_position_triplanar(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "position_triplanar".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_position_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "position_triplanar".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normal_triplanar(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "normal_triplanar".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_normal_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normal_triplanar".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_base_color_default(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "base_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_base_color_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "base_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specular_color_default(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "specular_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specular_color_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specular_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coat_color_default(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "coat_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_coat_color_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsurface_color_default(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "subsurface_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_subsurface_color_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emission_color_default(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "emission_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emission_color_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_color_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacity_default(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opacity_default".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opacity_default_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_default".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection(mut self, val: VopHmtlxpbrtexturesetProjection) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_triplanar_upaxis(mut self, val: VopHmtlxpbrtexturesetTriplanarUpaxis) -> Self {
        self.params.insert(
            "triplanar_upaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_triplanar_upaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triplanar_upaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_default_values_mode(mut self, val: VopHmtlxpbrtexturesetDefaultValuesMode) -> Self {
        self.params.insert(
            "default_values_mode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_default_values_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "default_values_mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bump_style(mut self, val: VopHmtlxpbrtexturesetBumpStyle) -> Self {
        self.params.insert(
            "bump_style".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_bump_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bump_style".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filtertype(mut self, val: &str) -> Self {
        self.params.insert(
            "filtertype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filtertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filtertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_base_color_file(mut self, val: &str) -> Self {
        self.params.insert(
            "base_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_base_color_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "base_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_metalness_file(mut self, val: &str) -> Self {
        self.params.insert(
            "metalness_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_metalness_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metalness_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specular_color_file(mut self, val: &str) -> Self {
        self.params.insert(
            "specular_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specular_color_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specular_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specular_roughness_file(mut self, val: &str) -> Self {
        self.params.insert(
            "specular_roughness_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specular_roughness_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specular_roughness_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coat_color_file(mut self, val: &str) -> Self {
        self.params.insert(
            "coat_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coat_color_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coat_roughness_file(mut self, val: &str) -> Self {
        self.params.insert(
            "coat_roughness_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coat_roughness_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_roughness_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsurface_color_file(mut self, val: &str) -> Self {
        self.params.insert(
            "subsurface_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsurface_color_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsurface_scale_file(mut self, val: &str) -> Self {
        self.params.insert(
            "subsurface_scale_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsurface_scale_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_scale_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emission_color_file(mut self, val: &str) -> Self {
        self.params.insert(
            "emission_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emission_color_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_color_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_occlusion_file(mut self, val: &str) -> Self {
        self.params.insert(
            "occlusion_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_occlusion_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlusion_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacity_file(mut self, val: &str) -> Self {
        self.params.insert(
            "opacity_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacity_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bump_height_file(mut self, val: &str) -> Self {
        self.params.insert(
            "bump_height_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bump_height_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bump_height_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bump_normal_file(mut self, val: &str) -> Self {
        self.params.insert(
            "bump_normal_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bump_normal_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bump_normal_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxpbrtextureset {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxpbrtextureset"
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

pub trait VopHmtlxpbrtexturesetOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "base_color"
    fn out_base_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("base_color".to_string()),
        }
    }
    /// Output pin: "metalness"
    fn out_metalness(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("metalness".to_string()),
        }
    }
    /// Output pin: "specular_color"
    fn out_specular_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("specular_color".to_string()),
        }
    }
    /// Output pin: "specular_roughness"
    fn out_specular_roughness(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("specular_roughness".to_string()),
        }
    }
    /// Output pin: "coat_color"
    fn out_coat_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("coat_color".to_string()),
        }
    }
    /// Output pin: "coat_roughness"
    fn out_coat_roughness(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("coat_roughness".to_string()),
        }
    }
    /// Output pin: "subsurface_color"
    fn out_subsurface_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("subsurface_color".to_string()),
        }
    }
    /// Output pin: "subsurface_scale"
    fn out_subsurface_scale(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("subsurface_scale".to_string()),
        }
    }
    /// Output pin: "emission_color"
    fn out_emission_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("emission_color".to_string()),
        }
    }
    /// Output pin: "occlusion"
    fn out_occlusion(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("occlusion".to_string()),
        }
    }
    /// Output pin: "opacity"
    fn out_opacity(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("opacity".to_string()),
        }
    }
    /// Output pin: "normal"
    fn out_normal(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("normal".to_string()),
        }
    }
}

impl VopHmtlxpbrtexturesetOutputs for VopHmtlxpbrtextureset {}
impl VopHmtlxpbrtexturesetOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxpbrtextureset>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxrampc {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxrampc {
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

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_ramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxrampc {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxrampc"
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

pub trait VopHmtlxrampcOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHmtlxrampcOutputs for VopHmtlxrampc {}
impl VopHmtlxrampcOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxrampc> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHmtlxrampcInnerExt {
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHmtlxrampcInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHmtlxrampc> {
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invlinear")
    }
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numkeys")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uniformramp")
    }
}

#[derive(Debug, Clone)]
pub struct VopHmtlxrampf {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxrampf {
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

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_ramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxrampf {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxrampf"
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

pub trait VopHmtlxrampfOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "out"
    fn out_out(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("out".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHmtlxrampfOutputs for VopHmtlxrampf {}
impl VopHmtlxrampfOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxrampf> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHmtlxrampfInnerExt {
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHmtlxrampfInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHmtlxrampf> {
    fn invlinear(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invlinear")
    }
    fn numkeys(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numkeys")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uniformramp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uniformramp")
    }
}

#[derive(Debug, Clone)]
pub struct VopHmtlxudimoffset {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxudimoffset {
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

    pub fn set_udim_patch_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_patch<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("patch".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_in<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_in(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_patch(mut self, val: i32) -> Self {
        self.params.insert(
            "patch".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_patch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "patch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxudimoffset {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxudimoffset"
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

pub trait VopHmtlxudimoffsetOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "texcoord"
    fn out_texcoord(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("texcoord".to_string()),
        }
    }
}

impl VopHmtlxudimoffsetOutputs for VopHmtlxudimoffset {}
impl VopHmtlxudimoffsetOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxudimoffset>
{
}

#[derive(Debug, Clone)]
pub struct VopHmtlxudimpatch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHmtlxudimpatch {
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

    pub fn set_texture_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_texcoord<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("texcoord".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_texcoord(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_texcoord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texcoord".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHmtlxudimpatch {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hmtlxudimpatch"
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

pub trait VopHmtlxudimpatchOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "patch"
    fn out_patch(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("patch".to_string()),
        }
    }
}

impl VopHmtlxudimpatchOutputs for VopHmtlxudimpatch {}
impl VopHmtlxudimpatchOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHmtlxudimpatch>
{
}

#[derive(Debug, Clone)]
pub struct VopHscriptinstance {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHscriptinstance {
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

    pub fn set_sop_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_opname<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opname".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_otl_files_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_otllist<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("otllist".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_basic_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_size_attribute_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_sizeattrib<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("sizeattrib".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_to_size_attribute_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_doscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("doscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_orient_to_point_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_doorient<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("doorient".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_non_graphics_token_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_nongraphics<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("nongraphics".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

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
    pub fn with_opname(mut self, val: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_otllist(mut self, val: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_otllist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "sizeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doscale(mut self, val: bool) -> Self {
        self.params.insert(
            "doscale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doorient(mut self, val: bool) -> Self {
        self.params.insert(
            "doorient".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nongraphics(mut self, val: bool) -> Self {
        self.params.insert(
            "nongraphics".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nongraphics_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nongraphics".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHscriptinstance {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hscriptinstance"
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

pub trait VopHscriptinstanceOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Geometry"
    fn out_geometry(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("geometry".to_string()),
        }
    }
}

impl VopHscriptinstanceOutputs for VopHscriptinstance {}
impl VopHscriptinstanceOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopHscriptinstance>
{
}

#[derive(Debug, Clone)]
pub struct VopHsvtorgb {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHsvtorgb {
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

    pub fn set_hsv_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hsv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hsv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hsv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hsv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hsv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hsv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hsv_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hsv_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hsv_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hsv_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHsvtorgb {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hsvtorgb"
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

pub trait VopHsvtorgbOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "RGB color"
    fn out_rgb(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("rgb".to_string()),
        }
    }
}

impl VopHsvtorgbOutputs for VopHsvtorgb {}
impl VopHsvtorgbOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHsvtorgb> {}

#[derive(Debug, Clone)]
pub struct VopHueshift {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHueshift {
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

    pub fn set_shift_amount_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_shift<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("shift".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_shift_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_amp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("amp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_shift_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_clr<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("clr".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shift".to_string(),
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
    pub fn with_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHueshift {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hueshift"
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

pub trait VopHueshiftOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Shift Color"
    fn out_nuclr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("nuClr".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
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
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopHueshiftOutputs for VopHueshift {}
impl VopHueshiftOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHueshift> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopHueshiftInnerExt {
    fn compare2zero(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if_ampnotzer0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopHueshiftInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopHueshift> {
    fn compare2zero(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compare2zero")
    }
    fn if_ampnotzer0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if_ampNotZer0")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHvecgetcomponPart {
    Component1 = 0,
    Component2 = 1,
    Component3 = 2,
    Component4 = 3,
}

#[derive(Debug, Clone)]
pub struct VopHvecgetcompon {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHvecgetcompon {
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

    pub fn set_input_vector4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hvec<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hvec".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hvec(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_hvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_part(mut self, val: VopHvecgetcomponPart) -> Self {
        self.params.insert(
            "part".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_part_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "part".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHvecgetcompon {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hvecgetcompon"
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

pub trait VopHvecgetcomponOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Component"
    fn out_fval(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval".to_string()),
        }
    }
}

impl VopHvecgetcomponOutputs for VopHvecgetcompon {}
impl VopHvecgetcomponOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHvecgetcompon> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopHvecsetcomponPart {
    Component1 = 0,
    Component2 = 1,
    Component3 = 2,
    Component4 = 3,
}

#[derive(Debug, Clone)]
pub struct VopHvecsetcompon {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHvecsetcompon {
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

    pub fn set_input_vector4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hvec<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hvec".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_fval<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fval".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hvec(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_hvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_part(mut self, val: VopHvecsetcomponPart) -> Self {
        self.params.insert(
            "part".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_part_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "part".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHvecsetcompon {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hvecsetcompon"
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

pub trait VopHvecsetcomponOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Vector4"
    fn out_newhvec(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("newhvec".to_string()),
        }
    }
}

impl VopHvecsetcomponOutputs for VopHvecsetcompon {}
impl VopHvecsetcomponOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHvecsetcompon> {}

#[derive(Debug, Clone)]
pub struct VopHvectofloat {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHvectofloat {
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

    pub fn set_input_vector4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hvec<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hvec".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hvec(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_hvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHvectofloat {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hvectofloat"
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

pub trait VopHvectofloatOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Component 1"
    fn out_fval1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval1".to_string()),
        }
    }
    /// Output pin: "Component 2"
    fn out_fval2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval2".to_string()),
        }
    }
    /// Output pin: "Component 3"
    fn out_fval3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval3".to_string()),
        }
    }
    /// Output pin: "Component 4"
    fn out_fval4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval4".to_string()),
        }
    }
}

impl VopHvectofloatOutputs for VopHvectofloat {}
impl VopHvectofloatOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHvectofloat> {}

#[derive(Debug, Clone)]
pub struct VopHvectohmatx {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHvectohmatx {
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

    pub fn set_row_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_vval1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("vval1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_row_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_vval2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("vval2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_row_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_vval3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("vval3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_row_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_vval4<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("vval4".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_vval1(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "vval1".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_vval1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vval1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vval2(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "vval2".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_vval2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vval2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vval3(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "vval3".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_vval3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vval3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vval4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "vval4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_vval4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vval4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHvectohmatx {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hvectohmatx"
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

pub trait VopHvectohmatxOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Matrix4"
    fn out_matx(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("matx".to_string()),
        }
    }
}

impl VopHvectohmatxOutputs for VopHvectohmatx {}
impl VopHvectohmatxOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHvectohmatx> {}

#[derive(Debug, Clone)]
pub struct VopHvectovec {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHvectovec {
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

    pub fn set_input_vector4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hvec<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hvec".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hvec(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_hvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHvectovec {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hvectovec"
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

pub trait VopHvectovecOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Vector"
    fn out_vec(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("vec".to_string()),
        }
    }
    /// Output pin: "Fourth Component"
    fn out_fval4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("fval4".to_string()),
        }
    }
}

impl VopHvectovecOutputs for VopHvectovec {}
impl VopHvectovecOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHvectovec> {}

#[derive(Debug, Clone)]
pub struct VopHvectovec2 {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopHvectovec2 {
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

    pub fn set_input_vector4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_hvec<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hvec".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hvec(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_hvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hvec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopHvectovec2 {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hvectovec2"
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

pub trait VopHvectovec2Outputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "First Half Output Vector"
    fn out_vec2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("vec2".to_string()),
        }
    }
    /// Output pin: "Second Half Output Vector"
    fn out_avec2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("avec2".to_string()),
        }
    }
}

impl VopHvectovec2Outputs for VopHvectovec2 {}
impl VopHvectovec2Outputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopHvectovec2> {}
