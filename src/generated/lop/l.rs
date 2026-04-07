#[derive(Debug, Clone)]
pub struct LopLayerbreak {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLayerbreak {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLayerbreak {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "layerbreak"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopLayerreplace {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLayerreplace {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Replace Layer With Flattened Stage"
    pub fn set_input_replace_layer_with_flattened_stage<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Replace Layer With Flattened Stage" and specifies the output index of the target node.
    pub fn set_input_replace_layer_with_flattened_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_replace_layer_with_flattened_stage_by_name<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_replacepath(mut self, val: &str) -> Self {
        self.params.insert(
            "replacepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_replacepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "replacepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newpath(mut self, val: &str) -> Self {
        self.params.insert(
            "newpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLayerreplace {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "layerreplace"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopLayoutXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLayoutRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopLayout {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLayout {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_apply(mut self) -> Self {
        self.params
            .insert("apply".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_reset(mut self) -> Self {
        self.params
            .insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Data parameters ---
    pub fn with_delta(mut self, val: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_delta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_laststroke(mut self, val: &str) -> Self {
        self.params.insert(
            "laststroke".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_laststroke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "laststroke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lasttemplatepoints(mut self, val: &str) -> Self {
        self.params.insert(
            "lasttemplatepoints".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_lasttemplatepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lasttemplatepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lastpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "lastpoints".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_lastpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lastpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_points(mut self, val: &str) -> Self {
        self.params.insert(
            "points".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_points_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "points".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_templatepoints(mut self, val: &str) -> Self {
        self.params.insert(
            "templatepoints".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_templatepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "templatepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_editdelta(mut self, val: &str) -> Self {
        self.params.insert(
            "editdelta".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_editdelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "editdelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_duplicate(mut self, val: &str) -> Self {
        self.params.insert(
            "duplicate".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_duplicate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "duplicate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delete(mut self, val: &str) -> Self {
        self.params.insert(
            "delete".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_delete_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delete".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_workingsetinfo(mut self, val: &str) -> Self {
        self.params.insert(
            "workingsetinfo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_workingsetinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "workingsetinfo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
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
    pub fn with_globalscale(mut self, val: f32) -> Self {
        self.params.insert(
            "globalscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_stroke_num(mut self, val: i32) -> Self {
        self.params.insert(
            "stroke_num".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_num_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_num".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: i32) -> Self {
        self.params
            .insert("mode".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input(mut self, val: i32) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: LopLayoutXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopLayoutRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_protopattern(mut self, val: &str) -> Self {
        self.params.insert(
            "protopattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_protopattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protopattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewprotopattern(mut self, val: &str) -> Self {
        self.params.insert(
            "previewprotopattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_previewprotopattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewprotopattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_physprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "physprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_physprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "physprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bypassprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "bypassprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bypassprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bypassprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformdescription(mut self, val: &str) -> Self {
        self.params.insert(
            "xformdescription".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformdescription_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformdescription".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userinfo(mut self, val: &str) -> Self {
        self.params.insert(
            "userinfo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_userinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userinfo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_laststrokebrush(mut self, val: &str) -> Self {
        self.params.insert(
            "laststrokebrush".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_laststrokebrush_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "laststrokebrush".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remove_edited_group(mut self, val: &str) -> Self {
        self.params.insert(
            "remove_edited_group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remove_edited_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remove_edited_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_localxform(mut self, val: bool) -> Self {
        self.params.insert(
            "localxform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_localxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLayout {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "layout"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("ASSETS")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopLayoutInnerExt {
    fn brushes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn end_assets(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopLayoutInnerExt for crate::core::graph::InnerGraph<'a, LopLayout> {
    fn brushes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ASSETS/BRUSHES")
    }
    fn end_assets(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ASSETS/END_ASSETS")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLightCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLightLighttype {
    Cylinder = 0,
    Distant = 1,
    Disk = 2,
    Point = 3,
    Rectangle = 4,
    Sphere = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLightXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLightRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopLight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLight {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distance".to_string(),
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
    pub fn with_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsintensity_i0a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsintensity_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsradius_mva(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsradius_mva".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsradius_mva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsradius_mva".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputswidth_zta(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputswidth_zta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputswidth_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputswidth_zta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsheight_mva(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsheight_mva".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsheight_mva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsheight_mva".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputslength_mva(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputslength_mva".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputslength_mva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputslength_mva".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsangle_zta(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsangle_zta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsangle_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsangle_zta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_s3a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_s3a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_s3a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_s3a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spotlightdist(mut self, val: f32) -> Self {
        self.params.insert(
            "spotlightdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spotlightdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spotlightdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_wcbhe(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_wcbhe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_wcbhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_wcbhe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_shbhe(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_shbhe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_shbhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_shbhe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorleft(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorleft".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorleft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorleftedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorleftedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorleftedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleftedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorright(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorright".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorright".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorrightedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorrightedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorrightedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorrightedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoortop(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoortop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoortop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoortopedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoortopedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoortopedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortopedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorbottom(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorbottom".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorbottomedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorbottomedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorbottomedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottomedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_e5ah(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_e5ah".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_e5ah_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_e5ah".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_fjbhd(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_fjbhd".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_fjbhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_fjbhd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_06ag(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_06ag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_06ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_06ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_5fbg(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_5fbg".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_5fbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_5fbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_handlefocus(mut self, val: f32) -> Self {
        self.params.insert(
            "handlefocus".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_handlefocus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlefocus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_o8a(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_o8a".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_o8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_o8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hitpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hitpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnormal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hitnormal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hitnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitnormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowpivot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shadowpivot".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadowpivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowpivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowtarget(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shadowtarget".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadowtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowtarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pret(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pret".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pret_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pret".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prer(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "prer".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatposition(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lookatposition".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookatposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatposition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatprimpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lookatprimpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookatprimpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatprimpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatprimrot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lookatprimrot".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookatprimrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatprimrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upvec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolor_zta(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputscolor_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_wcbh(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_wcbh".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_wcbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_wcbh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopLightCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lighttype(mut self, val: LopLightLighttype) -> Self {
        self.params.insert(
            "lighttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lighttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lighttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopLightXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopLightRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_treataspoint_control(mut self, val: &str) -> Self {
        self.params.insert(
            "treatAsPoint_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_treataspoint_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "treatAsPoint_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatprim(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvecmethod(mut self, val: &str) -> Self {
        self.params.insert(
            "upvecmethod".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upvecmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvecmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsintensity_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsintensity_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputstexturefile_control_shbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_control_shbh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputstexturefile_control_shbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_control_shbh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputstexturefile_r3ah(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_r3ah".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputstexturefile_r3ah_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_r3ah".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry_control(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsradius_control_n8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsradius_control_n8a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsradius_control_n8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsradius_control_n8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputswidth_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputswidth_control_06a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputswidth_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputswidth_control_06a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsheight_control_n8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsheight_control_n8a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsheight_control_n8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsheight_control_n8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputslength_control_n8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputslength_control_n8a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputslength_control_n8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputslength_control_n8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsangle_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsangle_control_06a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsangle_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsangle_control_06a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_control_pmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_control_pmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_control_pmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_control_pmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_control_thb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_control_thb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_control_thb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_control_thb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_control_2kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_control_2kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_control_2kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_control_2kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_control_m8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_control_m8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_lva(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_lva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_control_xpbhe(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_control_xpbhe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_control_xpbhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_control_xpbhe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_control_tubhe(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_control_tubhe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_control_tubhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_control_tubhe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorleft_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorleft_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoorleft_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleft_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorleftedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorleftedge_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoorleftedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleftedge_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorright_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorright_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoorright_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorright_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorrightedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorrightedge_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoorrightedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorrightedge_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoortop_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoortop_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoortop_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortop_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoortopedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoortopedge_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoortopedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortopedge_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorbottom_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorbottom_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoorbottom_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottom_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barndoorbottomedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorbottomedge_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barndoorbottomedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottomedge_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_control_fjbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_control_fjbh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_control_fjbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_control_fjbh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_control_xpbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_control_xpbh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_control_xpbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_control_xpbh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_control_ombhd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_control_ombhd".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_control_ombhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_control_ombhd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_n8ahd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_n8ahd".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_n8ahd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_n8ahd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_control_tubhd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_control_tubhd".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_control_tubhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_control_tubhd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_control_gwbhd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_control_gwbhd".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_control_gwbhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_control_gwbhd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_control_1kbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_control_1kbg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_control_1kbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_control_1kbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_control_6sbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_control_6sbg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_control_6sbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_control_6sbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_treataspoint(mut self, val: bool) -> Self {
        self.params.insert(
            "treatAsPoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_treataspoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "treatAsPoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hit(mut self, val: bool) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hasshadowpivot(mut self, val: bool) -> Self {
        self.params.insert(
            "hasshadowpivot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasshadowpivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasshadowpivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hasshadowtarget(mut self, val: bool) -> Self {
        self.params.insert(
            "hasshadowtarget".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasshadowtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasshadowtarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatenable(mut self, val: bool) -> Self {
        self.params.insert(
            "lookatenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lookatenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepposition(mut self, val: bool) -> Self {
        self.params.insert(
            "keepposition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepposition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_omb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_omb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_16a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_16a".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_16a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_16a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spotlightenable(mut self, val: bool) -> Self {
        self.params.insert(
            "spotlightenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spotlightenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spotlightenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "focusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_focusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iesenable(mut self, val: bool) -> Self {
        self.params.insert(
            "iesenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iesenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iesenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_shbhd(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_shbhd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_shbhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_shbhd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "light"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopLightfilterlibrary {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLightfilterlibrary {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_filllightfilters(mut self) -> Self {
        self.params.insert(
            "filllightfilters".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterpathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "filterpathprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filterpathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterpathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filternet(mut self, val: &str) -> Self {
        self.params.insert(
            "filternet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filternet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filternet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_containerpath(mut self, val: &str) -> Self {
        self.params.insert(
            "containerpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_containerpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "containerpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filternode_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filternode{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filternode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filternode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filterpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filterpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filterpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lightpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("lightpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lightpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("lightpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allowparmanim(mut self, val: bool) -> Self {
        self.params.insert(
            "allowparmanim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowparmanim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowparmanim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matflag_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("matflag{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matflag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matflag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assign_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("assign{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_assign_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assign{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLightfilterlibrary {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lightfilterlibrary"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopLightlinker {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLightlinker {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_collection_prim_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_prim_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_prim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_prim_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collection_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_name_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_name_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collection_includes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_includes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_includes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_includes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collection_excludes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_excludes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_excludes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_excludes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_id_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_id_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_id_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_id_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_prim_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_prim_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_prim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_prim_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_includes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_includes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_includes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_includes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_excludes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_excludes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_excludes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_excludes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_type_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_type_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_type_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uiconfig(mut self, val: &str) -> Self {
        self.params.insert(
            "uiconfig".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uiconfig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uiconfig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_link_enabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("link_enabled_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_link_enabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_enabled_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_ispathexpression_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("link_ispathexpression_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_link_ispathexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_ispathexpression_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_reversed_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("link_reversed_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_link_reversed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_reversed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLightlinker {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lightlinker"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopLightmixerXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLightmixerRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopLightmixer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLightmixer {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_apply(mut self) -> Self {
        self.params
            .insert("apply".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_reset(mut self) -> Self {
        self.params
            .insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_removeunusedtransforms(mut self) -> Self {
        self.params.insert(
            "removeunusedtransforms".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_delta(mut self, val: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_delta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
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

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: LopLightmixerXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopLightmixerRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_collection_prim_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_prim_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_prim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_prim_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collection_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_name_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_name_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collection_includes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_includes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_includes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_includes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collection_excludes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_excludes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collection_excludes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_excludes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_physprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "physprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_physprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "physprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bypassprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "bypassprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bypassprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bypassprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformdescription(mut self, val: &str) -> Self {
        self.params.insert(
            "xformdescription".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformdescription_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformdescription".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_localxform(mut self, val: bool) -> Self {
        self.params.insert(
            "localxform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_localxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLightmixer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lightmixer"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopLiverenderResScaleMenu {
    /// 1/10 (One Tenth Resolution)
    N110OneTenthResolution = 0,
    /// 1/5 (One Fifth Resolution)
    N15OneFifthResolution = 1,
    /// 1/4 (One Quarter Resolution)
    N14OneQuarterResolution = 2,
    /// 1/3 (One Third Resolution)
    N13OneThirdResolution = 3,
    /// 1/2 (One Half Resolution)
    N12OneHalfResolution = 4,
    /// 2/3 (Two Thirds Resolution)
    N23TwoThirdsResolution = 5,
    /// 3/4 (Three Quarters Resolution)
    N34ThreeQuartersResolution = 6,
    FullResolution = 7,
}

#[derive(Debug, Clone)]
pub struct LopLiverender {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLiverender {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_startrender(mut self) -> Self {
        self.params.insert(
            "startrender".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_stoprender(mut self) -> Self {
        self.params.insert(
            "stoprender".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_res_usermenu(mut self) -> Self {
        self.params.insert(
            "res_userMenu".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_rendering(mut self, val: i32) -> Self {
        self.params.insert(
            "rendering".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rendering_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendering".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_res_scale(mut self, val: i32) -> Self {
        self.params.insert(
            "res_scale".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_res_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res_user(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "res_user".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_res_user_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_user".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_renderdelegate(mut self, val: i32) -> Self {
        self.params.insert(
            "renderdelegate".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_renderdelegate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderdelegate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_res_scale_menu(mut self, val: LopLiverenderResScaleMenu) -> Self {
        self.params.insert(
            "res_scale_menu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_res_scale_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_scale_menu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_renderid(mut self, val: &str) -> Self {
        self.params.insert(
            "renderid".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updaterenderermessage(mut self, val: &str) -> Self {
        self.params.insert(
            "updaterenderermessage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_updaterenderermessage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updaterenderermessage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updaterendererwarning(mut self, val: &str) -> Self {
        self.params.insert(
            "updaterendererwarning".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_updaterendererwarning_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updaterendererwarning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updaterenderererror(mut self, val: &str) -> Self {
        self.params.insert(
            "updaterenderererror".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_updaterenderererror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updaterenderererror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendersettings(mut self, val: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderpass(mut self, val: &str) -> Self {
        self.params.insert(
            "renderpass".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderpass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderpass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_override_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "override_camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_override_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_override_res(mut self, val: &str) -> Self {
        self.params.insert(
            "override_res".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_override_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_runlopnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "runlopnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_runlopnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "runlopnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLiverender {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "liverender"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("USER_AREA")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopLiverenderInnerExt {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopLiverenderInnerExt for crate::core::graph::InnerGraph<'a, LopLiverender> {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("USER_AREA/output0")
    }
}

#[derive(Debug, Clone)]
pub struct LopLoadlayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLoadlayer {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- String parameters ---
    pub fn with_setstagemetadata(mut self, val: &str) -> Self {
        self.params.insert(
            "setstagemetadata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_setstagemetadata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setstagemetadata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepath(mut self, val: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_handlemissingfiles(mut self, val: &str) -> Self {
        self.params.insert(
            "handlemissingfiles".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_handlemissingfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlemissingfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "newfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_loadpayloads(mut self, val: bool) -> Self {
        self.params.insert(
            "loadpayloads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadpayloads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadpayloads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLoadlayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "loadlayer"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopLoftpayloadinfo {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLoftpayloadinfo {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("color{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setcolor_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("setcolor{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_setcolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("setcolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setstats(mut self, val: bool) -> Self {
        self.params.insert(
            "setstats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setstats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setstats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setcountstats(mut self, val: bool) -> Self {
        self.params.insert(
            "setcountstats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcountstats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcountstats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLoftpayloadinfo {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "loftpayloadinfo"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopLookatconstraintSourcetype {
    Primitives = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintTargetsource {
    FirstInput = 0,
    SecondInput = 1,
    Position = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintTargettype {
    Primitive = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintLookataxis {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// Z-
    ZMinus = 2,
    /// X+
    XPlus = 3,
    /// Y+
    YPlus = 4,
    /// Z+
    ZPlus = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintLookupaxisx {
    /// Y-
    YMinus = 0,
    /// Z-
    ZMinus = 1,
    /// Y+
    YPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintLookupaxisy {
    /// X-
    XMinus = 0,
    /// Z-
    ZMinus = 1,
    /// X+
    XPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintLookupaxisz {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// X+
    XPlus = 2,
    /// Y+
    YPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintUp {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
    FromPrimitive = 3,
    Custom = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopLookatconstraintUpvectorsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone)]
pub struct LopLookatconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLookatconstraint {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Stage Containing Target Primitives (optional)"
    pub fn set_input_stage_containing_target_primitives_optio<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Stage Containing Target Primitives (optional)" and specifies the output index of the target node.
    pub fn set_input_stage_containing_target_primitives_optio_from<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_stage_containing_target_primitives_optio_by_name<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_importtime(mut self, val: f32) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_importtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "targetpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_targetpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetype(mut self, val: LopLookatconstraintSourcetype) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourcetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsource(mut self, val: LopLookatconstraintTargetsource) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targetsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targettype(mut self, val: LopLookatconstraintTargettype) -> Self {
        self.params.insert(
            "targettype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectorsource(mut self, val: LopLookatconstraintUpvectorsource) -> Self {
        self.params.insert(
            "upvectorsource".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_upvectorsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_lookataxis(mut self, val: LopLookatconstraintLookataxis) -> Self {
        self.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: LopLookatconstraintLookupaxisx) -> Self {
        self.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: LopLookatconstraintLookupaxisy) -> Self {
        self.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: LopLookatconstraintLookupaxisz) -> Self {
        self.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: LopLookatconstraintUp) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_sourceinstances(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceinstances".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourceinstances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceinstances".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetinstance(mut self, val: &str) -> Self {
        self.params.insert(
            "targetinstance".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetinstance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetinstance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectorxform(mut self, val: &str) -> Self {
        self.params.insert(
            "upvectorxform".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upvectorxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hidetarget(mut self, val: bool) -> Self {
        self.params.insert(
            "hidetarget".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hidetarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hidetarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLookatconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lookatconstraint"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopLopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLopnet {
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

    // --- Int parameters ---
    pub fn with_modifiedprimcounttostartnewlayer(mut self, val: i32) -> Self {
        self.params.insert(
            "modifiedprimcounttostartnewlayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modifiedprimcounttostartnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modifiedprimcounttostartnewlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_expansioneffect(mut self, val: &str) -> Self {
        self.params.insert(
            "expansioneffect".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expansioneffect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expansioneffect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinnedprims(mut self, val: &str) -> Self {
        self.params.insert(
            "pinnedprims".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pinnedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinnedprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextassetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextassetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst_expr(
        mut self,
        index1: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantselectionset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantselectionvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insertionpointdescriptor(mut self, val: &str) -> Self {
        self.params.insert(
            "insertionpointdescriptor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_insertionpointdescriptor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insertionpointdescriptor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendergallerysource(mut self, val: &str) -> Self {
        self.params.insert(
            "rendergallerysource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendergallerysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendergallerysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_resolvercontextstringenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolvercontextstringenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantselectionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lopnet"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopLpetagPreprocess {
    None = 0,
    LightName = 1,
    LightIndex = 2,
    TypeName = 3,
    Token = 4,
    Vexpression = 5,
}

#[derive(Debug, Clone)]
pub struct LopLpetag {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopLpetag {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_populate(mut self) -> Self {
        self.params.insert(
            "populate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_tokenindex(mut self, val: i32) -> Self {
        self.params.insert(
            "tokenindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tokenindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tokenindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_preprocess(mut self, val: LopLpetagPreprocess) -> Self {
        self.params.insert(
            "preprocess".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preprocess_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preprocess".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_lpetagattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "lpetagattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpetagattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpetagattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geolpetagattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "geolpetagattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geolpetagattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geolpetagattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addedgeometryprims(mut self, val: &str) -> Self {
        self.params.insert(
            "addedgeometryprims".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_addedgeometryprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addedgeometryprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultlpetag(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultlpetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultlpetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultlpetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefix(mut self, val: &str) -> Self {
        self.params.insert(
            "prefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tokenseparator(mut self, val: &str) -> Self {
        self.params.insert(
            "tokenseparator".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tokenseparator_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tokenseparator".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vexpression(mut self, val: &str) -> Self {
        self.params.insert(
            "vexpression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexpression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpetag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("lpetag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpetag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("lpetag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopLpetag {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lpetag"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
