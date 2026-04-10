#[derive(Debug, Clone)]
pub struct VopEdgefalloff {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEdgefalloff {
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

    pub fn set_direction_from_eye_to_surface_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("I".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("I".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("N".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_base_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "basecolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_basecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("basecolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_edge_falloff_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "falloff".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_falloff<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("falloff".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "falloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEdgefalloff {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "edgefalloff"
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

pub trait VopEdgefalloffOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Falloff Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
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

impl VopEdgefalloffOutputs for VopEdgefalloff {}
impl VopEdgefalloffOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEdgefalloff> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopEdgefalloffInnerExt {
    fn i_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn i_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn i_normalize(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_normalize(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn abs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dot(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopEdgefalloffInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopEdgefalloff> {
    fn i_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("I_global")
    }
    fn i_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("I_input")
    }
    fn i_normalize(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("I_normalize")
    }
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn n_normalize(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_normalize")
    }
    fn abs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("abs")
    }
    fn dot(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dot")
    }
    fn multiply(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply")
    }
    fn pow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pow")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopEfit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEfit {
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

    pub fn set_input_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "val".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_val<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("val".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_minimum_value_in_source_range_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "srcmin".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_srcmin<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("srcmin".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_maximum_value_in_source_range_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "srcmax".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_srcmax<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("srcmax".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_minimum_value_in_destination_range_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "destmin".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_destmin<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("destmin".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_maximum_value_in_destination_range_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "destmax".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_destmax<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("destmax".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_srcmin(mut self, val: f32) -> Self {
        self.params.insert(
            "srcmin".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srcmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax(mut self, val: f32) -> Self {
        self.params.insert(
            "srcmax".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srcmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin(mut self, val: f32) -> Self {
        self.params.insert(
            "destmin".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_destmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax(mut self, val: f32) -> Self {
        self.params.insert(
            "destmax".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_destmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_uf(mut self, val: f32) -> Self {
        self.params.insert(
            "srcmin_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srcmin_uf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_uf(mut self, val: f32) -> Self {
        self.params.insert(
            "srcmax_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srcmax_uf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_uf(mut self, val: f32) -> Self {
        self.params.insert(
            "destmin_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_destmin_uf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_uf(mut self, val: f32) -> Self {
        self.params.insert(
            "destmax_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_destmax_uf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_v2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "srcmin_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_srcmin_v2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_v2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "srcmax_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_srcmax_v2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_v2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "destmin_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_destmin_v2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_v2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "destmax_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_destmax_v2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_v2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_n".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_n".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_n".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_n".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_n".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_n".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_n".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_n".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_c".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_c".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_c".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_c".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_c".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_c".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_c".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_c".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_un(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_un".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_un_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_un".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_un(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_un".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_un_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_un".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_un(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_un".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_un_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_un".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_un(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_un".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_un_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_un".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmin_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmin_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "srcmax_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_srcmax_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmin_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmin_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "destmax_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_destmax_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmin_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "srcmin_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_srcmin_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srcmax_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "srcmax_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_srcmax_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmin_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "destmin_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_destmin_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destmax_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "destmax_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_destmax_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmax_v4".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for VopEfit {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "efit"
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

pub trait VopEfitOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Shifted Value"
    fn out_shift(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("shift".to_string()),
        }
    }
}

impl VopEfitOutputs for VopEfit {}
impl VopEfitOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEfit> {}

#[derive(Debug, Clone)]
pub struct VopEggShell {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEggShell {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "pos".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pos<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pos".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "norm".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_norm<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("norm".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_noise_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "freq".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bump_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "scale".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_object_space_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "space".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_space<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("space".to_string()),
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
    pub fn with_space(mut self, val: bool) -> Self {
        self.params.insert(
            "space".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEggShell {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "egg_shell"
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

pub trait VopEggShellOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "New Normal"
    fn out_nf(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("NF".to_string()),
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

impl VopEggShellOutputs for VopEggShell {}
impl VopEggShellOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEggShell> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopEggShellInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_normalized(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn aanoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn frontface1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopEggShellInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopEggShell> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn n_normalized(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_normalized")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn aanoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("aanoise1")
    }
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("displacenml1")
    }
    fn frontface1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("frontface1")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopEggshell {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEggshell {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("P".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("N".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_noise_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "freq".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_shell_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "shell".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_shell<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("shell".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_shell(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shell".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shell".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEggshell {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "eggshell"
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

pub trait VopEggshellOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
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

impl VopEggshellOutputs for VopEggshell {}
impl VopEggshellOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEggshell> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopEggshellInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_normalized(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn aanoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn frontface1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopEggshellInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopEggshell> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn n_normalized(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_normalized")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn aanoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("aanoise1")
    }
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("displacenml1")
    }
    fn frontface1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("frontface1")
    }
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lighting1")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopEigenvalues {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEigenvalues {
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

    pub fn set_input_matrix3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "matx".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("matx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEigenvalues {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "eigenvalues"
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

pub trait VopEigenvaluesOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Number Real Roots"
    fn out_nroot(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("nroot".to_string()),
        }
    }
    /// Output pin: "Real Values"
    fn out_r_values(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("r_values".to_string()),
        }
    }
    /// Output pin: "Imaginary Values"
    fn out_i_values(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("i_values".to_string()),
        }
    }
}

impl VopEigenvaluesOutputs for VopEigenvalues {}
impl VopEigenvaluesOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEigenvalues> {}

#[derive(Debug, Clone)]
pub struct VopEndswith {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEndswith {
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

    pub fn set_string_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "string".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_string<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("string".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_label_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "endswith".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_endswith<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("endswith".to_string()),
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
    pub fn with_string(mut self, val: &str) -> Self {
        self.params.insert(
            "string".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "string".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endswith(mut self, val: &str) -> Self {
        self.params.insert(
            "endswith".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endswith_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endswith".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEndswith {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "endswith"
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

pub trait VopEndswithOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Result"
    fn out_result(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("result".to_string()),
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

impl VopEndswithOutputs for VopEndswith {}
impl VopEndswithOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEndswith> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopEndswithInnerExt {
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn snippet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopEndswithInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopEndswith> {
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
pub struct VopEnginecurvegen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEnginecurvegen {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "opname".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "otllist".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("otllist".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_create_object_hda_instead_of_sop_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "objecthda".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_objecthda<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("objecthda".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_basic_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "size".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_orient_to_point_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "doorient".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_doorient<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("doorient".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_defer_geometry_creation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "defergeo".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_defergeo<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("defergeo".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_unload_geometry_after_cooking_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "unload".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_unload<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("unload".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sop_hda_requires_lod_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "requirelod".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_requirelod<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("requirelod".to_string()),
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
    pub fn with_objecthda(mut self, val: bool) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_objecthda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objecthda".to_string(),
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
    pub fn with_defergeo(mut self, val: bool) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defergeo".to_string(),
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
    pub fn with_requirelod(mut self, val: bool) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEnginecurvegen {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "enginecurvegen"
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

pub trait VopEnginecurvegenOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Geometry"
    fn out_geometry(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("geometry".to_string()),
        }
    }
}

impl VopEnginecurvegenOutputs for VopEnginecurvegen {}
impl VopEnginecurvegenOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopEnginecurvegen>
{
}

#[derive(Debug, Clone)]
pub struct VopEngineinstance {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEngineinstance {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "opname".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "otllist".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("otllist".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_create_object_hda_instead_of_sop_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "objecthda".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_objecthda<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("objecthda".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_basic_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "size".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_orient_to_point_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "doorient".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_doorient<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("doorient".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_defer_geometry_creation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "defergeo".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_defergeo<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("defergeo".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_unload_geometry_after_cooking_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "unload".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_unload<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("unload".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sop_hda_requires_lod_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "requirelod".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_requirelod<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("requirelod".to_string()),
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
    pub fn with_objecthda(mut self, val: bool) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_objecthda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objecthda".to_string(),
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
    pub fn with_defergeo(mut self, val: bool) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defergeo".to_string(),
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
    pub fn with_requirelod(mut self, val: bool) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEngineinstance {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "engineinstance"
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

pub trait VopEngineinstanceOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Geometry"
    fn out_geometry(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("geometry".to_string()),
        }
    }
}

impl VopEngineinstanceOutputs for VopEngineinstance {}
impl VopEngineinstanceOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopEngineinstance>
{
}

#[derive(Debug, Clone)]
pub struct VopEnginepointgen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEnginepointgen {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "opname".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "otllist".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("otllist".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_create_object_hda_instead_of_sop_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "objecthda".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_objecthda<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("objecthda".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_basic_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "size".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_size<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("size".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_orient_to_point_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "doorient".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_doorient<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("doorient".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_defer_geometry_creation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "defergeo".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_defergeo<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("defergeo".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_unload_geometry_after_cooking_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "unload".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_unload<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("unload".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sop_hda_requires_lod_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "requirelod".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_requirelod<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("requirelod".to_string()),
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
    pub fn with_objecthda(mut self, val: bool) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_objecthda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objecthda".to_string(),
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
    pub fn with_defergeo(mut self, val: bool) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defergeo".to_string(),
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
    pub fn with_requirelod(mut self, val: bool) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEnginepointgen {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "enginepointgen"
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

pub trait VopEnginepointgenOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Geometry"
    fn out_geometry(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("geometry".to_string()),
        }
    }
}

impl VopEnginepointgenOutputs for VopEnginepointgen {}
impl VopEnginepointgenOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopEnginepointgen>
{
}

#[derive(Debug, Clone)]
pub struct VopEnvmap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEnvmap {
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

    pub fn set_color_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "envmap".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_envmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("envmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_direction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "dir".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dir<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dir".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_wrap_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "wrap".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_wrap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("wrap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "filter".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_filter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("filter".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "width".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_width<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("width".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_angle_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "angle".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_angle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("angle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_border_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "border".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_border<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("border".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_extrapolate_derivatives_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "extrapol".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_extrapol<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("extrapol".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_interpolate_mip_levels_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "interp".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_interp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(8));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("interp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "tint".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(9));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transform_space_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "envobject".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_envobject<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(10));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("envobject".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_channel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "channel".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_channel<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(11));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("channel".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_border(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
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
    pub fn with_tint_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "tint_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_tint_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tint_v4".to_string(),
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
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
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
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap(mut self, val: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_envobject(mut self, val: &str) -> Self {
        self.params.insert(
            "envobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_envobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extrapol(mut self, val: bool) -> Self {
        self.params.insert(
            "extrapol".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extrapol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrapol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interp(mut self, val: bool) -> Self {
        self.params.insert(
            "interp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_interp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEnvmap {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "envmap"
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

pub trait VopEnvmapOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Environment Map Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
}

impl VopEnvmapOutputs for VopEnvmap {}
impl VopEnvmapOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEnvmap> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopEulertoquatXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct VopEulertoquat {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopEulertoquat {
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

    pub fn set_rotation_order_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "xyz".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xyz<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xyz".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_rotate_radians_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "rot".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_rot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xyz(mut self, val: VopEulertoquatXyz) -> Self {
        self.params.insert(
            "xyz".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xyz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopEulertoquat {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "eulertoquat"
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

pub trait VopEulertoquatOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Quaternion"
    fn out_quat(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("quat".to_string()),
        }
    }
}

impl VopEulertoquatOutputs for VopEulertoquat {}
impl VopEulertoquatOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopEulertoquat> {}

#[derive(Debug, Clone)]
pub struct VopExp {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopExp {
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

    pub fn set_input_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "val".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_val<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("val".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_val(mut self, val: f32) -> Self {
        self.params.insert(
            "val".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_uf(mut self, val: f32) -> Self {
        self.params.insert(
            "val_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_uf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_uf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "val_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_val_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_n".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_n".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_c".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_c".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_uv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_un(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_un".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_un_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_un".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_uc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_val_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "val_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_val_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_v4".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for VopExp {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "exp"
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

pub trait VopExpOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Exponent Value"
    fn out_exp(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("exp".to_string()),
        }
    }
}

impl VopExpOutputs for VopExp {}
impl VopExpOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopExp> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopExtractxformTrs {
    ScaleRotateTranslate = 0,
    ScaleTranslateRotate = 1,
    RotateScaleTranslate = 2,
    RotateTranslateScale = 3,
    TranslateScaleRotate = 4,
    TranslateRotateScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopExtractxformXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct VopExtractxform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopExtractxform {
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

    pub fn set_transform_order_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "trs".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_rotation_order_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "xyz".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xyz<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xyz".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_pivot_translate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "pivot".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pivot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pivot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_pivot_rotate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "pivotrotate".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pivotrotate<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pivotrotate".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transform_matrix_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "xform".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xform<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xform".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_pivot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pivot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pivotrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pivotrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pivotrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivotrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trs(mut self, val: VopExtractxformTrs) -> Self {
        self.params.insert(
            "trs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xyz(mut self, val: VopExtractxformXyz) -> Self {
        self.params.insert(
            "xyz".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xyz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopExtractxform {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "extractxform"
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

pub trait VopExtractxformOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Translation"
    fn out_trans(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("trans".to_string()),
        }
    }
    /// Output pin: "Rotate"
    fn out_rot(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("rot".to_string()),
        }
    }
    /// Output pin: "Scale"
    fn out_scale(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("scale".to_string()),
        }
    }
    /// Output pin: "Shear"
    fn out_shear(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("shear".to_string()),
        }
    }
}

impl VopExtractxformOutputs for VopExtractxform {}
impl VopExtractxformOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopExtractxform> {}
