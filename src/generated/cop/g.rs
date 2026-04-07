#[derive(Debug, Clone)]
pub struct CopGamma {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGamma {
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
    pub fn with_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "gamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
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
    pub fn with_invert(mut self, val: bool) -> Self {
        self.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ispremult(mut self, val: bool) -> Self {
        self.params.insert(
            "ispremult".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ispremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ispremult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopGamma {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "gamma"
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
pub enum CopGeotolayerPrecision {
    /// 8
    N8 = 0,
    /// 16
    N16 = 1,
    /// 32
    N32 = 2,
}

#[derive(Debug, Clone)]
pub struct CopGeotolayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGeotolayer {
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

    // --- Menu parameters ---
    pub fn with_precision(mut self, val: CopGeotolayerPrecision) -> Self {
        self.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "precision".to_string(),
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
    pub fn with_setprecision(mut self, val: bool) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopGeotolayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "geotolayer"
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
pub enum CopGlowFilter {
    Box = 0,
    Gaussian = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopGlowUnits {
    Image = 0,
    Pixels = 1,
}

#[derive(Debug, Clone)]
pub struct CopGlow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGlow {
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
    pub fn with_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bright(mut self, val: f32) -> Self {
        self.params.insert(
            "bright".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bright".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_satscale(mut self, val: f32) -> Self {
        self.params.insert(
            "satscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_satscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "satscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_size_pixel(mut self, val: f32) -> Self {
        self.params.insert(
            "size_pixel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_size_pixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size_pixel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_scales(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "scales".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_scales_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scales".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gaintint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gaintint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gaintint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gaintint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_filter(mut self, val: CopGlowFilter) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: CopGlowUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
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
    pub fn with_outputonlyglow(mut self, val: bool) -> Self {
        self.params.insert(
            "outputonlyglow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputonlyglow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputonlyglow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopGlow {
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
pub enum CopGrungeAuroraElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopGrungeAurora {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGrungeAurora {
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
    pub fn with_diststrength(mut self, val: f32) -> Self {
        self.params.insert(
            "diststrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diststrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diststrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distsize(mut self, val: f32) -> Self {
        self.params.insert(
            "distsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distsize".to_string(),
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

    // --- Int parameters ---
    pub fn with_maxsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "maxsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_elementsizetype(mut self, val: CopGrungeAuroraElementsizetype) -> Self {
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

    // --- Ramp parameters ---
    pub fn with_streakweight(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "streakweight".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_streakweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "streakweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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

impl crate::core::types::HoudiniNode for CopGrungeAurora {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "grunge_aurora"
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
pub enum CopGrungeBirchbarkElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopGrungeBirchbark {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGrungeBirchbark {
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
    pub fn with_basecoverage(mut self, val: f32) -> Self {
        self.params.insert(
            "basecoverage".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basecoverage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecoverage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basesharpness(mut self, val: f32) -> Self {
        self.params.insert(
            "basesharpness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basesharpness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basesharpness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lenticels(mut self, val: f32) -> Self {
        self.params.insert(
            "lenticels".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lenticels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lenticels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lenticelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "lenticelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lenticelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lenticelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cracks(mut self, val: f32) -> Self {
        self.params.insert(
            "cracks".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cracks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cracks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cracksize(mut self, val: f32) -> Self {
        self.params.insert(
            "cracksize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cracksize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cracksize".to_string(),
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
    pub fn with_elementsizetype(mut self, val: CopGrungeBirchbarkElementsizetype) -> Self {
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

impl crate::core::types::HoudiniNode for CopGrungeBirchbark {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "grunge_birchbark"
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
pub enum CopGrungeLayerednoiseElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopGrungeLayerednoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGrungeLayerednoise {
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
    pub fn with_basenoise_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "basenoise_amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basenoise_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basenoise_amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basenoise_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "basenoise_elementsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basenoise_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basenoise_elementsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecells_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "basecells_amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basecells_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecells_amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecells_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "basecells_elementsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basecells_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecells_elementsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secnoise_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "secnoise_amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_secnoise_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secnoise_amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secnoise_center(mut self, val: f32) -> Self {
        self.params.insert(
            "secnoise_center".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_secnoise_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secnoise_center".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secnoise_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "secnoise_elementsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_secnoise_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secnoise_elementsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seccells_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "seccells_amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seccells_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seccells_amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seccells_center(mut self, val: f32) -> Self {
        self.params.insert(
            "seccells_center".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seccells_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seccells_center".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seccells_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "seccells_elementsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seccells_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seccells_elementsize".to_string(),
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
    pub fn with_elementsizetype(mut self, val: CopGrungeLayerednoiseElementsizetype) -> Self {
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
    pub fn with_basecells_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "basecells_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basecells_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecells_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecells_complement(mut self, val: bool) -> Self {
        self.params.insert(
            "basecells_complement".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basecells_complement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecells_complement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secnoise_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "secnoise_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_secnoise_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secnoise_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secnoise_complement(mut self, val: bool) -> Self {
        self.params.insert(
            "secnoise_complement".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_secnoise_complement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secnoise_complement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seccells_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "seccells_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seccells_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seccells_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seccells_complement(mut self, val: bool) -> Self {
        self.params.insert(
            "seccells_complement".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seccells_complement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seccells_complement".to_string(),
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

impl crate::core::types::HoudiniNode for CopGrungeLayerednoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "grunge_layerednoise"
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
pub enum CopGrungePinebarkElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopGrungePinebark {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGrungePinebark {
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
    pub fn with_barkintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "barkintensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barkintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barkintensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barksize(mut self, val: f32) -> Self {
        self.params.insert(
            "barksize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barksize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barksize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barkroughness(mut self, val: f32) -> Self {
        self.params.insert(
            "barkroughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barkroughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barkroughness".to_string(),
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
    pub fn with_elementsizetype(mut self, val: CopGrungePinebarkElementsizetype) -> Self {
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

impl crate::core::types::HoudiniNode for CopGrungePinebark {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "grunge_pinebark"
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
pub enum CopGrungeRustElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone)]
pub struct CopGrungeRust {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopGrungeRust {
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
    pub fn with_coverage(mut self, val: f32) -> Self {
        self.params.insert(
            "coverage".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coverage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coverage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rust(mut self, val: f32) -> Self {
        self.params.insert(
            "rust".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rust".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rustsize(mut self, val: f32) -> Self {
        self.params.insert(
            "rustsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rustsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rustsize".to_string(),
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
    pub fn with_elementsizetype(mut self, val: CopGrungeRustElementsizetype) -> Self {
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

impl crate::core::types::HoudiniNode for CopGrungeRust {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "grunge_rust"
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
