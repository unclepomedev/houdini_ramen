#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverPrepostPrerange {
    /// Pre render ROP's specified range
    PreRenderRopSSpecifiedRange = 0,
    /// Main job's first frame
    MainJobSFirstFrame = 1,
    /// Main job's last frame
    MainJobSLastFrame = 2,
    /// Main job's rendered frame range
    MainJobSRenderedFrameRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverPrepostPostrange {
    /// Post render ROP's specified range
    PostRenderRopSSpecifiedRange = 0,
    /// Main job's first frame
    MainJobSFirstFrame = 1,
    /// Main job's last frame
    MainJobSLastFrame = 2,
    /// Main job's rendered frame range
    MainJobSRenderedFrameRange = 3,
}

#[derive(Debug, Clone)]
pub struct DriverPrepost {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverPrepost {
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

    pub fn set_pre_render_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_main_job_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_post_render_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn with_prerange(mut self, val: DriverPrepostPrerange) -> Self {
        self.params.insert(
            "prerange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_prerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrange(mut self, val: DriverPrepostPostrange) -> Self {
        self.params.insert(
            "postrange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablepre(mut self, val: bool) -> Self {
        self.params.insert(
            "enablepre".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepre_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablepre".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablepost(mut self, val: bool) -> Self {
        self.params.insert(
            "enablepost".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablepost".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverPrepost {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "prepost"
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

pub trait DriverPrepostOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DriverPrepostOutputs for DriverPrepost {}
impl DriverPrepostOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DriverPrepost> {}

pub trait DriverPrepostWiringExt {
    fn set_pre_render_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_main_job_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self;
    fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_post_render_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_input3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> DriverPrepostWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, DriverPrepost>
{
    fn set_pre_render_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input1", output)
    }
    fn set_main_job_input<O: Into<houdini_ramen_core::types::NodeOutput>>(self, output: O) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input2", output)
    }
    fn set_post_render_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(2, output)
    }
    fn set_input_name_input3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input3", output)
    }
}
