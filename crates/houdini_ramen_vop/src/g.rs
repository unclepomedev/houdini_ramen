#[derive(Debug, Clone)]
pub struct VopGain {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGain {
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
    pub fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "amount".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_amount<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("amount".to_string()),
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
    pub fn with_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "amount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount".to_string(),
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
    pub fn with_amount_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "amount_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amount_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount_u".to_string(),
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
    pub fn with_amount_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amount_c".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amount_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount_c".to_string(),
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
    pub fn with_amount_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amount_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amount_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount_v".to_string(),
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
    pub fn with_amount_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "amount_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_amount_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount_v4".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for VopGain {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "gain"
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

pub trait VopGainOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Result"
    fn out_gain(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("gain".to_string()),
        }
    }
}

impl VopGainOutputs for VopGain {}
impl VopGainOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGain> {}

pub trait VopGainWiringExt {
    fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_val<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_amount<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGainWiringExt for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGain> {
    fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_val<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("val", output)
    }
    fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_amount<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("amount", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGather {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGather {
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

    pub fn with_category(mut self, val: &str) -> Self {
        self.params.insert(
            "category".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_category_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "category".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_import_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("import{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_import_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("import{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGather {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "gather"
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

pub trait VopGatherOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 0"
    fn out_output0(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output0".to_string()),
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

impl VopGatherOutputs for VopGather {}
impl VopGatherOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGather> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGatherInnerExt {
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGatherInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopGather> {
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopGeneralfresnelFresStyle {
    PhysicallyBased = 0,
    /// Use Connected 'fresnel'
    UseConnectedFresnel = 1,
}

#[derive(Debug, Clone)]
pub struct VopGeneralfresnel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGeneralfresnel {
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

    pub fn set_n_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("N".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_i_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("I".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
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
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("I".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fresnel_blending_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "fres_enable".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fres_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fres_enable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_thin_film_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "thinfilm".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_thinfilm<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("thinfilm".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fresnel_style_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "fres_style".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fres_style<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fres_style".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_inside_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "iorin".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_iorin<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("iorin".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_outside_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "iorout".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_iorout<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("iorout".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fresnel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "fresnel".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fresnel<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fresnel".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_iorin(mut self, val: f32) -> Self {
        self.params.insert(
            "iorin".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iorin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iorin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iorout(mut self, val: f32) -> Self {
        self.params.insert(
            "iorout".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iorout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iorout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fres_style(mut self, val: VopGeneralfresnelFresStyle) -> Self {
        self.params.insert(
            "fres_style".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fres_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fres_style".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fres_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fres_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fres_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fres_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thinfilm(mut self, val: bool) -> Self {
        self.params.insert(
            "thinfilm".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_thinfilm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thinfilm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGeneralfresnel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "generalfresnel"
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

pub trait VopGeneralfresnelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "kr"
    fn out_kr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("kr".to_string()),
        }
    }
    /// Output pin: "kt"
    fn out_kt(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("kt".to_string()),
        }
    }
    /// Output pin: "R"
    fn out_r(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("R".to_string()),
        }
    }
    /// Output pin: "T"
    fn out_t(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("T".to_string()),
        }
    }
    /// Output pin: "eta"
    fn out_eta(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("eta".to_string()),
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

impl VopGeneralfresnelOutputs for VopGeneralfresnel {}
impl VopGeneralfresnelOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopGeneralfresnel>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGeneralfresnelInnerExt {
    fn r(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn t(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn facefwd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_i_n1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn divide1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn eta(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fresnel1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fresnelstyle_to_int(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn isconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn kr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn kr_mode(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn kt(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn kt_mode(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn thinfilmfresnel1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGeneralfresnelInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopGeneralfresnel>
{
    fn r(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R")
    }
    fn t(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("T")
    }
    fn facefwd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("_facefwd")
    }
    fn clamp1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp1")
    }
    fn complement1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement1")
    }
    fn compute_i_n1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_I_N1")
    }
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("const1")
    }
    fn divide1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("divide1")
    }
    fn eta(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("eta")
    }
    fn fresnel1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fresnel1")
    }
    fn fresnelstyle_to_int(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fresnelstyle_to_int")
    }
    fn isconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("isconnected1")
    }
    fn kr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("kr")
    }
    fn kr_mode(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("kr_mode")
    }
    fn kt(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("kt")
    }
    fn kt_mode(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("kt_mode")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null2")
    }
    fn null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null3")
    }
    fn null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null4")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn subinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput2")
    }
    fn subinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput3")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn thinfilmfresnel1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("thinfilmfresnel1")
    }
}
pub trait VopGeneralfresnelWiringExt {
    fn set_n_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_i_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_fresnel_blending_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_fres_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_thin_film_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_input_name_thinfilm<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_fresnel_style_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_fres_style<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_inside_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_iorin<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_outside_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_iorout<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_fresnel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_fresnel<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGeneralfresnelWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGeneralfresnel>
{
    fn set_n_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("N", output)
    }
    fn set_i_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("I", output)
    }
    fn set_fresnel_blending_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_fres_enable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("fres_enable", output)
    }
    fn set_thin_film_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_thinfilm<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("thinfilm", output)
    }
    fn set_fresnel_style_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_fres_style<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("fres_style", output)
    }
    fn set_inside_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(5, output)
    }
    fn set_input_name_iorin<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("iorin", output)
    }
    fn set_outside_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(6, output)
    }
    fn set_input_name_iorout<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("iorout", output)
    }
    fn set_fresnel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(7, output)
    }
    fn set_input_name_fresnel<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("fresnel", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGenericshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGenericshader {
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

    pub fn with_sidefx_name(mut self, val: &str) -> Self {
        self.params.insert(
            "sidefx_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sidefx_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sidefx_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGenericshader {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "genericshader"
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

pub trait VopGenericshaderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 0"
    fn out_output0(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output0".to_string()),
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

impl VopGenericshaderOutputs for VopGenericshader {}
impl VopGenericshaderOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGenericshader> {}

#[derive(Debug, Clone)]
pub struct VopGeometryvopglobal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGeometryvopglobal {
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

impl houdini_ramen_core::types::HoudiniNode for VopGeometryvopglobal {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "geometryvopglobal"
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

pub trait VopGeometryvopglobalOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "P"
    fn out_p(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("P".to_string()),
        }
    }
    /// Output pin: "Velocity"
    fn out_v(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("v".to_string()),
        }
    }
    /// Output pin: "Force"
    fn out_force(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("force".to_string()),
        }
    }
    /// Output pin: "Age"
    fn out_age(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("age".to_string()),
        }
    }
    /// Output pin: "Life"
    fn out_life(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("life".to_string()),
        }
    }
    /// Output pin: "Id"
    fn out_id(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("id".to_string()),
        }
    }
    /// Output pin: "Cd"
    fn out_cd(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Cd".to_string()),
        }
    }
    /// Output pin: "UV"
    fn out_uv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("uv".to_string()),
        }
    }
    /// Output pin: "N"
    fn out_n(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("N".to_string()),
        }
    }
    /// Output pin: "Time"
    fn out_time(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Time".to_string()),
        }
    }
    /// Output pin: "Time Inc"
    fn out_timeinc(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("TimeInc".to_string()),
        }
    }
    /// Output pin: "Frame"
    fn out_frame(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Frame".to_string()),
        }
    }
    /// Output pin: "Point Number"
    fn out_ptnum(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("ptnum".to_string()),
        }
    }
    /// Output pin: "Primitive Number"
    fn out_primnum(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("primnum".to_string()),
        }
    }
    /// Output pin: "Vertex Number"
    fn out_vtxnum(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("vtxnum".to_string()),
        }
    }
    /// Output pin: "Number of Points"
    fn out_numpt(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("numpt".to_string()),
        }
    }
    /// Output pin: "Number of Prims"
    fn out_numprim(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("numprim".to_string()),
        }
    }
    /// Output pin: "Number of Vertices"
    fn out_numvtx(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("numvtx".to_string()),
        }
    }
    /// Output pin: "First Input"
    fn out_opinput1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("OpInput1".to_string()),
        }
    }
    /// Output pin: "Second Input"
    fn out_opinput2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("OpInput2".to_string()),
        }
    }
    /// Output pin: "Third Input"
    fn out_opinput3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("OpInput3".to_string()),
        }
    }
    /// Output pin: "Fourth Input"
    fn out_opinput4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("OpInput4".to_string()),
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

impl VopGeometryvopglobalOutputs for VopGeometryvopglobal {}
impl VopGeometryvopglobalOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopGeometryvopglobal>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGeometryvopglobalInnerExt {
    fn cd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn frame(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn time(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn timeinc(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn age(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn force(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn id(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn life(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numprim(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numpt(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn numvtx(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn primnum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ptnum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uv(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn v(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vtxnum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGeometryvopglobalInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopGeometryvopglobal>
{
    fn cd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Cd")
    }
    fn frame(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Frame")
    }
    fn n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N")
    }
    fn p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P")
    }
    fn time(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Time")
    }
    fn timeinc(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Timeinc")
    }
    fn age(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("age")
    }
    fn force(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("force")
    }
    fn id(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("id")
    }
    fn life(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("life")
    }
    fn numprim(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numprim")
    }
    fn numpt(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numpt")
    }
    fn numvtx(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("numvtx")
    }
    fn opinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opinput1")
    }
    fn opinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opinput2")
    }
    fn opinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opinput3")
    }
    fn opinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opinput4")
    }
    fn primnum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("primnum")
    }
    fn ptnum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ptnum")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uv(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uv")
    }
    fn v(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("v")
    }
    fn vtxnum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vtxnum")
    }
}

#[derive(Debug, Clone)]
pub struct VopGeometryvopoutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGeometryvopoutput {
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

    pub fn set_p_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_velocity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("v".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_v<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("v".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_force_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "force".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_force<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("force".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("Cd".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cd<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Cd".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("N".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
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
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGeometryvopoutput {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "geometryvopoutput"
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

pub trait VopGeometryvopoutputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 0"
    fn out_output0(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output0".to_string()),
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

impl VopGeometryvopoutputOutputs for VopGeometryvopoutput {}
impl VopGeometryvopoutputOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopGeometryvopoutput>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGeometryvopoutputInnerExt {
    fn cd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn force(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn v(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGeometryvopoutputInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopGeometryvopoutput>
{
    fn cd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Cd")
    }
    fn n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N")
    }
    fn p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P")
    }
    fn force(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("force")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn v(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("v")
    }
}
pub trait VopGeometryvopoutputWiringExt {
    fn set_p_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_velocity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_v<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_force_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_force<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_cd<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
}

impl<'a, 'g, C> VopGeometryvopoutputWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGeometryvopoutput>
{
    fn set_p_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("P", output)
    }
    fn set_velocity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_v<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("v", output)
    }
    fn set_force_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_force<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("force", output)
    }
    fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_cd<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("Cd", output)
    }
    fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("N", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGetattrib {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetattrib {
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

    pub fn set_geometry_file_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "file".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_attribute_class_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "atype".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_atype<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("atype".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_attribute_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "attrib".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_attrib<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("attrib".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_point_or_primitive_number_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("i1".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_i1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("i1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_vertex_number_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("i2".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_i2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("i2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "adefault".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_adefault<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("adefault".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_i1(mut self, val: i32) -> Self {
        self.params.insert(
            "i1".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_i1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "i1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_i2(mut self, val: i32) -> Self {
        self.params.insert(
            "i2".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_i2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "i2".to_string(),
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
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
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
    pub fn with_atype(mut self, val: &str) -> Self {
        self.params.insert(
            "atype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGetattrib {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getattrib"
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

pub trait VopGetattribOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Attribute Value"
    fn out_adata(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("adata".to_string()),
        }
    }
    /// Output pin: "Return 1 For Success, 0 For Failure"
    fn out_success(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("success".to_string()),
        }
    }
}

impl VopGetattribOutputs for VopGetattrib {}
impl VopGetattribOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetattrib> {}

pub trait VopGetattribWiringExt {
    fn set_geometry_file_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_file<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_attribute_class_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_atype<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_attribute_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_attrib<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_point_or_primitive_number_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_i1<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_vertex_number_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_i2<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_adefault<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGetattribWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGetattrib>
{
    fn set_geometry_file_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("file", output)
    }
    fn set_attribute_class_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_atype<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("atype", output)
    }
    fn set_attribute_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_attrib<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("attrib", output)
    }
    fn set_point_or_primitive_number_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_i1<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("i1", output)
    }
    fn set_vertex_number_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_i2<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("i2", output)
    }
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(5, output)
    }
    fn set_input_name_adefault<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("adefault", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGetblurp {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetblurp {
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

    pub fn set_blur_time_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "time".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_time<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("time".to_string()),
            (out.node_id, out.pin),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopGetblurp {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getblurP"
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

pub trait VopGetblurpOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Motion Blur P"
    fn out_blurp(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("blurP".to_string()),
        }
    }
}

impl VopGetblurpOutputs for VopGetblurp {}
impl VopGetblurpOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetblurp> {}

pub trait VopGetblurpWiringExt {
    fn set_blur_time_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_input_name_time<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
}

impl<'a, 'g, C> VopGetblurpWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGetblurp>
{
    fn set_blur_time_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_time<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("time", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGetdictelement {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetdictelement {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "dict".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "key".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("key".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "defaultvalue".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("defaultvalue".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_defaultvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "defaultvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defaultvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s5(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "defaultvalue_s5".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_defaultvalue_s5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "defaultvalue_s4".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_defaultvalue_s4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s3(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "defaultvalue_s3".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_s3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s6(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "defaultvalue_s6".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_s6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s7(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "defaultvalue_s7".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_defaultvalue_s7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s8(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "defaultvalue_s8".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_defaultvalue_s8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s1(mut self, val: i32) -> Self {
        self.params.insert(
            "defaultvalue_s1".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_defaultvalue_s1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s1".to_string(),
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
    pub fn with_defaultvalue_s2(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultvalue_s2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGetdictelement {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getdictelement"
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

pub trait VopGetdictelementOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Value at Index"
    fn out_value(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("value".to_string()),
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

impl VopGetdictelementOutputs for VopGetdictelement {}
impl VopGetdictelementOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetdictelement>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGetdictelementInnerExt {
    fn snippet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGetdictelementInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopGetdictelement>
{
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
pub trait VopGetdictelementWiringExt {
    fn set_dictionary_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_dict<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_key_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_key<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGetdictelementWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGetdictelement>
{
    fn set_dictionary_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_dict<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("dict", output)
    }
    fn set_key_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_key<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("key", output)
    }
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("defaultvalue", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGetelement {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetelement {
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

    pub fn set_array_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "array".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_array<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("array".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "index".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_index<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("index".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "defaultvalue".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("defaultvalue".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_defaultvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "defaultvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defaultvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s5(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "defaultvalue_s5".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_defaultvalue_s5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "defaultvalue_s4".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_defaultvalue_s4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s3(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "defaultvalue_s3".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_s3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s6(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "defaultvalue_s6".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_s6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s7(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "defaultvalue_s7".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_defaultvalue_s7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s8(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "defaultvalue_s8".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_defaultvalue_s8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s1(mut self, val: i32) -> Self {
        self.params.insert(
            "defaultvalue_s1".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_defaultvalue_s1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s1".to_string(),
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
    pub fn with_defaultvalue_s2(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultvalue_s2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGetelement {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getelement"
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

pub trait VopGetelementOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Value at Index"
    fn out_value(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("value".to_string()),
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

impl VopGetelementOutputs for VopGetelement {}
impl VopGetelementOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetelement> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGetelementInnerExt {
    fn snippet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGetelementInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopGetelement> {
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
pub trait VopGetelementWiringExt {
    fn set_array_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_array<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_index<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGetelementWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGetelement>
{
    fn set_array_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_array<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("array", output)
    }
    fn set_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_index<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("index", output)
    }
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("defaultvalue", output)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopGetlayerexportType {
    Float = 0,
    Vector = 1,
    Vector4 = 2,
}

#[derive(Debug, Clone)]
pub struct VopGetlayerexport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetlayerexport {
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

    pub fn set_layer_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "layer".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_layer<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("layer".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "name".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_name<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("name".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "defaultvalue".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("defaultvalue".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_defaultvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "defaultvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defaultvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "defaultvalue_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_defaultvalue_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultvalue_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "defaultvalue_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultvalue_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportparms(mut self, val: i32) -> Self {
        self.params.insert(
            "exportparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_exportparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type(mut self, val: VopGetlayerexportType) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGetlayerexport {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getlayerexport"
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

pub trait VopGetlayerexportOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Layer"
    fn out_value(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("value".to_string()),
        }
    }
    /// Output pin: "Export Found"
    fn out_found(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("found".to_string()),
        }
    }
    /// Output pin: "Export Index"
    fn out_index(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("index".to_string()),
        }
    }
}

impl VopGetlayerexportOutputs for VopGetlayerexport {}
impl VopGetlayerexportOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetlayerexport>
{
}

pub trait VopGetlayerexportWiringExt {
    fn set_layer_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_layer<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_name<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGetlayerexportWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGetlayerexport>
{
    fn set_layer_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_layer<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("layer", output)
    }
    fn set_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_name<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("name", output)
    }
    fn set_default_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_defaultvalue<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("defaultvalue", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGetobjxform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetobjxform {
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

    pub fn set_object_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "object".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_object<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("object".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGetobjxform {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getobjxform"
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

pub trait VopGetobjxformOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Object Transform"
    fn out_xform(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("xform".to_string()),
        }
    }
}

impl VopGetobjxformOutputs for VopGetobjxform {}
impl VopGetobjxformOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetobjxform> {}

pub trait VopGetobjxformWiringExt {
    fn set_object_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_object<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGetobjxformWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGetobjxform>
{
    fn set_object_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_object<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("object", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGetprimid {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetprimid {
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

impl houdini_ramen_core::types::HoudiniNode for VopGetprimid {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getprimid"
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

pub trait VopGetprimidOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Primitive ID"
    fn out_id(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("id".to_string()),
        }
    }
}

impl VopGetprimidOutputs for VopGetprimid {}
impl VopGetprimidOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetprimid> {}

#[derive(Debug, Clone)]
pub struct VopGetptextureid {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGetptextureid {
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

impl houdini_ramen_core::types::HoudiniNode for VopGetptextureid {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "getptextureid"
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

pub trait VopGetptextureidOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "PTexture ID"
    fn out_id(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("id".to_string()),
        }
    }
}

impl VopGetptextureidOutputs for VopGetptextureid {}
impl VopGetptextureidOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGetptextureid> {}

#[derive(Debug, Clone)]
pub struct VopGettransformchannels {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGettransformchannels {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "inputidx".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("inputidx".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_channel_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("C".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_c<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("C".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sample_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("I".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("I".to_string()),
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
    pub fn with_c(mut self, val: i32) -> Self {
        self.params.insert(
            "C".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "C".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_i(mut self, val: i32) -> Self {
        self.params.insert(
            "I".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_i_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "I".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGettransformchannels {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "gettransformchannels"
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

pub trait VopGettransformchannelsOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Translate"
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
    /// Output pin: "Transform Matrix"
    fn out_xform(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("xform".to_string()),
        }
    }
}

impl VopGettransformchannelsOutputs for VopGettransformchannels {}
impl VopGettransformchannelsOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopGettransformchannels>
{
}

pub trait VopGettransformchannelsWiringExt {
    fn set_input_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_inputidx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_channel_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_c<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_sample_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
}

impl<'a, 'g, C> VopGettransformchannelsWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGettransformchannels>
{
    fn set_input_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_inputidx<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("inputidx", output)
    }
    fn set_channel_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_c<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("C", output)
    }
    fn set_sample_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_i<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("I", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGingham {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGingham {
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("uv".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_checksize_0_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "checksize".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_checksize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("checksize".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_blur_0_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "blur".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_blur<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("blur".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_checksize(mut self, val: f32) -> Self {
        self.params.insert(
            "checksize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_checksize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checksize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "blur".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGingham {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "gingham"
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

pub trait VopGinghamOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Checker Amount"
    fn out_amount(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("amount".to_string()),
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

impl VopGinghamOutputs for VopGingham {}
impl VopGinghamOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGingham> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGinghamInnerExt {
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn inline1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGinghamInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopGingham> {
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
pub trait VopGinghamWiringExt {
    fn set_uv_0_0_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_uv<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_checksize_0_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_checksize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_blur_0_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_blur<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
}

impl<'a, 'g, C> VopGinghamWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGingham>
{
    fn set_uv_0_0_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_uv<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("uv", output)
    }
    fn set_checksize_0_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_checksize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("checksize", output)
    }
    fn set_blur_0_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_blur<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("blur", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGlass {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGlass {
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
    pub fn set_index_of_refraction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "eta".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_eta<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("eta".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "rbias".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rbias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rbias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "rough".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_diffuse_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "dcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "scolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflection_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "rcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_refraction_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "tcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_tcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(8));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_environment_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "envmap".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
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
            .remove(&houdini_ramen_core::types::InputPin::Index(9));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("envmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_environment_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "envcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_envcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(10));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("envcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_eta(mut self, val: f32) -> Self {
        self.params.insert(
            "eta".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rbias(mut self, val: f32) -> Self {
        self.params.insert(
            "rbias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rbias".to_string(),
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
    pub fn with_scolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_envcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "envcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_envcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envcolor".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopGlass {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "glass"
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

pub trait VopGlassOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
        }
    }
    /// Output pin: "Opacity"
    fn out_opacity(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("opacity".to_string()),
        }
    }
    /// Output pin: "Alpha"
    fn out_alpha(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("alpha".to_string()),
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

impl VopGlassOutputs for VopGlass {}
impl VopGlassOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGlass> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGlassInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_alphas(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_colors(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn facefwd_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fullopacity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tcolor_avg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tcolor_complem(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn translucent1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGlassInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopGlass> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn add_alphas(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_alphas")
    }
    fn add_colors(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_colors")
    }
    fn alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("alpha")
    }
    fn facefwd_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("facefwd_N")
    }
    fn fullopacity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fullopacity")
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
    fn tcolor_avg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tcolor_avg")
    }
    fn tcolor_complem(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tcolor_complem")
    }
    fn translucent1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("translucent1")
    }
}
pub trait VopGlassWiringExt {
    fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_index_of_refraction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_eta<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_reflection_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_rbias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_diffuse_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_dcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_scolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_reflection_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_rcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_refraction_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_tcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_environment_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_envmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_environment_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_envcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGlassWiringExt for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGlass> {
    fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("P", output)
    }
    fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("N", output)
    }
    fn set_index_of_refraction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_eta<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("eta", output)
    }
    fn set_reflection_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_rbias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("rbias", output)
    }
    fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("rough", output)
    }
    fn set_diffuse_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(5, output)
    }
    fn set_input_name_dcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("dcolor", output)
    }
    fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(6, output)
    }
    fn set_input_name_scolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("scolor", output)
    }
    fn set_reflection_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(7, output)
    }
    fn set_input_name_rcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("rcolor", output)
    }
    fn set_refraction_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(8, output)
    }
    fn set_input_name_tcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("tcolor", output)
    }
    fn set_environment_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(9, output)
    }
    fn set_input_name_envmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("envmap", output)
    }
    fn set_environment_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(10, output)
    }
    fn set_input_name_envcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("envcolor", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGlassshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGlassshader {
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

    pub fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("N".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "rough".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "ior".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ior<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ior".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reflect_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "reflcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_reflcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("reflcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_refract_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "refrcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_refrcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("refrcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "attenenable".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_attenenable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("attenenable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "attencolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_attencolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("attencolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_attenuation_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "attendensity".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_attendensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("attendensity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "fakecausticenable".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fakecausticenable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(8));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fakecausticenable".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transmission_tint_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "fakecaustictransmit".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fakecaustictransmit<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(9));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fakecaustictransmit".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_shadow_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "fakecausticshadow".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fakecausticshadow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(10));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fakecausticshadow".to_string()),
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
    pub fn with_ior(mut self, val: f32) -> Self {
        self.params.insert(
            "ior".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attendensity(mut self, val: f32) -> Self {
        self.params.insert(
            "attendensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attendensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attendensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fakecaustictransmit(mut self, val: f32) -> Self {
        self.params.insert(
            "fakecaustictransmit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fakecaustictransmit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fakecaustictransmit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fakecausticshadow(mut self, val: f32) -> Self {
        self.params.insert(
            "fakecausticshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fakecausticshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fakecausticshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refrcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refrcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refrcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refrcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attencolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "attencolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_attencolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attencolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attenenable(mut self, val: bool) -> Self {
        self.params.insert(
            "attenenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fakecausticenable(mut self, val: bool) -> Self {
        self.params.insert(
            "fakecausticenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fakecausticenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fakecausticenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGlassshader {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "glassshader"
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

pub trait VopGlassshaderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "layer"
    fn out_layer(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("layer".to_string()),
        }
    }
    /// Output pin: "BSDF"
    fn out_f(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("F".to_string()),
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

impl VopGlassshaderOutputs for VopGlassshader {}
impl VopGlassshaderOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGlassshader> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGlassshaderInnerExt {
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn attenuate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn color1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn color2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn condition(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn condition_in(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn conserveenergy1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn density(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn end_if1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn eta(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fakecaustics(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fresnel2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if_begin1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn invert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn layerpack1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrspecular1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pbrspecular2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rough(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn shadow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transmit(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn twoway1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGlassshaderInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopGlassshader> {
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add1")
    }
    fn attenuate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("attenuate1")
    }
    fn color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("color")
    }
    fn color1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("color1")
    }
    fn color2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("color2")
    }
    fn condition(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("condition")
    }
    fn condition_in(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("condition_in")
    }
    fn conserveenergy1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("conserveenergy1")
    }
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("const1")
    }
    fn density(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("density")
    }
    fn end_if1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("end_if1")
    }
    fn eta(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("eta")
    }
    fn fakecaustics(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fakecaustics")
    }
    fn fresnel2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fresnel2")
    }
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global1")
    }
    fn global2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global2")
    }
    fn if_begin1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if_begin1")
    }
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected1")
    }
    fn invert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invert1")
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
    fn multiply5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply5")
    }
    fn normalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize1")
    }
    fn pbrspecular1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrspecular1")
    }
    fn pbrspecular2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pbrspecular2")
    }
    fn rough(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rough")
    }
    fn shadow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("shadow")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn transmit(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transmit")
    }
    fn twoway1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("twoway1")
    }
}
pub trait VopGlassshaderWiringExt {
    fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_ior<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_reflect_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_reflcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_refract_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_refrcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_enable_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_attenenable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_attencolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_attenuation_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_attendensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_enable_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_fakecausticenable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_transmission_tint_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_fakecaustictransmit<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_shadow_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_fakecausticshadow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGlassshaderWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGlassshader>
{
    fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("N", output)
    }
    fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("rough", output)
    }
    fn set_ior_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_ior<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("ior", output)
    }
    fn set_reflect_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_reflcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("reflcolor", output)
    }
    fn set_refract_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_refrcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("refrcolor", output)
    }
    fn set_enable_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(5, output)
    }
    fn set_input_name_attenenable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("attenenable", output)
    }
    fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(6, output)
    }
    fn set_input_name_attencolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("attencolor", output)
    }
    fn set_attenuation_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(7, output)
    }
    fn set_input_name_attendensity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("attendensity", output)
    }
    fn set_enable_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(8, output)
    }
    fn set_input_name_fakecausticenable<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("fakecausticenable", output)
    }
    fn set_transmission_tint_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(9, output)
    }
    fn set_input_name_fakecaustictransmit<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("fakecaustictransmit", output)
    }
    fn set_shadow_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(10, output)
    }
    fn set_input_name_fakecausticshadow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("fakecausticshadow", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGlobal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGlobal {
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

    pub fn with_contexttype(mut self, val: &str) -> Self {
        self.params.insert(
            "contexttype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contexttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contexttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varname(mut self, val: &str) -> Self {
        self.params.insert(
            "varname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usemenu(mut self, val: bool) -> Self {
        self.params.insert(
            "usemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGlobal {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "global"
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

pub trait VopGlobalOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Surface Color"
    fn out_cf(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Cf".to_string()),
        }
    }
    /// Output pin: "Surface Opacity"
    fn out_of(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Of".to_string()),
        }
    }
    /// Output pin: "Surface Alpha"
    fn out_af(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Af".to_string()),
        }
    }
    /// Output pin: "Surface Position"
    fn out_p(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("P".to_string()),
        }
    }
    /// Output pin: "Surface Depth"
    fn out_pz(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Pz".to_string()),
        }
    }
    /// Output pin: "Direction from Eye to Surface"
    fn out_i(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("I".to_string()),
        }
    }
    /// Output pin: "Change in Position with s"
    fn out_dpds(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dPds".to_string()),
        }
    }
    /// Output pin: "Change in Position with t"
    fn out_dpdt(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dPdt".to_string()),
        }
    }
    /// Output pin: "Surface Normal"
    fn out_n(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("N".to_string()),
        }
    }
    /// Output pin: "Surface Geometric Normal"
    fn out_ng(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Ng".to_string()),
        }
    }
    /// Output pin: "Position of Eye"
    fn out_eye(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Eye".to_string()),
        }
    }
    /// Output pin: "Parametric s Coordinate (u)"
    fn out_s(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("s".to_string()),
        }
    }
    /// Output pin: "Parametric t Coordinate (v)"
    fn out_t(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("t".to_string()),
        }
    }
    /// Output pin: "Shading time"
    fn out_time(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("Time".to_string()),
        }
    }
    /// Output pin: "Change in Position with depth"
    fn out_dpdz(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dPdz".to_string()),
        }
    }
    /// Output pin: "Sample Identifier"
    fn out_sid(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("SID".to_string()),
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

impl VopGlobalOutputs for VopGlobal {}
impl VopGlobalOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGlobal> {}

#[derive(Debug, Clone)]
pub struct VopGlow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGlow {
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
    pub fn set_position_of_eye_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "Eye".to_string(),
            ));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("Cf".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("Of".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("Af".to_string()));
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
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("Af".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_inner_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "incolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_incolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("incolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_outer_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "outcolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_outcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("outcolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_core_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "corecolor".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_corecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("corecolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_core_brightness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "corebright".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_corebright<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(8));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("corebright".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_core_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "coresize".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_coresize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(9));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("coresize".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_inner_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "insize".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_insize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(10));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("insize".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_outer_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "outsize".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_outsize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(11));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("outsize".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "density".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_density<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(12));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("density".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_corebright(mut self, val: f32) -> Self {
        self.params.insert(
            "corebright".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corebright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "corebright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coresize(mut self, val: f32) -> Self {
        self.params.insert(
            "coresize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_insize(mut self, val: f32) -> Self {
        self.params.insert(
            "insize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_insize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outsize(mut self, val: f32) -> Self {
        self.params.insert(
            "outsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outsize".to_string(),
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
    pub fn with_incolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "incolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_incolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "incolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "outcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_outcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_corecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "corecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_corecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "corecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGlow {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "glow"
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

pub trait VopGlowOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Glow Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
        }
    }
    /// Output pin: "Alpha"
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

impl VopGlowOutputs for VopGlow {}
impl VopGlowOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGlow> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopGlowInnerExt {
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
    fn wo_space_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wo_space_p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn apply_density(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn attenuate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bright_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn brightness_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_core_size(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_glow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dist_eye_to_surface(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn divide1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn final_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn glow_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn glow_plus_brightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn glow_times_opacity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn invert(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn luminance1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mix_glow_colors(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale_glow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ten(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec_brightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopGlowInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopGlow> {
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
    fn wo_space_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("WO_Space_Eye")
    }
    fn wo_space_p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("WO_Space_P")
    }
    fn apply_density(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("apply_density")
    }
    fn attenuate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("attenuate")
    }
    fn bright_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bright_color")
    }
    fn brightness_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("brightness_amount")
    }
    fn clamp2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp2")
    }
    fn clamp_core_size(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_core_size")
    }
    fn compute_alpha(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_alpha")
    }
    fn compute_glow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_glow")
    }
    fn dist_eye_to_surface(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dist_eye_to_surface")
    }
    fn divide1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("divide1")
    }
    fn final_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("final_color")
    }
    fn glow_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("glow_amount")
    }
    fn glow_plus_brightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("glow_plus_brightness")
    }
    fn glow_times_opacity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("glow_times_opacity")
    }
    fn invert(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invert")
    }
    fn luminance1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("luminance1")
    }
    fn mix_glow_colors(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mix_glow_colors")
    }
    fn scale_glow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale_glow")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn ten(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ten")
    }
    fn vec_brightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec_brightness")
    }
}
pub trait VopGlowWiringExt {
    fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_position_of_eye_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_eye<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_surface_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_cf<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_surface_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_of<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_surface_alpha_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_af<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_inner_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_incolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_outer_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_outcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_core_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_corecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_core_brightness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_corebright<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_core_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_input_name_coresize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_inner_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_insize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_outer_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_outsize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_density<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGlowWiringExt for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGlow> {
    fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("P", output)
    }
    fn set_position_of_eye_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_eye<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("Eye", output)
    }
    fn set_surface_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_cf<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("Cf", output)
    }
    fn set_surface_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_of<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("Of", output)
    }
    fn set_surface_alpha_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_af<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("Af", output)
    }
    fn set_inner_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(5, output)
    }
    fn set_input_name_incolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("incolor", output)
    }
    fn set_outer_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(6, output)
    }
    fn set_input_name_outcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("outcolor", output)
    }
    fn set_core_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(7, output)
    }
    fn set_input_name_corecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("corecolor", output)
    }
    fn set_core_brightness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(8, output)
    }
    fn set_input_name_corebright<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("corebright", output)
    }
    fn set_core_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(9, output)
    }
    fn set_input_name_coresize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("coresize", output)
    }
    fn set_inner_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(10, output)
    }
    fn set_input_name_insize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("insize", output)
    }
    fn set_outer_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(11, output)
    }
    fn set_input_name_outsize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("outsize", output)
    }
    fn set_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(12, output)
    }
    fn set_input_name_density<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("density", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGradient3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGradient3d {
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

    pub fn set_n_3d_image_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "image3d".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_image3d<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("image3d".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_channel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "channel".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
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
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("channel".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "pos".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
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
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pos".to_string()),
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
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
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
    pub fn with_image3d(mut self, val: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_image3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopGradient3d {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "gradient3d"
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

pub trait VopGradient3dOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "3D Image Value"
    fn out_i3dval(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("i3dval".to_string()),
        }
    }
}

impl VopGradient3dOutputs for VopGradient3d {}
impl VopGradient3dOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGradient3d> {}

pub trait VopGradient3dWiringExt {
    fn set_n_3d_image_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_image3d<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_channel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_channel<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_pos<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_filter_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_filter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_filter_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_width<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGradient3dWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGradient3d>
{
    fn set_n_3d_image_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_image3d<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("image3d", output)
    }
    fn set_channel_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_channel<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("channel", output)
    }
    fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_pos<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("pos", output)
    }
    fn set_filter_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(3, output)
    }
    fn set_input_name_filter<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("filter", output)
    }
    fn set_filter_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(4, output)
    }
    fn set_input_name_width<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("width", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGrandom {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGrandom {
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

    pub fn set_seed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "seed".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_seed<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("seed".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_std_deviation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "sigma".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_sigma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("sigma".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_sigma(mut self, val: f32) -> Self {
        self.params.insert(
            "sigma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sigma".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for VopGrandom {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "grandom"
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

pub trait VopGrandomOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Random Value"
    fn out_nrand(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("nrand".to_string()),
        }
    }
}

impl VopGrandomOutputs for VopGrandom {}
impl VopGrandomOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGrandom> {}

pub trait VopGrandomWiringExt {
    fn set_seed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_seed<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O)
    -> Self;
    fn set_std_deviation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_sigma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGrandomWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGrandom>
{
    fn set_seed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_seed<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("seed", output)
    }
    fn set_std_deviation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_sigma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("sigma", output)
    }
}

#[derive(Debug, Clone)]
pub struct VopGrandomuv {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopGrandomuv {
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

    pub fn set_u_random_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("u".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_u<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("u".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_v_random_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name("v".to_string()));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_v<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("v".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_std_deviation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "sigma".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_sigma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("sigma".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_u(mut self, val: f32) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_v(mut self, val: f32) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sigma(mut self, val: f32) -> Self {
        self.params.insert(
            "sigma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sigma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopGrandomuv {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "grandomuv"
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

pub trait VopGrandomuvOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Random Value"
    fn out_nrand(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("nrand".to_string()),
        }
    }
}

impl VopGrandomuvOutputs for VopGrandomuv {}
impl VopGrandomuvOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopGrandomuv> {}

pub trait VopGrandomuvWiringExt {
    fn set_u_random_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_u<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_v_random_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_v<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_std_deviation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_sigma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> VopGrandomuvWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, VopGrandomuv>
{
    fn set_u_random_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_u<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("u", output)
    }
    fn set_v_random_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_v<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_name("v", output)
    }
    fn set_std_deviation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_sigma<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("sigma", output)
    }
}
