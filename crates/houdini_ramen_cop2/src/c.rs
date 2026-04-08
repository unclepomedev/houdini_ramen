#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChannelcopyNewplane {
    /// C (rgb)
    CRgb = 0,
    A = 1,
    /// A (rgb)
    ARgb = 2,
    M = 3,
    /// M (rgb)
    MRgb = 4,
    Pz = 5,
    L = 6,
    /// B (uv)
    BUv = 7,
    /// P (xyz)
    PXyz = 8,
    /// N (xyz)
    NXyz = 9,
    /// V (xyz)
    VXyz = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChannelcopyCopyop {
    Luminance = 0,
    NtscLuminance = 1,
    Average = 2,
    MaximumComponent = 3,
    MinimumComponent = 4,
    Magnitude = 5,
    Hue = 6,
    Saturation = 7,
    Red = 8,
    Green = 9,
    Blue = 10,
    Component4 = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChannelcopyMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone)]
pub struct Cop2Channelcopy {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Channelcopy {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Input 3"
    pub fn set_input_input_3<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Input 3" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_color_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("color{}", index1),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_newplane_inst(mut self, index1: usize, val: Cop2ChannelcopyNewplane) -> Self {
        self.params.insert(
            format!("newplane{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_newplane_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("newplane{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyop_inst(mut self, index1: usize, val: Cop2ChannelcopyCopyop) -> Self {
        self.params.insert(
            format!("copyop{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyop_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("copyop{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ChannelcopyMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_copyto_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("copyto{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyto_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("copyto{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyfrom_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("copyfrom{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyfrom_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("copyfrom{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Channelcopy {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "channelcopy"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct Cop2Chopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Chopnet {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Chopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "chopnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ChromakeyRollfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Gaussian = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChromakeyMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChromakeyFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChromakeyFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ChromakeyFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Chromakey {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Chromakey {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Key"
    pub fn set_input_image_to_key<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Key" and specifies the output index of the target node.
    pub fn set_input_image_to_key_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_hrolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "hrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hrolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "srolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lrolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "lrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lrolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_lum(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "lum".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_huesat(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "huesat".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_huesat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "huesat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_subsample(mut self, val: i32) -> Self {
        self.params.insert(
            "subsample".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_subsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsample".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_rollfunc(mut self, val: Cop2ChromakeyRollfunc) -> Self {
        self.params.insert(
            "rollfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rollfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ChromakeyMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ChromakeyFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ChromakeyFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ChromakeyFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_keyplane(mut self, val: &str) -> Self {
        self.params.insert(
            "keyplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keyplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keyplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_discardkey(mut self, val: bool) -> Self {
        self.params.insert(
            "discardkey".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_discardkey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "discardkey".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addalpha(mut self, val: bool) -> Self {
        self.params.insert(
            "addalpha".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addalpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Chromakey {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "chromakey"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ColorPlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorPreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorPostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Color {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Color {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Add To"
    pub fn set_input_image_to_add_to<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Add To" and specifies the output index of the target node.
    pub fn set_input_image_to_add_to_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_comp4(mut self, val: f32) -> Self {
        self.params.insert(
            "comp4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_comp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scalar(mut self, val: f32) -> Self {
        self.params.insert(
            "scalar".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalar".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_color(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "color".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
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

    // --- Int parameters ---
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
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
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
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
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2ColorPlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2ColorAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2ColorAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2ColorDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2ColorInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2ColorIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2ColorPreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2ColorPostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Color {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "color"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ColorcorrectShowarea {
    AllRanges = 0,
    Shadow = 1,
    Midtone = 2,
    Highlight = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcorrectQuantize {
    WhereOptimal = 0,
    QuantizeAtThisNode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcorrectMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcorrectFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcorrectFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcorrectFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Colorcorrect {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Colorcorrect {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Correct"
    pub fn set_input_image_to_correct<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Correct" and specifies the output index of the target node.
    pub fn set_input_image_to_correct_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_showcurves(mut self) -> Self {
        self.params.insert(
            "showcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_add(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "add".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_add_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "add".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mult".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_gamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
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
    pub fn with_contrast(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "contrast".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
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
    pub fn with_sadd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sadd".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sadd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sadd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_smult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "smult".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_smult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sgamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scontrast(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scontrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_madd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "madd".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_madd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "madd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mmult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mmult".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mmult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mmult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mgamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mcontrast(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mcontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mcontrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mcontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hadd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hadd".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hadd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hadd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hmult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hmult".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hmult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hmult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hgamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hcontrast(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hcontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hcontrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hcontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_showarea(mut self, val: Cop2ColorcorrectShowarea) -> Self {
        self.params.insert(
            "showarea".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_showarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showarea".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quantize(mut self, val: Cop2ColorcorrectQuantize) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ColorcorrectMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ColorcorrectFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ColorcorrectFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ColorcorrectFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dounpremult(mut self, val: bool) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dounpremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Colorcorrect {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colorcorrect"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ColorcurveExtend {
    HoldLastValue = 0,
    ContinueAlongLastSlope = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcurveQuantize {
    WhereOptimal = 0,
    QuantizeAtThisNode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcurveMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcurveFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcurveFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorcurveFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Colorcurve {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Colorcurve {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to ColorCurve"
    pub fn set_input_image_to_colorcurve<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to ColorCurve" and specifies the output index of the target node.
    pub fn set_input_image_to_colorcurve_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_edit(mut self) -> Self {
        self.params.insert(
            "edit".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_curvepnt_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("curvepnt{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_curvepnt_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("curvepnt{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_redpnt_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("redpnt{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_redpnt_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("redpnt{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_greenpnt_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("greenpnt{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_greenpnt_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("greenpnt{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bluepnt_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("bluepnt{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bluepnt_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bluepnt{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alphapnt_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("alphapnt{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_alphapnt_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("alphapnt{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_extend(mut self, val: Cop2ColorcurveExtend) -> Self {
        self.params.insert(
            "extend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quantize(mut self, val: Cop2ColorcurveQuantize) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ColorcurveMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ColorcurveFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ColorcurveFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ColorcurveFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dounpremult(mut self, val: bool) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dounpremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Colorcurve {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colorcurve"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ColormapQuantize {
    WhereOptimal = 0,
    QuantizeAtThisNode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColormapMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColormapFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColormapFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColormapFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Colormap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Colormap {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to ColorMap"
    pub fn set_input_image_to_colormap<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to ColorMap" and specifies the output index of the target node.
    pub fn set_input_image_to_colormap_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_show(mut self) -> Self {
        self.params.insert(
            "show".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_from(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "from".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "from".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_to(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "to".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_to_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "to".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_redfrom(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "redfrom".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_redfrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "redfrom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_redto(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "redto".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_redto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "redto".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_greenfrom(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "greenfrom".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_greenfrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "greenfrom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_greento(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "greento".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_greento_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "greento".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bluefrom(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bluefrom".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bluefrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bluefrom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blueto(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "blueto".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_blueto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blueto".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alphafrom(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "alphafrom".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_alphafrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alphafrom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alphato(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "alphato".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_alphato_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alphato".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_quantize(mut self, val: Cop2ColormapQuantize) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ColormapMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ColormapFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ColormapFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ColormapFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dounpremult(mut self, val: bool) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dounpremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Colormap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colormap"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ColorreplaceRollfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Gaussian = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorreplaceOperation {
    FullReplace = 0,
    ReplaceOnlyHue = 1,
    ReplaceHueAndSaturation = 2,
    ReplaceHueAndLuminance = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorreplaceQuantize {
    WhereOptimal = 0,
    QuantizeAtThisNode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorreplaceMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorreplaceFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorreplaceFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorreplaceFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Colorreplace {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Colorreplace {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image Input"
    pub fn set_input_image_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image Input" and specifies the output index of the target node.
    pub fn set_input_image_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_hrolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "hrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hrolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "srolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lrolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "lrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lrolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lrolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_srclum(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "srclum".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_srclum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srclum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_targetclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "targetclr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_targetclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetclr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_srchuesat(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "srchuesat".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_srchuesat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srchuesat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_supersample(mut self, val: i32) -> Self {
        self.params.insert(
            "supersample".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_supersample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "supersample".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_rollfunc(mut self, val: Cop2ColorreplaceRollfunc) -> Self {
        self.params.insert(
            "rollfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rollfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_operation(mut self, val: Cop2ColorreplaceOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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
    pub fn with_quantize(mut self, val: Cop2ColorreplaceQuantize) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ColorreplaceMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ColorreplaceFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ColorreplaceFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ColorreplaceFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dounpremult(mut self, val: bool) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dounpremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Colorreplace {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colorreplace"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ColorwheelPlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelPreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ColorwheelPostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Colorwheel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Colorwheel {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.params.insert(
            "vex_edit".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.params.insert(
            "vex_reload".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
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
    pub fn with_satcenter(mut self, val: f32) -> Self {
        self.params.insert(
            "satCenter".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_satcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "satCenter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_satedge(mut self, val: f32) -> Self {
        self.params.insert(
            "satEdge".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_satedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "satEdge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_valcenter(mut self, val: f32) -> Self {
        self.params.insert(
            "valCenter".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_valcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valCenter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_valedge(mut self, val: f32) -> Self {
        self.params.insert(
            "valEdge".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_valedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valEdge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
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
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
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
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2ColorwheelPlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2ColorwheelAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2ColorwheelAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2ColorwheelDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2ColorwheelInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2ColorwheelIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2ColorwheelPreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2ColorwheelPostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Colorwheel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "colorwheel"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2CompositeOp {
    Over = 0,
    Under = 1,
    Atop = 2,
    Inside = 3,
    Outside = 4,
    Screen = 5,
    Add = 6,
    Subtract = 7,
    Diff = 8,
    Multiply = 9,
    Minimum = 10,
    Maximum = 11,
    Average = 12,
    ExclusiveOr = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeAlphaop {
    SameAsAbove = 0,
    Over = 1,
    Under = 2,
    Atop = 3,
    Inside = 4,
    Outside = 5,
    Screen = 6,
    Add = 7,
    Subtract = 8,
    Diff = 9,
    Multiply = 10,
    Minimum = 11,
    Maximum = 12,
    Average = 13,
    ExclusiveOr = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeWrap {
    Repeat = 0,
    Clamp = 1,
    Black = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeMtype {
    Velocity = 0,
    Deformation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositePlanemerge {
    MergeAllPlanes = 0,
    OnlyKeepCommonPlanes = 1,
    /// Only Keep First Input's Planes
    OnlyKeepFirstInputSPlanes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeDepthmatch {
    PromoteToHighestBitDepth = 0,
    DemoteToLowestBitDepth = 1,
    /// Use the First Input's Bit Depth
    UseTheFirstInputSBitDepth = 2,
    ErrorIfBitDepthsDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeRangematch {
    /// Trim To The First Input's Range
    TrimToTheFirstInputSRange = 0,
    /// Shift To The First Input's Range
    ShiftToTheFirstInputSRange = 1,
    ExtendSequenceToMaximumRange = 2,
    TrimToSmallestRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeRatematch {
    /// Use The First Input's Frame Rate
    UseTheFirstInputSFrameRate = 0,
    UseTheHighestFrameRate = 1,
    UseTheLowestFrameRate = 2,
    ErrorIfTheFrameRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeFramematch {
    UseNearestFrame = 0,
    UseTheClosestPreviousFrame = 1,
    UseTheClosestNextFrame = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CompositeFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Composite {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Composite {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Foreground"
    pub fn set_input_foreground<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Foreground" and specifies the output index of the target node.
    pub fn set_input_foreground_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Background"
    pub fn set_input_background<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Background" and specifies the output index of the target node.
    pub fn set_input_background_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rz(mut self, val: f32) -> Self {
        self.params.insert(
            "rz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fgweight(mut self, val: f32) -> Self {
        self.params.insert(
            "fgweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fgweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bgweight(mut self, val: f32) -> Self {
        self.params.insert(
            "bgweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bgweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mblur(mut self, val: f32) -> Self {
        self.params.insert(
            "mblur".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mbias(mut self, val: f32) -> Self {
        self.params.insert(
            "mbias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_mseg(mut self, val: i32) -> Self {
        self.params.insert(
            "mseg".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mseg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mseg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_op(mut self, val: Cop2CompositeOp) -> Self {
        self.params.insert(
            "op".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "op".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alphaop(mut self, val: Cop2CompositeAlphaop) -> Self {
        self.params.insert(
            "alphaop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_alphaop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alphaop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: Cop2CompositeUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap(mut self, val: Cop2CompositeWrap) -> Self {
        self.params.insert(
            "wrap".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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
    pub fn with_mtype(mut self, val: Cop2CompositeMtype) -> Self {
        self.params.insert(
            "mtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_planemerge(mut self, val: Cop2CompositePlanemerge) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planemerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthmatch(mut self, val: Cop2CompositeDepthmatch) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depthmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rangematch(mut self, val: Cop2CompositeRangematch) -> Self {
        self.params.insert(
            "rangematch".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rangematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangematch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ratematch(mut self, val: Cop2CompositeRatematch) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ratematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_framematch(mut self, val: Cop2CompositeFramematch) -> Self {
        self.params.insert(
            "framematch".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_framematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framematch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2CompositeMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2CompositeFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2CompositeFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2CompositeFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useblur(mut self, val: bool) -> Self {
        self.params.insert(
            "useblur".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Composite {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "composite"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ContrastQuantize {
    WhereOptimal = 0,
    QuantizeAtThisNode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ContrastMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ContrastFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ContrastFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ContrastFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Contrast {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Contrast {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Contrast"
    pub fn set_input_image_to_contrast<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Contrast" and specifies the output index of the target node.
    pub fn set_input_image_to_contrast_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_showcurves(mut self) -> Self {
        self.params.insert(
            "showcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
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
    pub fn with_center(mut self, val: f32) -> Self {
        self.params.insert(
            "center".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_global(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "global".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_global_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "global".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_red(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "red".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_red_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "red".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_green(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "green".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_green_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "green".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blue(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "blue".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_blue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alpha(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_comp4(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "comp4".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_comp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cred(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "cred".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_cred_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cred".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cgreen(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "cgreen".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_cgreen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cgreen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cblue(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "cblue".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_cblue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cblue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_calpha(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "calpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_calpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "calpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ccomp4(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ccomp4".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ccomp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ccomp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_quantize(mut self, val: Cop2ContrastQuantize) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ContrastMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ContrastFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ContrastFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ContrastFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dounpremult(mut self, val: bool) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dounpremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dounpremult".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Contrast {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "contrast"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ConvertDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 8 Bit Integer with B/W Points
    N8BitIntegerWithBWPoints = 1,
    /// 16 Bit Integer
    N16BitInteger = 2,
    /// 16 Bit Integer with B/W Points
    N16BitIntegerWithBWPoints = 3,
    /// 32 Bit Integer
    N32BitInteger = 4,
    /// 32 Bit Integer with B/W Points
    N32BitIntegerWithBWPoints = 5,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 6,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Convert {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Convert {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sequence to Convert"
    pub fn set_input_sequence_to_convert<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sequence to Convert" and specifies the output index of the target node.
    pub fn set_input_sequence_to_convert_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Reference Planes"
    pub fn set_input_reference_planes<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Reference Planes" and specifies the output index of the target node.
    pub fn set_input_reference_planes_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_depth(mut self, val: Cop2ConvertDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_bwpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Convert {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "convert"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2ConvolveMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ConvolveFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ConvolveFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ConvolveFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Convolve {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Convolve {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Convolve"
    pub fn set_input_image_to_convolve<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Convolve" and specifies the output index of the target node.
    pub fn set_input_image_to_convolve_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Convolve Kernel"
    pub fn set_input_convolve_kernel<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Convolve Kernel" and specifies the output index of the target node.
    pub fn set_input_convolve_kernel_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_kernel0(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel0".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel1(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel1".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel2(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel2".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel3(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel3".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel4(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel4".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel5(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel5".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel6(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel6".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel7(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel7".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kernel8(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "kernel8".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_kernel8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_ksize(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "ksize".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_ksize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ksize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_kplane(mut self, val: i32) -> Self {
        self.params.insert(
            "kplane".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_kplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ConvolveMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ConvolveFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ConvolveFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ConvolveFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_normalize(mut self, val: bool) -> Self {
        self.params.insert(
            "normalize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Convolve {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "convolve"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct Cop2Cop2net {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Cop2net {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Cop2net {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "cop2net"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2CornerrampDistance {
    Radial = 0,
    Block = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampPlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampPreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CornerrampPostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Cornerramp {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Cornerramp {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Add To"
    pub fn set_input_image_to_add_to<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Add To" and specifies the output index of the target node.
    pub fn set_input_image_to_add_to_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_pweight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("p{}weight", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pweight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("p{}weight", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_ppos_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("p{}pos", index1),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ppos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("p{}pos", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_pcolor_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("p{}color", index1),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_pcolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("p{}color", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
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
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
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
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_distance(mut self, val: Cop2CornerrampDistance) -> Self {
        self.params.insert(
            "distance".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2CornerrampPlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2CornerrampAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2CornerrampAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2CornerrampDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2CornerrampInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2CornerrampIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2CornerrampPreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2CornerrampPostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "keepaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Cornerramp {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "cornerramp"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2CropCrop {
    ByCorners = 0,
    /// By Offset, Width & Height
    ByOffsetWidthHeight = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2CropUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Crop {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Crop {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sequence to Crop"
    pub fn set_input_sequence_to_crop<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sequence to Crop" and specifies the output index of the target node.
    pub fn set_input_sequence_to_crop_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float2 parameters ---
    pub fn with_hcrop(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "hcrop".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_hcrop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hcrop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vcrop(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "vcrop".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vcrop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vcrop".to_string(),
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

    // --- Menu parameters ---
    pub fn with_crop(mut self, val: Cop2CropCrop) -> Self {
        self.params.insert(
            "crop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_crop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: Cop2CropUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hold(mut self, val: bool) -> Self {
        self.params.insert(
            "hold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Crop {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "crop"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum Cop2CryptomatteMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone)]
pub struct Cop2Cryptomatte {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl Cop2Cryptomatte {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Cryptomatte Image"
    pub fn set_input_cryptomatte_image<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Cryptomatte Image" and specifies the output index of the target node.
    pub fn set_input_cryptomatte_image_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_maskinput(mut self, val: Cop2CryptomatteMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_channelname(mut self, val: &str) -> Self {
        self.params.insert(
            "channelname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_channelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channelname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_selection(mut self, val: &str) -> Self {
        self.params.insert(
            "selection".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_selection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for Cop2Cryptomatte {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "cryptomatte"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
