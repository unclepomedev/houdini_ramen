#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFeatherDirection {
    Diamond = 0,
    Square = 1,
    Octagon = 2,
    Custom = 3,
    Circle = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFeatherSweepdir {
    /// -X
    MinusX = 0,
    /// +X
    PlusX = 1,
    /// -Y
    MinusY = 2,
    /// +Y
    PlusY = 3,
    /// -X-Y
    MinusXMinusY = 4,
    /// -X+Y
    MinusXPlusY = 5,
    /// +X-Y
    PlusXMinusY = 6,
    /// +X+Y
    PlusXPlusY = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFeatherDecaymode {
    DistanceForUnitChange = 0,
    ChangePerDistance = 1,
}

#[derive(Debug, Clone)]
pub struct CopFeather {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFeather {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitdist(mut self, val: f32) -> Self {
        self.params.insert(
            "unitdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_decay(mut self, val: f32) -> Self {
        self.params.insert(
            "decay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_decay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "decay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_direction(mut self, val: CopFeatherDirection) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepdir(mut self, val: CopFeatherSweepdir) -> Self {
        self.params.insert(
            "sweepdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sweepdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_decaymode(mut self, val: CopFeatherDecaymode) -> Self {
        self.params.insert(
            "decaymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_decaymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "decaymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_outside(mut self, val: bool) -> Self {
        self.params.insert(
            "outside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_includeborder(mut self, val: bool) -> Self {
        self.params.insert(
            "includeborder".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_includeborder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includeborder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFeather {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "feather"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFetchOutputtype {
    Id = 0,
    Mono = 1,
    Uv = 2,
    Rgb = 3,
    Rgba = 4,
    Geometry = 5,
    IntegerVdb = 6,
    FloatVdb = 7,
    VectorVdb = 8,
    Cable = 9,
}

#[derive(Debug, Clone)]
pub struct CopFetch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFetch {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_setports(mut self) -> Self {
        self.params.insert(
            "setports".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_port_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("port{}", index1),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_port_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("port{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtype_inst(mut self, index1: usize, val: CopFetchOutputtype) -> Self {
        self.params.insert(
            format!("outputtype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputtype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_coppath(mut self, val: &str) -> Self {
        self.params.insert(
            "coppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputlabel_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputlabel{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputlabel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputlabel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFetch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fetch"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFileSource {
    File = 0,
    Clone = 1,
    LiveRender = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFileVideoframemethod {
    StartFrame = 0,
    FrameNumber = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFileBorder {
    Default = 0,
    Constant = 1,
    Clamp = 2,
    Mirror = 3,
    Wrap = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFileMissingdata {
    ReportMessage = 0,
    UseColor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFileColorspace {
    Opencolorio = 0,
    Raw = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFileType {
    Mono = 0,
    Uv = 1,
    Rgb = 2,
    Rgba = 3,
    Id = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFilePrecision {
    Default = 0,
    File = 1,
    /// 8-bit
    N8MinusBit = 2,
    /// 16-bit
    N16MinusBit = 3,
    /// 32-bit
    N32MinusBit = 4,
}

#[derive(Debug, Clone)]
pub struct CopFile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFile {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_liverenderstart(mut self) -> Self {
        self.params.insert(
            "liverenderstart".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_liverenderstop(mut self) -> Self {
        self.params.insert(
            "liverenderstop".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_addaovs(mut self) -> Self {
        self.params.insert(
            "addaovs".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_missingcolor(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "missingcolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_missingcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missingcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_videoframestart(mut self, val: i32) -> Self {
        self.params.insert(
            "videoframestart".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_videoframestart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "videoframestart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_videoframe(mut self, val: i32) -> Self {
        self.params.insert(
            "videoframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_videoframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "videoframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_udim(mut self, val: i32) -> Self {
        self.params
            .insert("udim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_source(mut self, val: CopFileSource) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_videoframemethod(mut self, val: CopFileVideoframemethod) -> Self {
        self.params.insert(
            "videoframemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_videoframemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "videoframemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: CopFileBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingdata(mut self, val: CopFileMissingdata) -> Self {
        self.params.insert(
            "missingdata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missingdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorspace(mut self, val: CopFileColorspace) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_colorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: CopFileType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: CopFilePrecision) -> Self {
        self.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clonename(mut self, val: &str) -> Self {
        self.params.insert(
            "clonename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clonename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clonename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clonesession(mut self, val: &str) -> Self {
        self.params.insert(
            "clonesession".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clonesession_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clonesession".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clonemenu(mut self, val: &str) -> Self {
        self.params.insert(
            "clonemenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clonemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clonemenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_liverenderpath(mut self, val: &str) -> Self {
        self.params.insert(
            "liverenderpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_liverenderpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "liverenderpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("aov{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aov{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_setudim(mut self, val: bool) -> Self {
        self.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_premultiplied(mut self, val: bool) -> Self {
        self.params.insert(
            "premultiplied".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_premultiplied_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "premultiplied".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadcamera(mut self, val: bool) -> Self {
        self.params.insert(
            "loadcamera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raw_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("raw{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_raw_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("raw{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "file"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFillSource {
    Value = 0,
    ImageLocation = 1,
    FirstPixel = 2,
    LastPixel = 3,
}

#[derive(Debug, Clone)]
pub struct CopFill {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFill {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_location(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "location".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_location_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "location".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_color(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_id(mut self, val: i32) -> Self {
        self.params
            .insert("id".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_id_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "id".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_source(mut self, val: CopFillSource) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFill {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fill"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFillconnectedSource {
    Value = 0,
    ImageLocation = 1,
    FirstPixel = 2,
    LastPixel = 3,
}

#[derive(Debug, Clone)]
pub struct CopFillconnected {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFillconnected {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_seedloc(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "seedloc".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_seedloc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seedloc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceloc(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sourceloc".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sourceloc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceloc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_color(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_id(mut self, val: i32) -> Self {
        self.params
            .insert("id".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_id_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "id".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_source(mut self, val: CopFillconnectedSource) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_fallofframp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "fallofframp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_fallofframp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallofframp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFillconnected {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fillconnected"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct CopFlip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFlip {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_xflip(mut self, val: bool) -> Self {
        self.params.insert(
            "xflip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xflip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xflip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yflip(mut self, val: bool) -> Self {
        self.params.insert(
            "yflip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_yflip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yflip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flop(mut self, val: bool) -> Self {
        self.params.insert(
            "flop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFlip {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flip"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct CopFlowBlockBegin {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFlowBlockBegin {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            4,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_4_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_buoyancy(mut self, val: f32) -> Self {
        self.params.insert(
            "buoyancy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_buoyancy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buoyancy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_buoyancydir(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "buoyancydir".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_buoyancydir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buoyancydir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_blockpath(mut self, val: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blockpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advectlist(mut self, val: &str) -> Self {
        self.params.insert(
            "advectlist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_advectlist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "advectlist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_correctvel(mut self, val: bool) -> Self {
        self.params.insert(
            "correctvel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_correctvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "correctvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFlowBlockBegin {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flow_block_begin"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFlowBlockEndBorderx {
    Open = 0,
    Closed = 1,
    Periodic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFlowBlockEndBordery {
    Open = 0,
    Closed = 1,
    Periodic = 2,
}

#[derive(Debug, Clone)]
pub struct CopFlowBlockEnd {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFlowBlockEnd {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            4,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_4_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            5,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            5,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_5_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            5,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            6,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            6,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_6_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            6,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_continuouscook_toggle(mut self) -> Self {
        self.params.insert(
            "continuouscook_toggle".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_continuouscook_tick(mut self, val: i32) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_continuouscook_tick_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachedframes(mut self, val: i32) -> Self {
        self.params.insert(
            "cachedframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachedframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachedframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointframes(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpointframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "collision_iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_collision_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "pressure_iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pressure_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_borderx(mut self, val: CopFlowBlockEndBorderx) -> Self {
        self.params.insert(
            "borderx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bordery(mut self, val: CopFlowBlockEndBordery) -> Self {
        self.params.insert(
            "bordery".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bordery_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bordery".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_simulate(mut self, val: bool) -> Self {
        self.params.insert(
            "simulate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_simulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "simulate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_continuouscook(mut self, val: bool) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuouscook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFlowBlockEnd {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flow_block_end"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFlowProjectnondivergentmultigridBorderxneg {
    Open = 0,
    Closed = 1,
    Periodic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFlowProjectnondivergentmultigridBorderxpos {
    Open = 0,
    Closed = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFlowProjectnondivergentmultigridBorderyneg {
    Open = 0,
    Closed = 1,
    Periodic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFlowProjectnondivergentmultigridBorderypos {
    Open = 0,
    Closed = 1,
}

#[derive(Debug, Clone)]
pub struct CopFlowProjectnondivergentmultigrid {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFlowProjectnondivergentmultigrid {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_coliterations(mut self, val: i32) -> Self {
        self.params.insert(
            "coliterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_coliterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coliterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_borderxneg(mut self, val: CopFlowProjectnondivergentmultigridBorderxneg) -> Self {
        self.params.insert(
            "borderxneg".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderxneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderxneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_borderxpos(mut self, val: CopFlowProjectnondivergentmultigridBorderxpos) -> Self {
        self.params.insert(
            "borderxpos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderxpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderxpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_borderyneg(mut self, val: CopFlowProjectnondivergentmultigridBorderyneg) -> Self {
        self.params.insert(
            "borderyneg".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_borderypos(mut self, val: CopFlowProjectnondivergentmultigridBorderypos) -> Self {
        self.params.insert(
            "borderypos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFlowProjectnondivergentmultigrid {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flow_projectnondivergentmultigrid"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFontOutput {
    Mask = 0,
    Sdf = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFontSmoothunits {
    Image = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFontHalign {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFontValign {
    FirstLine = 0,
    Top = 1,
    Middle = 2,
    Bottom = 3,
}

#[derive(Debug, Clone)]
pub struct CopFont {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFont {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_smooth(mut self, val: f32) -> Self {
        self.params.insert(
            "smooth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smooth_pixel(mut self, val: f32) -> Self {
        self.params.insert(
            "smooth_pixel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smooth_pixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smooth_pixel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fontsize(mut self, val: f32) -> Self {
        self.params.insert(
            "fontsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fontsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fontsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rz(mut self, val: f32) -> Self {
        self.params
            .insert("rz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tracking(mut self, val: f32) -> Self {
        self.params.insert(
            "tracking".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tracking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tracking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_leading(mut self, val: f32) -> Self {
        self.params.insert(
            "leading".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leading_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leading".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oblique(mut self, val: f32) -> Self {
        self.params.insert(
            "oblique".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oblique_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oblique".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lod(mut self, val: f32) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: CopFontOutput) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothunits(mut self, val: CopFontSmoothunits) -> Self {
        self.params.insert(
            "smoothunits".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_smoothunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smoothunits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_halign(mut self, val: CopFontHalign) -> Self {
        self.params.insert(
            "halign".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_halign_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "halign".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valign(mut self, val: CopFontValign) -> Self {
        self.params.insert(
            "valign".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_valign_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valign".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_text(mut self, val: &str) -> Self {
        self.params.insert(
            "text".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_text_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "text".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dosmooth(mut self, val: bool) -> Self {
        self.params.insert(
            "dosmooth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_descender(mut self, val: bool) -> Self {
        self.params.insert(
            "use_descender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_descender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_descender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFont {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "font"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFractalnoiseNoisetype {
    Torus = 0,
    Perlin = 1,
    WorleyCellularF1 = 2,
    /// Worley Cellular F2-F1
    WorleyCellularF2MinusF1 = 3,
    /// White (Random)
    WhiteRandom = 4,
    Alligator = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFractalnoiseMetric {
    Euclidean = 0,
    Manhattan = 1,
    Chebyshev = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFractalnoiseElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopFractalnoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFractalnoise {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            4,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_4_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center(mut self, val: f32) -> Self {
        self.params.insert(
            "center".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pulselength(mut self, val: f32) -> Self {
        self.params.insert(
            "pulselength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pulselength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pulselength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "timeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_looplen(mut self, val: f32) -> Self {
        self.params.insert(
            "looplen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_looplen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "looplen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oct(mut self, val: f32) -> Self {
        self.params.insert(
            "oct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lac(mut self, val: f32) -> Self {
        self.params.insert(
            "lac".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lac".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "post_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_gain(mut self, val: f32) -> Self {
        self.params.insert(
            "post_gain".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_gain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "post_gamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_gamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "post_contrast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_contrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_minimum(mut self, val: f32) -> Self {
        self.params.insert(
            "post_minimum".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_minimum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_minimum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_maximum(mut self, val: f32) -> Self {
        self.params.insert(
            "post_maximum".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_maximum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_maximum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_elementscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "elementscale".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_elementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_off(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "off".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_off_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "off".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tilesize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "tilesize".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_tilesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tilesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_noisetype(mut self, val: CopFractalnoiseNoisetype) -> Self {
        self.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metric(mut self, val: CopFractalnoiseMetric) -> Self {
        self.params.insert(
            "metric".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metric".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementsizetype(mut self, val: CopFractalnoiseElementsizetype) -> Self {
        self.params.insert(
            "elementsizetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_elementsizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsizetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_percomp(mut self, val: bool) -> Self {
        self.params.insert(
            "percomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_percomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "percomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotiled(mut self, val: bool) -> Self {
        self.params.insert(
            "dotiled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dotiled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotiled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use3d(mut self, val: bool) -> Self {
        self.params.insert(
            "use3d".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animate(mut self, val: bool) -> Self {
        self.params.insert(
            "animate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doloop(mut self, val: bool) -> Self {
        self.params.insert(
            "doloop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doloop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doloop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dofold(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dofold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dofold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dofold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_docomplement(mut self, val: bool) -> Self {
        self.params.insert(
            "post_docomplement".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_docomplement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_docomplement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dobias(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dobias".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dobias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dobias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dogain(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dogain".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dogain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dogain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dogamma(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dogamma".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dogamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dogamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_docontrast(mut self, val: bool) -> Self {
        self.params.insert(
            "post_docontrast".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_docontrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_docontrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_doclampmin(mut self, val: bool) -> Self {
        self.params.insert(
            "post_doclampmin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_doclampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_doclampmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_doclampmax(mut self, val: bool) -> Self {
        self.params.insert(
            "post_doclampmax".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_doclampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_doclampmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFractalnoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fractalnoise"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFractalnoise3dNoisetype {
    Simplex = 0,
    Perlin = 1,
    WorleyCellularF1 = 2,
    /// Worley Cellular F2-F1
    WorleyCellularF2MinusF1 = 3,
    /// White (Random)
    WhiteRandom = 4,
    Alligator = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFractalnoise3dMetric {
    Euclidean = 0,
    Manhattan = 1,
    Chebyshev = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFractalnoise3dElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopFractalnoise3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFractalnoise3d {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center(mut self, val: f32) -> Self {
        self.params.insert(
            "center".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oct(mut self, val: f32) -> Self {
        self.params.insert(
            "oct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lac(mut self, val: f32) -> Self {
        self.params.insert(
            "lac".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lac".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "post_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_gain(mut self, val: f32) -> Self {
        self.params.insert(
            "post_gain".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_gain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "post_gamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_gamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "post_contrast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_contrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_minimum(mut self, val: f32) -> Self {
        self.params.insert(
            "post_minimum".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_minimum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_minimum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_maximum(mut self, val: f32) -> Self {
        self.params.insert(
            "post_maximum".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_maximum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_maximum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_elementscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "elementscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_elementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_off(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "off".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_off_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "off".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_noisetype(mut self, val: CopFractalnoise3dNoisetype) -> Self {
        self.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metric(mut self, val: CopFractalnoise3dMetric) -> Self {
        self.params.insert(
            "metric".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metric".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementsizetype(mut self, val: CopFractalnoise3dElementsizetype) -> Self {
        self.params.insert(
            "elementsizetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_elementsizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsizetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_percomp(mut self, val: bool) -> Self {
        self.params.insert(
            "percomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_percomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "percomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dofold(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dofold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dofold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dofold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_docomplement(mut self, val: bool) -> Self {
        self.params.insert(
            "post_docomplement".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_docomplement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_docomplement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dobias(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dobias".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dobias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dobias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dogain(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dogain".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dogain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dogain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_dogamma(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dogamma".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dogamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dogamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_docontrast(mut self, val: bool) -> Self {
        self.params.insert(
            "post_docontrast".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_docontrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_docontrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_doclampmin(mut self, val: bool) -> Self {
        self.params.insert(
            "post_doclampmin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_doclampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_doclampmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_post_doclampmax(mut self, val: bool) -> Self {
        self.params.insert(
            "post_doclampmax".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_doclampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_doclampmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFractalnoise3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fractalnoise3d"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFunctionFunc {
    Arithmetic = 0,
    Powers = 1,
    Trigonometry = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFunctionFuncMath {
    Absolute = 0,
    Sign = 1,
    Normalize = 2,
    Negate = 3,
    Reciprocal = 4,
    Floor = 5,
    Round = 6,
    Ceil = 7,
    Modulus = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFunctionFuncPow {
    SquareRoot = 0,
    /// pow(x, E)
    PowXE = 1,
    /// pow(b, x)
    PowBX = 2,
    /// exp(x)
    ExpX = 3,
    /// Base-Log
    BaseMinusLog = 4,
    NaturalLog = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFunctionFuncTrig {
    Sin = 0,
    Cos = 1,
    Tan = 2,
    ArcSin = 3,
    ArcCos = 4,
    ArcTan = 5,
    HyperbolicSin = 6,
    HyperbolicCos = 7,
    HyperbolicTan = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopFunctionAngleunit {
    Radians = 0,
    Degrees = 1,
    /// Circles (2pi)
    Circles2pi = 2,
}

#[derive(Debug, Clone)]
pub struct CopFunction {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopFunction {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_base(mut self, val: f32) -> Self {
        self.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exponent(mut self, val: f32) -> Self {
        self.params.insert(
            "exponent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modbase(mut self, val: f32) -> Self {
        self.params.insert(
            "modbase".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_modbase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modbase".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_func(mut self, val: CopFunctionFunc) -> Self {
        self.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_func_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func_math(mut self, val: CopFunctionFuncMath) -> Self {
        self.params.insert(
            "func_math".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_func_math_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "func_math".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func_pow(mut self, val: CopFunctionFuncPow) -> Self {
        self.params.insert(
            "func_pow".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_func_pow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "func_pow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func_trig(mut self, val: CopFunctionFuncTrig) -> Self {
        self.params.insert(
            "func_trig".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_func_trig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "func_trig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angleunit(mut self, val: CopFunctionAngleunit) -> Self {
        self.params.insert(
            "angleunit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_angleunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angleunit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopFunction {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "function"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
