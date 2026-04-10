#[derive(Debug, Clone)]
pub struct TopChopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopChopnet {
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

impl houdini_ramen_core::types::HoudiniNode for TopChopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "chopnet"
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
pub struct TopCommandserverend {
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

impl TopCommandserverend {
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

    pub fn with_pdg_partitionmergeoperation_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("pdg_partitionmergeoperation{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_partitionmergeoperation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergeoperation{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackbegin(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_feedbackbegin".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackbegin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackbegin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_partitionmergepattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepattern{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_partitionmergepattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepattern{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_partitionmergepreserve_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepreserve{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionmergepreserve_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepreserve{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopCommandserverend {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "commandserverend"
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

pub trait TopCommandserverendOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopCommandserverendOutputs for TopCommandserverend {}
impl TopCommandserverendOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<TopCommandserverend>
{
}

#[derive(Debug, Clone)]
pub struct TopCop2net {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopCop2net {
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

impl houdini_ramen_core::types::HoudiniNode for TopCop2net {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "cop2net"
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
pub enum TopCopnetBorder {
    Constant = 0,
    Clamp = 1,
    Mirror = 2,
    Wrap = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCopnetPrecision {
    /// 16-bit
    N16MinusBit = 0,
    /// 32-bit
    N32MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct TopCopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopCopnet {
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

    pub fn with_pixelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_udim(mut self, val: i32) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vistile(mut self, val: i32) -> Self {
        self.params.insert(
            "vistile".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vistile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_border(mut self, val: TopCopnetBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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
    pub fn with_precision(mut self, val: TopCopnetPrecision) -> Self {
        self.params.insert(
            "precision".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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
    pub fn with_setres(mut self, val: bool) -> Self {
        self.params.insert(
            "setres".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setpixelscale(mut self, val: bool) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setpixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setborder(mut self, val: bool) -> Self {
        self.params.insert(
            "setborder".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setborder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setborder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setprecision(mut self, val: bool) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setudim(mut self, val: bool) -> Self {
        self.params.insert(
            "setudim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setudim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setvistile(mut self, val: bool) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopCopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "copnet"
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
pub enum TopCsvconcatConcatrows {
    ByColumn = 0,
    ByRow = 1,
}

#[derive(Debug, Clone)]
pub struct TopCsvconcat {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopCsvconcat {
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

    pub fn with_concatrows(mut self, val: TopCsvconcatConcatrows) -> Self {
        self.params.insert(
            "concatrows".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_concatrows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "concatrows".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefix2column(mut self, val: &str) -> Self {
        self.params.insert(
            "prefix2column".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefix2column_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefix2column".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extracts(mut self, val: &str) -> Self {
        self.params.insert(
            "extracts".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extracts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extracts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hasheader(mut self, val: bool) -> Self {
        self.params.insert(
            "hasheader".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasheader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasheader".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopCsvconcat {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "csvconcat"
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

pub trait TopCsvconcatOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopCsvconcatOutputs for TopCsvconcat {}
impl TopCsvconcatOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopCsvconcat> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvinputUseparent {
    CustomFilePath = 0,
    UpstreamOutputFiles = 1,
    CustomString = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvinputIndexmode {
    RowIndex = 0,
    UpstreamIndex = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvinputExtractall {
    ByColumnIndex = 0,
    All = 1,
    ByColumnName = 2,
    ByAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvinputExtracttype {
    String = 0,
    Integer = 1,
    Float = 2,
}

#[derive(Debug, Clone)]
pub struct TopCsvinput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopCsvinput {
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

    pub fn with_limitrowcount(mut self, val: i32) -> Self {
        self.params.insert(
            "limitrowcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_limitrowcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitrowcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractrow_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("extractrow{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_extractrow_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("extractrow{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("extractindex{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_extractindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("extractindex{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extracttype_inst(mut self, index1: usize, val: TopCsvinputExtracttype) -> Self {
        self.params.insert(
            format!("extracttype{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_extracttype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("extracttype{}", index1),
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
    pub fn with_useparent(mut self, val: TopCsvinputUseparent) -> Self {
        self.params.insert(
            "useparent".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_useparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useparent".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indexmode(mut self, val: TopCsvinputIndexmode) -> Self {
        self.params.insert(
            "indexmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_indexmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractall(mut self, val: TopCsvinputExtractall) -> Self {
        self.params.insert(
            "extractall".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extractall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extractall".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useparenttag(mut self, val: &str) -> Self {
        self.params.insert(
            "useparenttag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useparenttag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useparenttag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_csvfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "csvfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_csvfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "csvfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_csvstring(mut self, val: &str) -> Self {
        self.params.insert(
            "csvstring".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_csvstring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "csvstring".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_delimiter(mut self, val: &str) -> Self {
        self.params.insert(
            "delimiter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_delimiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delimiter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_columnorderattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "columnorderattrname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_columnorderattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "columnorderattrname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractbyattrattr(mut self, val: &str) -> Self {
        self.params.insert(
            "extractbyattrattr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractbyattrattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extractbyattrattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractcolumnname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("extractcolumnname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extractcolumnname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("extractcolumnname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extracttag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("extracttag{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extracttag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("extracttag{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hasheaderrow(mut self, val: bool) -> Self {
        self.params.insert(
            "hasheaderrow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasheaderrow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasheaderrow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_escapedoublequotes(mut self, val: bool) -> Self {
        self.params.insert(
            "escapedoublequotes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_escapedoublequotes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "escapedoublequotes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifycolumndelimiter(mut self, val: bool) -> Self {
        self.params.insert(
            "specifycolumndelimiter".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_specifycolumndelimiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifycolumndelimiter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitrows(mut self, val: bool) -> Self {
        self.params.insert(
            "limitrows".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitrows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitrows".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addcolumnorderattr(mut self, val: bool) -> Self {
        self.params.insert(
            "addcolumnorderattr".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addcolumnorderattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addcolumnorderattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopCsvinput {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "csvinput"
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

pub trait TopCsvinputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopCsvinputOutputs for TopCsvinput {}
impl TopCsvinputOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopCsvinput> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvmodifyOperation {
    InsertColumn = 0,
    DeleteColumn = 1,
    IsolateColumn = 2,
    SortRows = 3,
    SplitFileSequencePath = 4,
    JoinFileSequencePath = 5,
    ReplaceColumn = 6,
    ReplaceRows = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvmodifyExprOp {
    Custom = 0,
    /// Find/Replace
    FindReplace = 1,
    IsolateSequence = 2,
}

#[derive(Debug, Clone)]
pub struct TopCsvmodify {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopCsvmodify {
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

    pub fn trigger_generateexpr(mut self) -> Self {
        self.params.insert(
            "generateexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_operation(mut self, val: TopCsvmodifyOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_operation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_columnindex(mut self, val: i32) -> Self {
        self.params.insert(
            "columnindex".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_columnindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "columnindex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expr_op(mut self, val: TopCsvmodifyExprOp) -> Self {
        self.params.insert(
            "expr_op".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_expr_op_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expr_op".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_findcount(mut self, val: i32) -> Self {
        self.params.insert(
            "findcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_findcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_findcol(mut self, val: i32) -> Self {
        self.params.insert(
            "findcol".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_findcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findcol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_isolatecol(mut self, val: i32) -> Self {
        self.params.insert(
            "isolatecol".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_isolatecol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isolatecol".to_string(),
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
    pub fn with_findstring(mut self, val: &str) -> Self {
        self.params.insert(
            "findstring".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_findstring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findstring".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_replacestring(mut self, val: &str) -> Self {
        self.params.insert(
            "replacestring".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_replacestring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "replacestring".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_isolateprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "isolateprefix".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_isolateprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isolateprefix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expr(mut self, val: &str) -> Self {
        self.params.insert(
            "expr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_findinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "findinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_findinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_findbefore(mut self, val: bool) -> Self {
        self.params.insert(
            "findbefore".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_findbefore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findbefore".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_findafter(mut self, val: bool) -> Self {
        self.params.insert(
            "findafter".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_findafter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findafter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_isolateinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "isolateinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_isolateinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isolateinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopCsvmodify {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "csvmodify"
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

pub trait TopCsvmodifyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopCsvmodifyOutputs for TopCsvmodify {}
impl TopCsvmodifyOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopCsvmodify> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvoutputQuoting {
    Minimal = 0,
    NonNumeric = 1,
    All = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvoutputQuerymode {
    ByDataQueries = 0,
    ByColumnAttribute = 1,
    ByAttributePattern = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvoutputErrorifnomatch {
    None = 0,
    ReportError = 1,
    ReportWarning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopCsvoutputMultiplevalueshandling {
    AddRows = 0,
    AddColumns = 1,
}

#[derive(Debug, Clone)]
pub struct TopCsvoutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopCsvoutput {
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
    pub fn with_quoting(mut self, val: TopCsvoutputQuoting) -> Self {
        self.params.insert(
            "quoting".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quoting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quoting".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_querymode(mut self, val: TopCsvoutputQuerymode) -> Self {
        self.params.insert(
            "querymode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_querymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "querymode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_errorifnomatch(mut self, val: TopCsvoutputErrorifnomatch) -> Self {
        self.params.insert(
            "errorifnomatch".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_errorifnomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorifnomatch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_multiplevalueshandling(mut self, val: TopCsvoutputMultiplevalueshandling) -> Self {
        self.params.insert(
            "multiplevalueshandling".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_multiplevalueshandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multiplevalueshandling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_csvfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "csvfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_csvfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "csvfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resultdatatag(mut self, val: &str) -> Self {
        self.params.insert(
            "resultdatatag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resultdatatag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resultdatatag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_delimiter(mut self, val: &str) -> Self {
        self.params.insert(
            "delimiter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_delimiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delimiter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_queriesattr(mut self, val: &str) -> Self {
        self.params.insert(
            "queriesattr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_queriesattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "queriesattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pattern(mut self, val: &str) -> Self {
        self.params.insert(
            "pattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataqueriesmult_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dataqueriesmult{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataqueriesmult_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dataqueriesmult{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataqueriesoverride_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dataqueriesoverride{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataqueriesoverride_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dataqueriesoverride{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablecsvfile(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecsvfile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecsvfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecsvfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifycolumndelimiter(mut self, val: bool) -> Self {
        self.params.insert(
            "specifycolumndelimiter".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_specifycolumndelimiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifycolumndelimiter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_writeheader(mut self, val: bool) -> Self {
        self.params.insert(
            "writeheader".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writeheader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writeheader".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedataqueriesoverride_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usedataqueriesoverride{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedataqueriesoverride_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usedataqueriesoverride{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopCsvoutput {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "csvoutput"
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

pub trait TopCsvoutputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopCsvoutputOutputs for TopCsvoutput {}
impl TopCsvoutputOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopCsvoutput> {}
