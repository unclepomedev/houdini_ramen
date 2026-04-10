#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMakedirOutputfiles {
    KeepExistingOutputs = 0,
    AppendDirectoryPathToOutputs = 1,
    ReplaceOutputsWithDirectoryPath = 2,
}

#[derive(Debug, Clone)]
pub struct TopMakedir {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMakedir {
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

    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputfiles(mut self, val: TopMakedirOutputfiles) -> Self {
        self.params.insert(
            "outputfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dirname(mut self, val: &str) -> Self {
        self.params.insert(
            "dirname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dirname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useitemindex(mut self, val: bool) -> Self {
        self.params.insert(
            "useitemindex".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useitemindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useitemindex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMakedir {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "makedir"
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

pub trait TopMakedirOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMakedirOutputs for TopMakedir {}
impl TopMakedirOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMakedir> {}

#[derive(Debug, Clone)]
pub struct TopMapall {
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

impl TopMapall {
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
}

impl houdini_ramen_core::types::HoudiniNode for TopMapall {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mapall"
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

pub trait TopMapallOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMapallOutputs for TopMapall {}
impl TopMapallOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMapall> {}

#[derive(Debug, Clone)]
pub struct TopMapbyexpression {
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

impl TopMapbyexpression {
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

    pub fn with_mapstatic(mut self, val: i32) -> Self {
        self.params.insert(
            "mapstatic".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mapstatic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapstatic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mapdynamic(mut self, val: i32) -> Self {
        self.params.insert(
            "mapdynamic".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mapdynamic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapdynamic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMapbyexpression {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mapbyexpression"
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

pub trait TopMapbyexpressionOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMapbyexpressionOutputs for TopMapbyexpression {}
impl TopMapbyexpressionOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<TopMapbyexpression>
{
}

#[derive(Debug, Clone)]
pub struct TopMapbyindex {
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

impl TopMapbyindex {
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
}

impl houdini_ramen_core::types::HoudiniNode for TopMapbyindex {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mapbyindex"
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

pub trait TopMapbyindexOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMapbyindexOutputs for TopMapbyindex {}
impl TopMapbyindexOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMapbyindex> {}

#[derive(Debug, Clone)]
pub struct TopMapbyrange {
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

impl TopMapbyrange {
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

    pub fn with_upstream_range(mut self, val: i32) -> Self {
        self.params.insert(
            "upstream_range".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_upstream_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upstream_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_downstream_range(mut self, val: i32) -> Self {
        self.params.insert(
            "downstream_range".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_downstream_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "downstream_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_left_offset(mut self, val: i32) -> Self {
        self.params.insert(
            "left_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_right_offset(mut self, val: i32) -> Self {
        self.params.insert(
            "right_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMapbyrange {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mapbyrange"
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

pub trait TopMapbyrangeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMapbyrangeOutputs for TopMapbyrange {}
impl TopMapbyrangeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMapbyrange> {}

#[derive(Debug, Clone)]
pub struct TopMatnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMatnet {
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

impl houdini_ramen_core::types::HoudiniNode for TopMatnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "matnet"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverCopyinputs {
    NoIterations = 0,
    FirstIteration = 1,
    AllIterations = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgCooktype {
    /// Shared Server (Deprecated)
    SharedServerDeprecated = 0,
    Service = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgServicereset {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgServiceresetwhen {
    BeforeCook = 0,
    AfterCook = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopMayaserver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMayaserver {
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

    pub fn trigger_createservice(mut self) -> Self {
        self.params.insert(
            "createservice".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert(
            "manageservices".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_commandport(mut self, val: i32) -> Self {
        self.params.insert(
            "commandport".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_commandport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "commandport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timeout(mut self, val: i32) -> Self {
        self.params.insert(
            "timeout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyinputs(mut self, val: TopMayaserverCopyinputs) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopMayaserverPdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopMayaserverPdgServicereset) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopMayaserverPdgServiceresetwhen) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMayaserverPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopMayaserverPdgWorkitempriority) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_feedbackpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalhost(mut self, val: &str) -> Self {
        self.params.insert(
            "externalhost".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalhost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalhost".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_serverbinary(mut self, val: &str) -> Self {
        self.params.insert(
            "serverbinary".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_serverbinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "serverbinary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "iterattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizeattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "sizeattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizeattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "numattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterationsfromupstream(mut self, val: bool) -> Self {
        self.params.insert(
            "iterationsfromupstream".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iterationsfromupstream_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterationsfromupstream".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loopsequential(mut self, val: bool) -> Self {
        self.params.insert(
            "loopsequential".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loopsequential_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopsequential".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_feedbackattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackfiles(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_feedbackfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalserver(mut self, val: bool) -> Self {
        self.params.insert(
            "externalserver".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_externalserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalserver".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMayaserver {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mayaserver"
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

pub trait TopMayaserverOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMayaserverOutputs for TopMayaserver {}
impl TopMayaserverOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMayaserver> {}

#[derive(Debug, Clone)]
pub struct TopMerge {
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

impl TopMerge {
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

    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extrainput_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("extrainput{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extrainput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("extrainput{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepfailed(mut self, val: bool) -> Self {
        self.params.insert(
            "keepfailed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepfailed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepfailed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableextrainput_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enableextrainput{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableextrainput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enableextrainput{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMerge {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "merge"
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

pub trait TopMergeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMergeOutputs for TopMerge {}
impl TopMergeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMerge> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnCleanauxiliary {
    No = 0,
    Yes = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnDeterministic {
    Yes = 0,
    No = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone)]
pub struct TopMlPreprocessoidn {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMlPreprocessoidn {
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

    pub fn with_deviceid(mut self, val: i32) -> Self {
        self.params.insert(
            "deviceid".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deviceid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deviceid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicecount(mut self, val: i32) -> Self {
        self.params.insert(
            "devicecount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_devicecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "devicecount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cleanauxiliary(mut self, val: TopMlPreprocessoidnCleanauxiliary) -> Self {
        self.params.insert(
            "cleanauxiliary".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cleanauxiliary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cleanauxiliary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deterministic(mut self, val: TopMlPreprocessoidnDeterministic) -> Self {
        self.params.insert(
            "deterministic".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deterministic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deterministic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopMlPreprocessoidnPythonbin) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlPreprocessoidnPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlPreprocessoidnPdgWorkitempriority,
    ) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputdirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "inputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trainingdataset(mut self, val: &str) -> Self {
        self.params.insert(
            "trainingdataset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trainingdataset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trainingdataset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_validationdataset(mut self, val: &str) -> Self {
        self.params.insert(
            "validationdataset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_validationdataset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validationdataset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extension(mut self, val: &str) -> Self {
        self.params.insert(
            "extension".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extension".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputdirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "outputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputdirectory".to_string(),
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
    pub fn with_inputfeatures(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfeatures".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputfeatures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfeatures".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transfer(mut self, val: &str) -> Self {
        self.params.insert(
            "transfer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transfer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transfer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicename(mut self, val: &str) -> Self {
        self.params.insert(
            "devicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "devicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "python".to_string(),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetransfer(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransfer".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransfer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransfer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedevicename(mut self, val: bool) -> Self {
        self.params.insert(
            "usedevicename".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedevicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedevicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMlPreprocessoidn {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ml_preprocessoidn"
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

pub trait TopMlPreprocessoidnOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMlPreprocessoidnOutputs for TopMlPreprocessoidn {}
impl TopMlPreprocessoidnOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<TopMlPreprocessoidn>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopMlPreprocessoidnInnerExt {
    fn load_parameters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn partitionbyindex1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pythonscript1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pythonvenv1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopMlPreprocessoidnInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, TopMlPreprocessoidn>
{
    fn load_parameters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("load_parameters")
    }
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn partitionbyindex1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("partitionbyindex1")
    }
    fn pythonscript1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pythonscript1")
    }
    fn pythonvenv1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pythonvenv1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlRegressionkernelKerneltype {
    Gaussian = 0,
    Polynomial = 1,
    Sigmoid = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlRegressionkernelPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlRegressionkernelPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone)]
pub struct TopMlRegressionkernel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMlRegressionkernel {
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

    pub fn with_weightdecay(mut self, val: f32) -> Self {
        self.params.insert(
            "weightdecay".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weightdecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightdecay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_errorthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "errorthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_errorthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorthreshold".to_string(),
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
    pub fn with_polynomialoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "polynomialoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_polynomialoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polynomialoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sigmoidscale(mut self, val: f32) -> Self {
        self.params.insert(
            "sigmoidscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigmoidscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sigmoidscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sigmoidoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "sigmoidoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigmoidoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sigmoidoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_polynomialdegree(mut self, val: i32) -> Self {
        self.params.insert(
            "polynomialdegree".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_polynomialdegree_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polynomialdegree".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kerneltype(mut self, val: TopMlRegressionkernelKerneltype) -> Self {
        self.params.insert(
            "kerneltype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_kerneltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kerneltype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlRegressionkernelPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlRegressionkernelPdgWorkitempriority,
    ) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "datasetfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datasetfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "datasetbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datasetbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernelfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "kernelfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernelfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernelfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernelbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "kernelbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernelbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernelbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "modelsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "modelbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logsfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "logsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logsfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "logbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logtostandardoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "logtostandardoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_logtostandardoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logtostandardoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMlRegressionkernel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ml_regressionkernel"
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

pub trait TopMlRegressionkernelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMlRegressionkernelOutputs for TopMlRegressionkernel {}
impl TopMlRegressionkernelOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<TopMlRegressionkernel>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnCleanauxiliary {
    No = 0,
    Yes = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnDeterministic {
    Yes = 0,
    No = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopMlTrainoidn {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMlTrainoidn {
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

    pub fn with_learningrate(mut self, val: f32) -> Self {
        self.params.insert(
            "learningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "learningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxlearningrate(mut self, val: f32) -> Self {
        self.params.insert(
            "maxlearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxlearningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxlearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_learningratewarmup(mut self, val: f32) -> Self {
        self.params.insert(
            "learningratewarmup".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningratewarmup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "learningratewarmup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_msssimweights(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "msssimweights".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_msssimweights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "msssimweights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_epochs(mut self, val: i32) -> Self {
        self.params.insert(
            "epochs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_epochs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "epochs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_validationrate(mut self, val: i32) -> Self {
        self.params.insert(
            "validationrate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_validationrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validationrate".to_string(),
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
    pub fn with_tilesize(mut self, val: i32) -> Self {
        self.params.insert(
            "tilesize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tilesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tilesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_batchsize(mut self, val: i32) -> Self {
        self.params.insert(
            "batchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_batchsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_workerthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "workerthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_workerthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "workerthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params.insert(
            "seed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
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
    pub fn with_exportepoch(mut self, val: i32) -> Self {
        self.params.insert(
            "exportepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_exportepoch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deviceid(mut self, val: i32) -> Self {
        self.params.insert(
            "deviceid".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deviceid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deviceid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicecount(mut self, val: i32) -> Self {
        self.params.insert(
            "devicecount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_devicecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "devicecount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cleanauxiliary(mut self, val: TopMlTrainoidnCleanauxiliary) -> Self {
        self.params.insert(
            "cleanauxiliary".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cleanauxiliary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cleanauxiliary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deterministic(mut self, val: TopMlTrainoidnDeterministic) -> Self {
        self.params.insert(
            "deterministic".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deterministic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deterministic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopMlTrainoidnPythonbin) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlTrainoidnPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopMlTrainoidnPdgWorkitempriority) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputdirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "inputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trainingdataset(mut self, val: &str) -> Self {
        self.params.insert(
            "trainingdataset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trainingdataset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trainingdataset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_validationdataset(mut self, val: &str) -> Self {
        self.params.insert(
            "validationdataset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_validationdataset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validationdataset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputdirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "outputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputname(mut self, val: &str) -> Self {
        self.params.insert(
            "outputname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loss(mut self, val: &str) -> Self {
        self.params.insert(
            "loss".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loss".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_precision(mut self, val: &str) -> Self {
        self.params.insert(
            "precision".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "precision".to_string(),
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
    pub fn with_inputfeatures(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfeatures".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputfeatures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfeatures".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transfer(mut self, val: &str) -> Self {
        self.params.insert(
            "transfer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transfer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transfer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportdirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "exportdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportdirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportfile(mut self, val: &str) -> Self {
        self.params.insert(
            "exportfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicename(mut self, val: &str) -> Self {
        self.params.insert(
            "devicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "devicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "python".to_string(),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uselearningrate(mut self, val: bool) -> Self {
        self.params.insert(
            "uselearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselearningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usemaxlearningrate(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxlearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxlearningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxlearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useseed(mut self, val: bool) -> Self {
        self.params.insert(
            "useseed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.params.insert(
            "verbose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetransfer(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransfer".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransfer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransfer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableexport(mut self, val: bool) -> Self {
        self.params.insert(
            "enableexport".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableexport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableexport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useexportepoch(mut self, val: bool) -> Self {
        self.params.insert(
            "useexportepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexportepoch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useexportepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedevicename(mut self, val: bool) -> Self {
        self.params.insert(
            "usedevicename".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedevicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedevicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMlTrainoidn {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ml_trainoidn"
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

pub trait TopMlTrainoidnOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMlTrainoidnOutputs for TopMlTrainoidn {}
impl TopMlTrainoidnOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopMlTrainoidn> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopMlTrainoidnInnerExt {
    fn load_parameters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn partitionbyindex1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pythonscript1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pythonvenv1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopMlTrainoidnInnerExt for houdini_ramen_core::graph::InnerGraph<'a, TopMlTrainoidn> {
    fn load_parameters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("load_parameters")
    }
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn partitionbyindex1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("partitionbyindex1")
    }
    fn pythonscript1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pythonscript1")
    }
    fn pythonvenv1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pythonvenv1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainregressionUniformhiddenactivation {
    Tanh = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainregressionPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainregressionPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone)]
pub struct TopMlTrainregression {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMlTrainregression {
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

    pub fn with_weightdecay(mut self, val: f32) -> Self {
        self.params.insert(
            "weightdecay".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weightdecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightdecay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_learningrate(mut self, val: f32) -> Self {
        self.params.insert(
            "learningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "learningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_betaone(mut self, val: f32) -> Self {
        self.params.insert(
            "betaone".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_betaone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "betaone".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_betatwo(mut self, val: f32) -> Self {
        self.params.insert(
            "betatwo".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_betatwo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "betatwo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adarho(mut self, val: f32) -> Self {
        self.params.insert(
            "adarho".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adarho_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adarho".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sgdmomentum(mut self, val: f32) -> Self {
        self.params.insert(
            "sgdmomentum".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sgdmomentum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sgdmomentum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stepgamma(mut self, val: f32) -> Self {
        self.params.insert(
            "stepgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exponentialgamma(mut self, val: f32) -> Self {
        self.params.insert(
            "exponentialgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exponentialgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exponentialgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endlearningrate(mut self, val: f32) -> Self {
        self.params.insert(
            "endlearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_endlearningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endlearningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniformhiddenlayers(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformhiddenlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformhiddenlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformhiddenlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniformhiddenwidth(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformhiddenwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformhiddenwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformhiddenwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weightinitializationrandomseed(mut self, val: i32) -> Self {
        self.params.insert(
            "weightinitializationrandomseed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_weightinitializationrandomseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightinitializationrandomseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shufflerandomseed(mut self, val: i32) -> Self {
        self.params.insert(
            "shufflerandomseed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shufflerandomseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shufflerandomseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "upperlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_upperlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxepochs(mut self, val: i32) -> Self {
        self.params.insert(
            "maxepochs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxepochs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxepochs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trainingdataproportion(mut self, val: i32) -> Self {
        self.params.insert(
            "trainingdataproportion".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_trainingdataproportion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trainingdataproportion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_validationdataproportion(mut self, val: i32) -> Self {
        self.params.insert(
            "validationdataproportion".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_validationdataproportion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validationdataproportion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_epochsperevaluation(mut self, val: i32) -> Self {
        self.params.insert(
            "epochsperevaluation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_epochsperevaluation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "epochsperevaluation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "maxiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_patience(mut self, val: i32) -> Self {
        self.params.insert(
            "patience".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_patience_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "patience".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lineardecay(mut self, val: i32) -> Self {
        self.params.insert(
            "lineardecay".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lineardecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lineardecay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stepsize(mut self, val: i32) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxbatchsize(mut self, val: i32) -> Self {
        self.params.insert(
            "maxbatchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxbatchsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxbatchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsaveepochsperoutput(mut self, val: i32) -> Self {
        self.params.insert(
            "modelsaveepochsperoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modelsaveepochsperoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelsaveepochsperoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelexportepochsperoutput(mut self, val: i32) -> Self {
        self.params.insert(
            "modelexportepochsperoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modelexportepochsperoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelexportepochsperoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_networkcomposition(mut self, val: i32) -> Self {
        self.params.insert(
            "networkcomposition".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_networkcomposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "networkcomposition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelkind(mut self, val: i32) -> Self {
        self.params.insert(
            "modelkind".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelstandardtype(mut self, val: i32) -> Self {
        self.params.insert(
            "modelstandardtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelstandardtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelstandardtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hiddenlayerformat(mut self, val: i32) -> Self {
        self.params.insert(
            "hiddenlayerformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hiddenlayerformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hiddenlayerformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniformhiddenactivation(
        mut self,
        val: TopMlTrainregressionUniformhiddenactivation,
    ) -> Self {
        self.params.insert(
            "uniformhiddenactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformhiddenactivation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformhiddenactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_losskind(mut self, val: i32) -> Self {
        self.params.insert(
            "losskind".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_losskind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "losskind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lossstandardtype(mut self, val: i32) -> Self {
        self.params.insert(
            "lossstandardtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_lossstandardtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lossstandardtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optimizertype(mut self, val: i32) -> Self {
        self.params.insert(
            "optimizertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_optimizertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "optimizertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_schedulertype(mut self, val: i32) -> Self {
        self.params.insert(
            "schedulertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_schedulertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "schedulertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exponentialstyle(mut self, val: i32) -> Self {
        self.params.insert(
            "exponentialstyle".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_exponentialstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exponentialstyle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsaveevent(mut self, val: i32) -> Self {
        self.params.insert(
            "modelsaveevent".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelsaveevent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelsaveevent".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelexportevent(mut self, val: i32) -> Self {
        self.params.insert(
            "modelexportevent".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelexportevent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelexportevent".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_losslogevent(mut self, val: i32) -> Self {
        self.params.insert(
            "losslogevent".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_losslogevent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "losslogevent".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlTrainregressionPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlTrainregressionPdgWorkitempriority,
    ) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelcreationscript(mut self, val: &str) -> Self {
        self.params.insert(
            "modelcreationscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelcreationscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelcreationscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_losscreationscript(mut self, val: &str) -> Self {
        self.params.insert(
            "losscreationscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_losscreationscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "losscreationscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unifiedcreationscript(mut self, val: &str) -> Self {
        self.params.insert(
            "unifiedcreationscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unifiedcreationscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unifiedcreationscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hyperparameterfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "hyperparameterfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hyperparameterfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hyperparameterfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hyperparameterbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "hyperparameterbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hyperparameterbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hyperparameterbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "datasetfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datasetfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "datasetbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datasetbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datasetbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "modelsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "modelbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logsfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "logsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logsfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logsfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logbasename(mut self, val: &str) -> Self {
        self.params.insert(
            "logbasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_statesfolder(mut self, val: &str) -> Self {
        self.params.insert(
            "statesfolder".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_statesfolder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "statesfolder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_statebasename(mut self, val: &str) -> Self {
        self.params.insert(
            "statebasename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_statebasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "statebasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shuffle(mut self, val: bool) -> Self {
        self.params.insert(
            "shuffle".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shuffle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shuffle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitsize(mut self, val: bool) -> Self {
        self.params.insert(
            "limitsize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitepochs(mut self, val: bool) -> Self {
        self.params.insert(
            "limitepochs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitepochs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitepochs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablevalidation(mut self, val: bool) -> Self {
        self.params.insert(
            "enablevalidation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevalidation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablevalidation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableearlystopping(mut self, val: bool) -> Self {
        self.params.insert(
            "enableearlystopping".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableearlystopping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableearlystopping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelsaveappendepoch(mut self, val: bool) -> Self {
        self.params.insert(
            "modelsaveappendepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_modelsaveappendepoch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelsaveappendepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelexportappendepoch(mut self, val: bool) -> Self {
        self.params.insert(
            "modelexportappendepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_modelexportappendepoch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelexportappendepoch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_logtostandardoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "logtostandardoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_logtostandardoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logtostandardoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usecpuexclusively(mut self, val: bool) -> Self {
        self.params.insert(
            "usecpuexclusively".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecpuexclusively_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecpuexclusively".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMlTrainregression {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ml_trainregression"
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

pub trait TopMlTrainregressionOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMlTrainregressionOutputs for TopMlTrainregression {}
impl TopMlTrainregressionOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<TopMlTrainregression>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferInputtype {
    SinglePairedImage = 0,
    MultiplePairedImages = 1,
    MultipleUnpairedImages = 2,
    CompositePairedImages = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferInputstorage {
    Automatic = 0,
    KeepOnTrainingDevice = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferTesttype {
    SinglePairedImage = 0,
    MultiplePairedImages = 1,
    MultipleUnpairedImages = 2,
    CompositePairedImages = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferTeststorage {
    Automatic = 0,
    KeepOnTrainingDevice = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopMlTrainstyletransfer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopMlTrainstyletransfer {
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

    pub fn with_randomcropping(mut self, val: f32) -> Self {
        self.params.insert(
            "randomcropping".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomcropping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomcropping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalflip(mut self, val: f32) -> Self {
        self.params.insert(
            "horizontalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalflip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalflip(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalflip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_learningrate(mut self, val: f32) -> Self {
        self.params.insert(
            "learningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "learningrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stepgamma(mut self, val: f32) -> Self {
        self.params.insert(
            "stepgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initializergain(mut self, val: f32) -> Self {
        self.params.insert(
            "initializergain".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initializergain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initializergain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adarho(mut self, val: f32) -> Self {
        self.params.insert(
            "adarho".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adarho_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adarho".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sgdmomentum(mut self, val: f32) -> Self {
        self.params.insert(
            "sgdmomentum".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sgdmomentum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sgdmomentum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatorlambda(mut self, val: f32) -> Self {
        self.params.insert(
            "generatorlambda".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_generatorlambda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatorlambda".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testssim(mut self, val: f32) -> Self {
        self.params.insert(
            "testssim".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_testssim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testssim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adambeta(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "adambeta".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_adambeta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adambeta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_imagechannels(mut self, val: i32) -> Self {
        self.params.insert(
            "imagechannels".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imagechannels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagechannels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_imagesize(mut self, val: i32) -> Self {
        self.params.insert(
            "imagesize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imagesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "inputthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_inputthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputbatchsize(mut self, val: i32) -> Self {
        self.params.insert(
            "inputbatchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_inputbatchsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputbatchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maximages(mut self, val: i32) -> Self {
        self.params.insert(
            "maximages".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maximages_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maximages".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "maxiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lineardecay(mut self, val: i32) -> Self {
        self.params.insert(
            "lineardecay".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lineardecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lineardecay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stepsize(mut self, val: i32) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatorlayers(mut self, val: i32) -> Self {
        self.params.insert(
            "generatorlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_generatorlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatorlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatorinitial(mut self, val: i32) -> Self {
        self.params.insert(
            "generatorinitial".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_generatorinitial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatorinitial".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatorkernel(mut self, val: i32) -> Self {
        self.params.insert(
            "generatorkernel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_generatorkernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatorkernel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_discriminatorlayers(mut self, val: i32) -> Self {
        self.params.insert(
            "discriminatorlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_discriminatorlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "discriminatorlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_discriminatorinitial(mut self, val: i32) -> Self {
        self.params.insert(
            "discriminatorinitial".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_discriminatorinitial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "discriminatorinitial".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_discriminatorkernel(mut self, val: i32) -> Self {
        self.params.insert(
            "discriminatorkernel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_discriminatorkernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "discriminatorkernel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "testthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testbatchsize(mut self, val: i32) -> Self {
        self.params.insert(
            "testbatchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testbatchsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testbatchsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testrate(mut self, val: i32) -> Self {
        self.params.insert(
            "testrate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testcount(mut self, val: i32) -> Self {
        self.params.insert(
            "testcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelrate(mut self, val: i32) -> Self {
        self.params.insert(
            "modelrate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modelrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_onnxrate(mut self, val: i32) -> Self {
        self.params.insert(
            "onnxrate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_onnxrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onnxrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_onnxversion(mut self, val: i32) -> Self {
        self.params.insert(
            "onnxversion".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_onnxversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onnxversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_plotrate(mut self, val: i32) -> Self {
        self.params.insert(
            "plotrate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_plotrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plotrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cputhreads(mut self, val: i32) -> Self {
        self.params.insert(
            "cputhreads".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cputhreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cputhreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputtype(mut self, val: TopMlTrainstyletransferInputtype) -> Self {
        self.params.insert(
            "inputtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputstorage(mut self, val: TopMlTrainstyletransferInputstorage) -> Self {
        self.params.insert(
            "inputstorage".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputstorage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputstorage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_schedulertype(mut self, val: i32) -> Self {
        self.params.insert(
            "schedulertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_schedulertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "schedulertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initializertype(mut self, val: i32) -> Self {
        self.params.insert(
            "initializertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initializertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initializertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optimizertype(mut self, val: i32) -> Self {
        self.params.insert(
            "optimizertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_optimizertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "optimizertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatordownactivation(mut self, val: i32) -> Self {
        self.params.insert(
            "generatordownactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_generatordownactivation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatordownactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatorupactivation(mut self, val: i32) -> Self {
        self.params.insert(
            "generatorupactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_generatorupactivation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatorupactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_discriminatoractivation(mut self, val: i32) -> Self {
        self.params.insert(
            "discriminatoractivation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_discriminatoractivation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "discriminatoractivation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testtype(mut self, val: TopMlTrainstyletransferTesttype) -> Self {
        self.params.insert(
            "testtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_testtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_teststorage(mut self, val: TopMlTrainstyletransferTeststorage) -> Self {
        self.params.insert(
            "teststorage".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_teststorage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "teststorage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_plottype(mut self, val: i32) -> Self {
        self.params.insert(
            "plottype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_plottype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plottype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopMlTrainstyletransferPythonbin) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlTrainstyletransferPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlTrainstyletransferPdgWorkitempriority,
    ) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singleinput(mut self, val: &str) -> Self {
        self.params.insert(
            "singleinput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singleinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singlereference(mut self, val: &str) -> Self {
        self.params.insert(
            "singlereference".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singlereference_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singlereference".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputdir(mut self, val: &str) -> Self {
        self.params.insert(
            "inputdir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_referencedir(mut self, val: &str) -> Self {
        self.params.insert(
            "referencedir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_referencedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "referencedir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_compositedir(mut self, val: &str) -> Self {
        self.params.insert(
            "compositedir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_compositedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compositedir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "testoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singletestinput(mut self, val: &str) -> Self {
        self.params.insert(
            "singletestinput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singletestinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singletestinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singletestreference(mut self, val: &str) -> Self {
        self.params.insert(
            "singletestreference".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singletestreference_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singletestreference".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testinputdir(mut self, val: &str) -> Self {
        self.params.insert(
            "testinputdir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testinputdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testinputdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testreferencedir(mut self, val: &str) -> Self {
        self.params.insert(
            "testreferencedir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testreferencedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testreferencedir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testcompositedir(mut self, val: &str) -> Self {
        self.params.insert(
            "testcompositedir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testcompositedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testcompositedir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelformat(mut self, val: &str) -> Self {
        self.params.insert(
            "modelformat".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modelformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_onnxformat(mut self, val: &str) -> Self {
        self.params.insert(
            "onnxformat".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_onnxformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onnxformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ssimplot(mut self, val: &str) -> Self {
        self.params.insert(
            "ssimplot".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ssimplot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssimplot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lossplot(mut self, val: &str) -> Self {
        self.params.insert(
            "lossplot".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lossplot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lossplot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scoreplot(mut self, val: &str) -> Self {
        self.params.insert(
            "scoreplot".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scoreplot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scoreplot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicename(mut self, val: &str) -> Self {
        self.params.insert(
            "devicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_devicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "devicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "python".to_string(),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usemaximages(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaximages".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaximages_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaximages".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_userandomcropping(mut self, val: bool) -> Self {
        self.params.insert(
            "userandomcropping".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userandomcropping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userandomcropping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usehorizontalflip(mut self, val: bool) -> Self {
        self.params.insert(
            "usehorizontalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usehorizontalflip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usehorizontalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useverticalflip(mut self, val: bool) -> Self {
        self.params.insert(
            "useverticalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useverticalflip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useverticalflip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_userotation(mut self, val: bool) -> Self {
        self.params.insert(
            "userotation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_imageshuffle(mut self, val: bool) -> Self {
        self.params.insert(
            "imageshuffle".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_imageshuffle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imageshuffle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_testmodel(mut self, val: bool) -> Self {
        self.params.insert(
            "testmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_testmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "testmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetestssim(mut self, val: bool) -> Self {
        self.params.insert(
            "usetestssim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetestssim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetestssim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addtestoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "addtestoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtestoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addtestoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_writemodels(mut self, val: bool) -> Self {
        self.params.insert(
            "writemodels".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writemodels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writemodels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addmodeloutput(mut self, val: bool) -> Self {
        self.params.insert(
            "addmodeloutput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addmodeloutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addmodeloutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_writeonnx(mut self, val: bool) -> Self {
        self.params.insert(
            "writeonnx".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writeonnx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writeonnx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_onnxdynamic(mut self, val: bool) -> Self {
        self.params.insert(
            "onnxdynamic".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onnxdynamic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onnxdynamic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addonnxoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "addonnxoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addonnxoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addonnxoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "addattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedevicename(mut self, val: bool) -> Self {
        self.params.insert(
            "usedevicename".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedevicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedevicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usecputhreads(mut self, val: bool) -> Self {
        self.params.insert(
            "usecputhreads".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecputhreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecputhreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopMlTrainstyletransfer {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ml_trainstyletransfer"
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

pub trait TopMlTrainstyletransferOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopMlTrainstyletransferOutputs for TopMlTrainstyletransfer {}
impl TopMlTrainstyletransferOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<TopMlTrainstyletransfer>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopMlTrainstyletransferInnerExt {
    fn load_parameters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn partitionbyindex1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pythonscript1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pythonvenv1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopMlTrainstyletransferInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, TopMlTrainstyletransfer>
{
    fn load_parameters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("load_parameters")
    }
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn partitionbyindex1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("partitionbyindex1")
    }
    fn pythonscript1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pythonscript1")
    }
    fn pythonvenv1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pythonvenv1")
    }
}
