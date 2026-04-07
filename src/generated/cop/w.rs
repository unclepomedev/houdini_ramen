#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWipeOp {
    CrossDissolve = 0,
    /// Per-Pixel Flip
    PerMinusPixelFlip = 1,
    Wipe = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWipeShape {
    Line = 0,
    Rectangle = 1,
    Circle = 2,
    /// 4 Corner Shrink
    N4CornerShrink = 3,
}

#[derive(Debug, Clone)]
pub struct CopWipe {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopWipe {
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
    pub fn with_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direction(mut self, val: f32) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_lwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "lwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_lcolor(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "lcolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_lcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_op(mut self, val: CopWipeOp) -> Self {
        self.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shape(mut self, val: CopWipeShape) -> Self {
        self.params.insert(
            "shape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shape".to_string(),
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

impl crate::core::types::HoudiniNode for CopWipe {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wipe"
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
pub enum CopWorleynoiseElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWorleynoiseJittertype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWorleynoiseLattice {
    Grid = 0,
    Hexagonal = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWorleynoiseMetric {
    Euclidean = 0,
    Manhattan = 1,
    Chebyshev = 2,
}

#[derive(Debug, Clone)]
pub struct CopWorleynoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopWorleynoise {
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
    pub fn with_jitterscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_jitterscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
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
    pub fn with_elementsizetype(mut self, val: CopWorleynoiseElementsizetype) -> Self {
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
    pub fn with_jittertype(mut self, val: CopWorleynoiseJittertype) -> Self {
        self.params.insert(
            "jittertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_jittertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jittertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lattice(mut self, val: CopWorleynoiseLattice) -> Self {
        self.params.insert(
            "lattice".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lattice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lattice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metric(mut self, val: CopWorleynoiseMetric) -> Self {
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

    // --- Toggle parameters ---
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

impl crate::core::types::HoudiniNode for CopWorleynoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "worleynoise"
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
pub enum CopWorleynoise3dElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWorleynoise3dJittertype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopWorleynoise3dMetric {
    Euclidean = 0,
    Manhattan = 1,
    Chebyshev = 2,
}

#[derive(Debug, Clone)]
pub struct CopWorleynoise3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopWorleynoise3d {
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
    pub fn with_jitterscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_jitterscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
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
    pub fn with_elementsizetype(mut self, val: CopWorleynoise3dElementsizetype) -> Self {
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
    pub fn with_jittertype(mut self, val: CopWorleynoise3dJittertype) -> Self {
        self.params.insert(
            "jittertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_jittertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jittertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metric(mut self, val: CopWorleynoise3dMetric) -> Self {
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

impl crate::core::types::HoudiniNode for CopWorleynoise3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "worleynoise3d"
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
pub enum CopWrangleType {
    Mono = 0,
    Uv = 1,
    Rgb = 2,
    Rgba = 3,
    Id = 4,
    Geometry = 5,
    FloatVdb = 6,
    VectorVdb = 7,
    IntegerVdb = 8,
}

#[derive(Debug, Clone)]
pub struct CopWrangle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopWrangle {
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

    // --- Int parameters ---
    pub fn with_geoinput_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("geoinput{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geoinput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("geoinput{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: CopWrangleType) -> Self {
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

    // --- String parameters ---
    pub fn with_vexsnippet(mut self, val: &str) -> Self {
        self.params.insert(
            "vexsnippet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vexsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexsnippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_exportlist(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_exportlist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_exportlist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_exportlist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_outputmask(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_outputmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_outputmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_outputmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_layer_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("layer{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_layer_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("layer{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_vex_strict(mut self, val: bool) -> Self {
        self.params.insert(
            "vex_strict".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vex_strict_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_strict".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopWrangle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wrangle"
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
