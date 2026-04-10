#[derive(Debug, Clone)]
pub struct VopJitteredhairnormal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopJitteredhairnormal {
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
    pub fn set_input_name_hairnn<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("hairNn".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_jitter_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_direction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_tex_coord<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("tex_coord".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_freq(mut self, val: f32) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
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
    pub fn with_tex_coord(mut self, val: f32) -> Self {
        self.params.insert(
            "tex_coord".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tex_coord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex_coord".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hairnn(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hairNn".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hairnn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hairNn".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopJitteredhairnormal {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "jitteredhairnormal"
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

pub trait VopJitteredhairnormalOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Hair Normal"
    fn out_hairnn(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("_hairNn".to_string()),
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

impl VopJitteredhairnormalOutputs for VopJitteredhairnormal {}
impl VopJitteredhairnormalOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopJitteredhairnormal>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopJitteredhairnormalInnerExt {
    fn compare_gt_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn if_freqgtzero(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopJitteredhairnormalInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopJitteredhairnormal>
{
    fn compare_gt_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compare_GT_0")
    }
    fn if_freqgtzero(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("if_freqGTzero")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopJoin {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopJoin {
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

    pub fn set_strings_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_strings<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("strings".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separators_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_input_name_spacer<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("spacer".to_string()),
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
    pub fn with_spacer(mut self, val: &str) -> Self {
        self.params.insert(
            "spacer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spacer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spacer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopJoin {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "join"
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

pub trait VopJoinOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Resulting Joined String"
    fn out_joinedstring(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("joinedstring".to_string()),
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

impl VopJoinOutputs for VopJoin {}
impl VopJoinOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopJoin> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopJoinInnerExt {
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn snippet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopJoinInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopJoin> {
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
