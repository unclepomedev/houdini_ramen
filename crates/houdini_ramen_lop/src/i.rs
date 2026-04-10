#[derive(Debug, Clone)]
pub struct LopInlineusd {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopInlineusd {
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

    pub fn with_usdsource(mut self, val: &str) -> Self {
        self.params.insert(
            "usdsource".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usdsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usdsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_allowediting(mut self, val: bool) -> Self {
        self.params.insert(
            "allowediting".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowediting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowediting".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopInlineusd {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "inlineusd"
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

pub trait LopInlineusdOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopInlineusdOutputs for LopInlineusd {}
impl LopInlineusdOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopInlineusd> {}

#[derive(Debug, Clone)]
pub struct LopInsertionpoint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopInsertionpoint {
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

    pub fn with_descriptor(mut self, val: &str) -> Self {
        self.params.insert(
            "descriptor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_descriptor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "descriptor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopInsertionpoint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "insertionpoint"
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

pub trait LopInsertionpointOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopInsertionpointOutputs for LopInsertionpoint {}
impl LopInsertionpointOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopInsertionpoint>
{
}

#[derive(Debug, Clone)]
pub struct LopInstancer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopInstancer {
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
    pub fn set_possible_prototype_source_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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

    pub fn with_pruneamount(mut self, val: f32) -> Self {
        self.params.insert(
            "pruneamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pruneamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pruneamount".to_string(),
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
    pub fn with_method(mut self, val: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
    pub fn with_transformsourcemode(mut self, val: &str) -> Self {
        self.params.insert(
            "transformsourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transformsourcemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transformsourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_locationpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "locationpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_locationpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "locationpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showlopstage(mut self, val: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showlopstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointsoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "pointsoppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointsoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointsoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointsopgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pointsopgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointsopgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointsopgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prunemode(mut self, val: &str) -> Self {
        self.params.insert(
            "prunemode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prunemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prunemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_instancename(mut self, val: &str) -> Self {
        self.params.insert(
            "instancename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_instancename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protopattern(mut self, val: &str) -> Self {
        self.params.insert(
            "protopattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protopattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protopattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protosourcemode(mut self, val: &str) -> Self {
        self.params.insert(
            "protosourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protosourcemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protosourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protoreftype(mut self, val: &str) -> Self {
        self.params.insert(
            "protoreftype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protoreftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protoreftype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlemissingprototypes(mut self, val: &str) -> Self {
        self.params.insert(
            "handlemissingprototypes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlemissingprototypes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlemissingprototypes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protorootkind(mut self, val: &str) -> Self {
        self.params.insert(
            "protorootkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protorootkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protorootkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protoindexsrc(mut self, val: &str) -> Self {
        self.params.insert(
            "protoindexsrc".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protoindexsrc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protoindexsrc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indexattr(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indexattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameattr(mut self, val: &str) -> Self {
        self.params.insert(
            "nameattr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setextents(mut self, val: bool) -> Self {
        self.params.insert(
            "setextents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setextents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setorient(mut self, val: bool) -> Self {
        self.params.insert(
            "setorient".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setscale(mut self, val: bool) -> Self {
        self.params.insert(
            "setscale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hidesourceprims(mut self, val: bool) -> Self {
        self.params.insert(
            "hidesourceprims".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hidesourceprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hidesourceprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usenameattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usenameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_makeuniquepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "makeuniquepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makeuniquepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makeuniquepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprotoarea(mut self, val: bool) -> Self {
        self.params.insert(
            "createprotoarea".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprotoarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprotoarea".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hideprotosourceprims(mut self, val: bool) -> Self {
        self.params.insert(
            "hideprotosourceprims".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hideprotosourceprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hideprotosourceprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_protouseroot(mut self, val: bool) -> Self {
        self.params.insert(
            "protouseroot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_protouseroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protouseroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_onlycopyprotoprims(mut self, val: bool) -> Self {
        self.params.insert(
            "onlycopyprotoprims".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onlycopyprotoprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onlycopyprotoprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_allowmissingprototypes(mut self, val: bool) -> Self {
        self.params.insert(
            "allowmissingprototypes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowmissingprototypes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowmissingprototypes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_warnonskipped(mut self, val: bool) -> Self {
        self.params.insert(
            "warnonskipped".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_warnonskipped_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "warnonskipped".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopInstancer {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "instancer"
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

pub trait LopInstancerOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopInstancerOutputs for LopInstancer {}
impl LopInstancerOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopInstancer> {}

#[derive(Debug, Clone)]
pub struct LopIsolatescene {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopIsolatescene {
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

    pub fn with_cameratime(mut self, val: f32) -> Self {
        self.params.insert(
            "cameratime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cameratime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cameratime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_camerapad(mut self, val: f32) -> Self {
        self.params.insert(
            "camerapad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_camerapad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camerapad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "excludepattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludepattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskedpayloads(mut self, val: bool) -> Self {
        self.params.insert(
            "maskedpayloads".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskedpayloads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskedpayloads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_docambounds(mut self, val: bool) -> Self {
        self.params.insert(
            "docambounds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docambounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docambounds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_newlayerforedits(mut self, val: bool) -> Self {
        self.params.insert(
            "newlayerforedits".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_newlayerforedits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newlayerforedits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopIsolatescene {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "isolatescene"
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
    fn get_dive_target(&self) -> Option<&'static str> {
        Some("edits")
    }
}

pub trait LopIsolatesceneOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopIsolatesceneOutputs for LopIsolatescene {}
impl LopIsolatesceneOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopIsolatescene> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopIsolatesceneInnerExt {
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopIsolatesceneInnerExt for houdini_ramen_core::graph::InnerGraph<'a, LopIsolatescene> {
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("edits/output0")
    }
}
