#[derive(Debug, Clone)]
pub struct SopEach {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEach {
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

    /// Connects to input 0: "Geometry To Select From"
    pub fn set_input_geometry_to_select_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Geometry To Select From" and specifies the output index of the target node.
    pub fn set_input_geometry_to_select_from_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_geometry_to_select_from_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_attribtol(mut self, val: f32) -> Self {
        self.params.insert(
            "attribtol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attribtol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_attribval(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "attribval".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_attribval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_forpath(mut self, val: &str) -> Self {
        self.params.insert(
            "forpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_forpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_feedback(mut self, val: bool) -> Self {
        self.params.insert(
            "feedback".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_feedback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "feedback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eachpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "eachpoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_eachpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eachpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEach {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "each"
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
pub struct SopEdgecollapse {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgecollapse {
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

    /// Connects to input 0: "Input Geometry"
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_connectivityattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "connectivityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_connectivityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectivityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_removedegen(mut self, val: bool) -> Self {
        self.params.insert(
            "removedegen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removedegen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removedegen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatenmls(mut self, val: bool) -> Self {
        self.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updatenmls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgecollapse {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgecollapse"
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
pub struct SopEdgecusp {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgecusp {
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

    /// Connects to input 0: "Input Geometry"
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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

    // --- Toggle parameters ---
    pub fn with_updatenorms(mut self, val: bool) -> Self {
        self.params.insert(
            "updatenorms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updatenorms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatenorms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutboundaries(mut self, val: bool) -> Self {
        self.params.insert(
            "cutboundaries".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cutboundaries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutboundaries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgecusp {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgecusp"
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
pub struct SopEdgedivide {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgedivide {
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

    /// Connects to input 0: "Input Geometry"
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_tolerance(mut self, val: f32) -> Self {
        self.params.insert(
            "tolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numdivs(mut self, val: i32) -> Self {
        self.params.insert(
            "numdivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numdivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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

    // --- Toggle parameters ---
    pub fn with_applytoall(mut self, val: bool) -> Self {
        self.params.insert(
            "applytoall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applytoall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applytoall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "sharedpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sharedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "connectpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_connectpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closepath(mut self, val: bool) -> Self {
        self.params.insert(
            "closepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgedivide {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgedivide"
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
pub enum SopEdgeequalizeGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgeequalizeMethod {
    AverageEdgeLength = 0,
    LongestEdgeLength = 1,
    ShortestEdgeLength = 2,
}

#[derive(Debug, Clone)]
pub struct SopEdgeequalize {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgeequalize {
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

    /// Connects to input 0: "Input Geometry."
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry." and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_grouptype(mut self, val: SopEdgeequalizeGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: SopEdgeequalizeMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_outputedgegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "outputedgegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputedgegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputedgegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgeequalize {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgeequalize"
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
pub struct SopEdgeflip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgeflip {
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

    /// Connects to input 0: "Input Geometry"
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_cycles(mut self, val: i32) -> Self {
        self.params.insert(
            "cycles".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cycles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cycles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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

    // --- Toggle parameters ---
    pub fn with_cycleattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "cycleattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cycleattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cycleattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgeflip {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgeflip"
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
pub struct SopEdgefracture {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgefracture {
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

    /// Connects to input 0: "Geometry to Fracture"
    pub fn set_input_geometry_to_fracture<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Geometry to Fracture" and specifies the output index of the target node.
    pub fn set_input_geometry_to_fracture_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_geometry_to_fracture_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Fracture Curves"
    pub fn set_input_fracture_curves<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Fracture Curves" and specifies the output index of the target node.
    pub fn set_input_fracture_curves_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_fracture_curves_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_resampleres(mut self, val: f32) -> Self {
        self.params.insert(
            "resampleres".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_resampleres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resampleres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_initialpieces(mut self, val: i32) -> Self {
        self.params.insert(
            "initialpieces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_initialpieces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialpieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_cost(mut self, val: &str) -> Self {
        self.params.insert(
            "cost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_origptsgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "origptsgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_origptsgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "origptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newptsgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "newptsgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newptsgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doinitialpieces(mut self, val: bool) -> Self {
        self.params.insert(
            "doinitialpieces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doinitialpieces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doinitialpieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doresample(mut self, val: bool) -> Self {
        self.params.insert(
            "doresample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doresample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doresample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecost(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputpieceattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputpieceattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputpieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputpieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doorigptsgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "doorigptsgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorigptsgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doorigptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_donewptsgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "donewptsgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donewptsgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "donewptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgefracture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgefracture"
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
pub enum SopEdgerelaxGrouptype {
    Primitives = 0,
    Points = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgerelaxSecondinputtype {
    ReferencePositions = 0,
    /// Reference Edge Lengths (Line Network)
    ReferenceEdgeLengthsLineNetwork = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgerelaxTargetmode {
    IndividualEdgeLengths = 0,
    /// Scale-Independent Edge Length Distribution
    ScaleMinusIndependentEdgeLengthDistribution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgerelaxAnisoscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgerelaxAnisovolpreservemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone)]
pub struct SopEdgerelax {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgerelax {
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

    /// Connects to input 0: "Deformed Geometry"
    pub fn set_input_deformed_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Deformed Geometry" and specifies the output index of the target node.
    pub fn set_input_deformed_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_deformed_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Reference Geometry"
    pub fn set_input_reference_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Reference Geometry" and specifies the output index of the target node.
    pub fn set_input_reference_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_reference_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: "Slide Surface"
    pub fn set_input_slide_surface<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "Slide Surface" and specifies the output index of the target node.
    pub fn set_input_slide_surface_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_slide_surface_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_stepsize(mut self, val: f32) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisoscale(mut self, val: f32) -> Self {
        self.params.insert(
            "anisoscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisoscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisoscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisovolcompensation(mut self, val: f32) -> Self {
        self.params.insert(
            "anisovolcompensation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisovolcompensation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisovolcompensation".to_string(),
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
    pub fn with_slideiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "slideiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slideiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slideiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopEdgerelaxGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secondinputtype(mut self, val: SopEdgerelaxSecondinputtype) -> Self {
        self.params.insert(
            "secondinputtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_secondinputtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondinputtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetmode(mut self, val: SopEdgerelaxTargetmode) -> Self {
        self.params.insert(
            "targetmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisoscalemode(mut self, val: SopEdgerelaxAnisoscalemode) -> Self {
        self.params.insert(
            "anisoscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_anisoscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisoscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisovolpreservemode(mut self, val: SopEdgerelaxAnisovolpreservemode) -> Self {
        self.params.insert(
            "anisovolpreservemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_anisovolpreservemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisovolpreservemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_pingroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pingroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pingroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restlengthattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "restlengthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restlengthattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlengthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "dirattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisoscaleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "anisoscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisoscaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisoscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisovolpreserveattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "anisovolpreserveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisovolpreserveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisovolpreserveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_grouprefinput(mut self, val: bool) -> Self {
        self.params.insert(
            "grouprefinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_grouprefinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouprefinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthcondition(mut self, val: bool) -> Self {
        self.params.insert(
            "lengthcondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lengthcondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lengthcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slideonsurface(mut self, val: bool) -> Self {
        self.params.insert(
            "slideonsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_slideonsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slideonsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slideundistort(mut self, val: bool) -> Self {
        self.params.insert(
            "slideundistort".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_slideundistort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slideundistort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useanisoscale(mut self, val: bool) -> Self {
        self.params.insert(
            "useanisoscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanisoscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useanisoscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgerelax {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgerelax"
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
pub enum SopEdgestraightenGrouptype {
    Edges = 0,
    Points = 1,
    Vertices = 2,
}

#[derive(Debug, Clone)]
pub struct SopEdgestraighten {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgestraighten {
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

    /// Connects to input 0: "Input Geometry."
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry." and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_grouptype(mut self, val: SopEdgestraightenGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_outputedgegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "outputedgegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputedgegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputedgegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgestraighten {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgestraighten"
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
pub enum SopEdgetransportMethod {
    EachCurve = 0,
    EdgeNetwork = 1,
    ParentAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportCurvedir {
    Forward = 0,
    Backward = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportRoottype {
    FirstPoint = 0,
    LastPoint = 1,
    Group = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportOperation {
    Transport = 0,
    TransportFromRoot = 1,
    Total = 2,
    Maximum = 3,
    Minimum = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportNormalization {
    /// Per-Component
    PerMinusComponent = 0,
    Global = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportRootvalue {
    Zero = 0,
    Hold = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportSplitmethod {
    Copy = 0,
    Split = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEdgetransportMergemethod {
    Add = 0,
    Maximum = 1,
    Minimum = 2,
}

#[derive(Debug, Clone)]
pub struct SopEdgetransport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdgetransport {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_method(mut self, val: SopEdgetransportMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_curvedir(mut self, val: SopEdgetransportCurvedir) -> Self {
        self.params.insert(
            "curvedir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_curvedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roottype(mut self, val: SopEdgetransportRoottype) -> Self {
        self.params.insert(
            "roottype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_roottype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roottype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_operation(mut self, val: SopEdgetransportOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalization(mut self, val: SopEdgetransportNormalization) -> Self {
        self.params.insert(
            "normalization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_normalization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootvalue(mut self, val: SopEdgetransportRootvalue) -> Self {
        self.params.insert(
            "rootvalue".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rootvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitmethod(mut self, val: SopEdgetransportSplitmethod) -> Self {
        self.params.insert(
            "splitmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mergemethod(mut self, val: SopEdgetransportMergemethod) -> Self {
        self.params.insert(
            "mergemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mergemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_ptgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "primgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribute(mut self, val: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "parentattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "rootgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rootgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_donormalization(mut self, val: bool) -> Self {
        self.params.insert(
            "donormalization".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donormalization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "donormalization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignoreattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "ignoreattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignoreattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignoreattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalebyedge(mut self, val: bool) -> Self {
        self.params.insert(
            "scalebyedge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scalebyedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalebyedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotatebyedge(mut self, val: bool) -> Self {
        self.params.insert(
            "rotatebyedge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rotatebyedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotatebyedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdgetransport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgetransport"
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
pub enum SopEditGrouptype {
    GuessFromGroup = 0,
    Breakpoints = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditPrexformXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditPrexformRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditLprexformXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditLprexformRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditXformspace {
    Global = 0,
    LocalFrame = 1,
    LocalFrameByConnectivity = 2,
    LocalOriginByConnectivity = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditLocalframe {
    Component = 0,
    /// Normal + Vector
    NormalPlusVector = 1,
    OrientedBoundingBox = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditDistmetric {
    Attribute = 0,
    Edge = 1,
    Radius = 2,
    /// Radius with Connectivity (deprecated)
    RadiusWithConnectivityDeprecated = 3,
    Surface = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditType {
    Linear = 0,
    Quadratic = 1,
    Cubic = 2,
    /// Meta-ball
    MetaMinusBall = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditVisualizefalloff {
    Never = 0,
    Always = 1,
    WhenViewportToolIsActive = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditOp {
    DeformPoints = 0,
    SmoothPoints = 1,
    DeformAndSmooth = 2,
    EraseChanges = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditAxis {
    X = 0,
    Y = 1,
    Z = 2,
    Normal = 3,
    /// User Defined:
    UserDefined = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditColor {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditShape {
    Circle = 0,
    Square = 1,
    Bitmap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditBmpchan {
    Alpha = 0,
    Luminance = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditUptype {
    StrokeDirection = 0,
    Fixed = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEditEvent {
    BeginStroke = 0,
    ActiveStroke = 1,
    EndStroke = 2,
    Click = 3,
    /// No-op
    NoMinusOp = 4,
}

#[derive(Debug, Clone)]
pub struct SopEdit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEdit {
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

    /// Connects to input 0: "Geometry to Edit"
    pub fn set_input_geometry_to_edit<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Geometry to Edit" and specifies the output index of the target node.
    pub fn set_input_geometry_to_edit_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_geometry_to_edit_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Reference Geometry"
    pub fn set_input_reference_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Reference Geometry" and specifies the output index of the target node.
    pub fn set_input_reference_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_reference_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Button parameters ---
    pub fn trigger_apply(mut self) -> Self {
        self.params
            .insert("apply".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_stencilclear(mut self) -> Self {
        self.params.insert(
            "stencilclear".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_resetnormal(mut self) -> Self {
        self.params.insert(
            "resetnormal".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_flood(mut self) -> Self {
        self.params
            .insert("flood".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_clearall(mut self) -> Self {
        self.params.insert(
            "clearall".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fd(mut self, val: f32) -> Self {
        self.params
            .insert("fd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_fd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bd(mut self, val: f32) -> Self {
        self.params
            .insert("bd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sculptrad(mut self, val: f32) -> Self {
        self.params.insert(
            "sculptrad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sculptrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sculptrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvradius(mut self, val: f32) -> Self {
        self.params.insert(
            "uvradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uvradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiuspressure(mut self, val: f32) -> Self {
        self.params.insert(
            "radiuspressure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiuspressure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiuspressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushangle(mut self, val: f32) -> Self {
        self.params.insert(
            "brushangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "brushangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squash(mut self, val: f32) -> Self {
        self.params.insert(
            "squash".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_squash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "squash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity(mut self, val: f32) -> Self {
        self.params.insert(
            "opacity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacitypresure(mut self, val: f32) -> Self {
        self.params.insert(
            "opacitypresure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacitypresure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacitypresure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushsplatter(mut self, val: f32) -> Self {
        self.params.insert(
            "brushsplatter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushsplatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "brushsplatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_papergrain(mut self, val: f32) -> Self {
        self.params.insert(
            "papergrain".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_papergrain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "papergrain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_softedge(mut self, val: f32) -> Self {
        self.params.insert(
            "softedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpressure(mut self, val: f32) -> Self {
        self.params.insert(
            "hitpressure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hitpressure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitpressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symdist(mut self, val: f32) -> Self {
        self.params.insert(
            "symdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_symdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "symthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_symthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symplanetol(mut self, val: f32) -> Self {
        self.params.insert(
            "symplanetol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_symplanetol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symplanetol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_tandeg(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "tandeg".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_tandeg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tandeg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
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
    pub fn with_prexform_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "prexform_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prexform_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prexform_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prexform_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "prexform_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prexform_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prexform_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prexform_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "prexform_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prexform_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prexform_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prexform_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "prexform_shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prexform_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prexform_shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lpr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprexform_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lprexform_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lprexform_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprexform_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprexform_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lprexform_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lprexform_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprexform_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprexform_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lprexform_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lprexform_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprexform_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprexform_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lprexform_shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lprexform_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprexform_shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localspaceupvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localspaceupvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localspaceupvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localspaceupvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vector".to_string(),
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
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dir".to_string(),
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
    pub fn with_hituvw(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hituvw".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hituvw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvw".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symaxis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "symaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_symaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symorig(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "symorig".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_symorig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symorig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_hitprim(mut self, val: i32) -> Self {
        self.params.insert(
            "hitprim".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hitprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpt(mut self, val: i32) -> Self {
        self.params.insert(
            "hitpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hitpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopEditGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: SopEditXord) -> Self {
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
    pub fn with_rord(mut self, val: SopEditRord) -> Self {
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
    pub fn with_prexform_xord(mut self, val: SopEditPrexformXord) -> Self {
        self.params.insert(
            "prexform_xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_prexform_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prexform_xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prexform_rord(mut self, val: SopEditPrexformRord) -> Self {
        self.params.insert(
            "prexform_rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_prexform_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prexform_rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprexform_xord(mut self, val: SopEditLprexformXord) -> Self {
        self.params.insert(
            "lprexform_xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lprexform_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprexform_xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprexform_rord(mut self, val: SopEditLprexformRord) -> Self {
        self.params.insert(
            "lprexform_rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lprexform_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprexform_rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformspace(mut self, val: SopEditXformspace) -> Self {
        self.params.insert(
            "xformspace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localframe(mut self, val: SopEditLocalframe) -> Self {
        self.params.insert(
            "localframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_localframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distmetric(mut self, val: SopEditDistmetric) -> Self {
        self.params.insert(
            "distmetric".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distmetric_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distmetric".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type(mut self, val: SopEditType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualizefalloff(mut self, val: SopEditVisualizefalloff) -> Self {
        self.params.insert(
            "visualizefalloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_visualizefalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visualizefalloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_op(mut self, val: SopEditOp) -> Self {
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
    pub fn with_axis(mut self, val: SopEditAxis) -> Self {
        self.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color(mut self, val: SopEditColor) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_shape(mut self, val: SopEditShape) -> Self {
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
    pub fn with_bmpchan(mut self, val: SopEditBmpchan) -> Self {
        self.params.insert(
            "bmpchan".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bmpchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bmpchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uptype(mut self, val: SopEditUptype) -> Self {
        self.params.insert(
            "uptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_event(mut self, val: SopEditEvent) -> Self {
        self.params.insert(
            "event".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_event_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "event".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_leadislandhint(mut self, val: &str) -> Self {
        self.params.insert(
            "leadislandhint".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_leadislandhint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leadislandhint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distattr(mut self, val: &str) -> Self {
        self.params.insert(
            "distattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_leadptattr(mut self, val: &str) -> Self {
        self.params.insert(
            "leadptattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_leadptattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leadptattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernel(mut self, val: &str) -> Self {
        self.params.insert(
            "kernel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bitmap(mut self, val: &str) -> Self {
        self.params.insert(
            "bitmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bitmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bitmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sculptkernel(mut self, val: &str) -> Self {
        self.params.insert(
            "sculptkernel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sculptkernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sculptkernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_vlength(mut self, val: bool) -> Self {
        self.params.insert(
            "vlength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slideonsurface(mut self, val: bool) -> Self {
        self.params.insert(
            "slideonsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_slideonsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slideonsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_leadnml(mut self, val: bool) -> Self {
        self.params.insert(
            "leadnml".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_leadnml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leadnml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applyrolloff(mut self, val: bool) -> Self {
        self.params.insert(
            "applyrolloff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applyrolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applyrolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stencilaccum(mut self, val: bool) -> Self {
        self.params.insert(
            "stencilaccum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stencilaccum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stencilaccum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedepth(mut self, val: bool) -> Self {
        self.params.insert(
            "usedepth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projtype(mut self, val: bool) -> Self {
        self.params.insert(
            "projtype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useconnectivity(mut self, val: bool) -> Self {
        self.params.insert(
            "useconnectivity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useconnectivity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useconnectivity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenormals(mut self, val: bool) -> Self {
        self.params.insert(
            "usenormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_realtime(mut self, val: bool) -> Self {
        self.params.insert(
            "realtime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_realtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "realtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useorient(mut self, val: bool) -> Self {
        self.params.insert(
            "useorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doreflect(mut self, val: bool) -> Self {
        self.params.insert(
            "doreflect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doreflect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doreflect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatenmls(mut self, val: bool) -> Self {
        self.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updatenmls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEdit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edit"
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
pub enum SopElastictransformGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopElastictransformStrokeSculptmode {
    Grab = 0,
    Twist = 1,
    Scale = 2,
    Pinch = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopElastictransformStrokeDecay {
    SlowDecay = 0,
    NormalDecay = 1,
    FastDecay = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopElastictransformStrokeHardradiusdecay {
    Off = 0,
    Instant = 1,
    Linear = 2,
    Quadratic = 3,
    Cubic = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopElastictransformInitialdispconstrgrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopElastictransformVisualizefalloff {
    Never = 0,
    Always = 1,
    WhenViewportIsActive = 2,
}

#[derive(Debug, Clone)]
pub struct SopElastictransform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopElastictransform {
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

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_sub_network_input_1_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_poissonratio(mut self, val: f32) -> Self {
        self.params.insert(
            "poissonratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_poissonratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "poissonratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grab_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "grab_strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grab_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grab_strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twist_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "twist_angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twist_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twist_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinch_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "pinch_strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pinch_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinch_strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_movingstep(mut self, val: f32) -> Self {
        self.params.insert(
            "stroke_movingstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_movingstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_movingstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_hardradius(mut self, val: f32) -> Self {
        self.params.insert(
            "stroke_hardradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_hardradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_hardradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "stroke_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialdispconstrradius(mut self, val: f32) -> Self {
        self.params.insert(
            "initialdispconstrradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initialdispconstrradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialdispconstrradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_grab_displacement(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "grab_displacement".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_grab_displacement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grab_displacement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twist_plane(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "twist_plane".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_twist_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twist_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinch_plane(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pinch_plane".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pinch_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinch_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinch_direction(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pinch_direction".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pinch_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinch_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_origin(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "stroke_origin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_origin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_origin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxdispconstr(mut self, val: i32) -> Self {
        self.params.insert(
            "maxdispconstr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxdispconstr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdispconstr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopElastictransformGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_sculptmode(mut self, val: SopElastictransformStrokeSculptmode) -> Self {
        self.params.insert(
            "stroke_sculptmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_sculptmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_sculptmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_decay(mut self, val: SopElastictransformStrokeDecay) -> Self {
        self.params.insert(
            "stroke_decay".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_decay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_decay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_hardradiusdecay(
        mut self,
        val: SopElastictransformStrokeHardradiusdecay,
    ) -> Self {
        self.params.insert(
            "stroke_hardradiusdecay".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_hardradiusdecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_hardradiusdecay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialdispconstrgrouptype(
        mut self,
        val: SopElastictransformInitialdispconstrgrouptype,
    ) -> Self {
        self.params.insert(
            "initialdispconstrgrouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialdispconstrgrouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialdispconstrgrouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualizefalloff(mut self, val: SopElastictransformVisualizefalloff) -> Self {
        self.params.insert(
            "visualizefalloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_visualizefalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visualizefalloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_initialdispconstr(mut self, val: &str) -> Self {
        self.params.insert(
            "initialdispconstr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initialdispconstr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialdispconstr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointsorigin(mut self, val: &str) -> Self {
        self.params.insert(
            "pointsorigin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointsorigin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointsorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_stroke_moving(mut self, val: bool) -> Self {
        self.params.insert(
            "stroke_moving".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stroke_moving_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stroke_moving".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visoffactive(mut self, val: bool) -> Self {
        self.params.insert(
            "visoffactive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visoffactive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visoffactive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopElastictransform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "elastictransform"
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
pub enum SopEndsCloseu {
    NoChange = 0,
    Open = 1,
    CloseStraight = 2,
    CloseRounded = 3,
    UnrollWithNewPoints = 4,
    UnrollWithSharedPoints = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEndsClosev {
    NoChange = 0,
    Open = 1,
    CloseStraight = 2,
    CloseRounded = 3,
    UnrollWithNewPoints = 4,
    UnrollWithSharedPoints = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEndsClampu {
    NoChange = 0,
    Clamp = 1,
    Unclamp = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEndsClampv {
    NoChange = 0,
    Clamp = 1,
    Unclamp = 2,
}

#[derive(Debug, Clone)]
pub struct SopEnds {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEnds {
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

    /// Connects to input 0: "Source Geometry"
    pub fn set_input_source_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Source Geometry" and specifies the output index of the target node.
    pub fn set_input_source_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_source_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_closeu(mut self, val: SopEndsCloseu) -> Self {
        self.params.insert(
            "closeu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_closeu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closev(mut self, val: SopEndsClosev) -> Self {
        self.params.insert(
            "closev".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_closev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clampu(mut self, val: SopEndsClampu) -> Self {
        self.params.insert(
            "clampu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clampu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clampu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clampv(mut self, val: SopEndsClampv) -> Self {
        self.params.insert(
            "clampv".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clampv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clampv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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

    // --- Toggle parameters ---
    pub fn with_pshapeu(mut self, val: bool) -> Self {
        self.params.insert(
            "pshapeu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pshapeu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pshapeu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pshapev(mut self, val: bool) -> Self {
        self.params.insert(
            "pshapev".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pshapev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pshapev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEnds {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ends"
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
pub enum SopEnumerateGrouptype {
    Primitives = 0,
    Points = 1,
    Vertices = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEnumeratePiecemode {
    EnumeratePieceElements = 0,
    EnumeratePieces = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopEnumerateAttribtype {
    Integer = 0,
    String = 1,
}

#[derive(Debug, Clone)]
pub struct SopEnumerate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopEnumerate {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_grouptype(mut self, val: SopEnumerateGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_piecemode(mut self, val: SopEnumeratePiecemode) -> Self {
        self.params.insert(
            "piecemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_piecemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "piecemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribtype(mut self, val: SopEnumerateAttribtype) -> Self {
        self.params.insert(
            "attribtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribname(mut self, val: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_usepieceattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usepieceattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopEnumerate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "enumerate"
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
pub enum SopErrorSeverity {
    Message = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone)]
pub struct SopError {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopError {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_enable_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Int(val),
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

    // --- Menu parameters ---
    pub fn with_severity_inst(mut self, index1: usize, val: SopErrorSeverity) -> Self {
        self.params.insert(
            format!("severity{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_severity_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("severity{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_errormsg_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("errormsg{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_errormsg_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("errormsg{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopError {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "error"
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
pub enum SopExplodedviewClass {
    Primitive = 0,
    Point = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExplodedviewCentroidmethod {
    CenterOfMass = 0,
    BoundingBoxCenter = 1,
}

#[derive(Debug, Clone)]
pub struct SopExplodedview {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExplodedview {
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

    /// Connects to input 0: "Geometry to Explode"
    pub fn set_input_geometry_to_explode<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Geometry to Explode" and specifies the output index of the target node.
    pub fn set_input_geometry_to_explode_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_geometry_to_explode_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Menu parameters ---
    pub fn with_class(mut self, val: SopExplodedviewClass) -> Self {
        self.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_class_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_centroidmethod(mut self, val: SopExplodedviewCentroidmethod) -> Self {
        self.params.insert(
            "centroidmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_centroidmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "centroidmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_overridecenter(mut self, val: bool) -> Self {
        self.params.insert(
            "overridecenter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExplodedview {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "explodedview"
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
pub struct SopExportobjtransforms {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExportobjtransforms {
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

    /// Connects to input 0: "xform"
    pub fn set_input_xform<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "xform" and specifies the output index of the target node.
    pub fn set_input_xform_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_xform_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "xformtarget"
    pub fn set_input_xformtarget<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "xformtarget" and specifies the output index of the target node.
    pub fn set_input_xformtarget_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_xformtarget_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Toggle parameters ---
    pub fn with_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExportobjtransforms {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "exportobjtransforms"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait SopExportobjtransformsInnerExt {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn target(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn transforms(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chopnet1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopExportobjtransformsInnerExt
    for crate::core::graph::InnerGraph<'a, SopExportobjtransforms>
{
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT")
    }
    fn target(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("TARGET")
    }
    fn transforms(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("TRANSFORMS")
    }
    fn chopnet1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("chopnet1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtractcentroidPartitiontype {
    Primitives = 0,
    Pieces = 1,
    Detail = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtractcentroidClass {
    Primitive = 0,
    Point = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtractcentroidMethod {
    CenterOfMass = 0,
    BoundingBoxCenter = 1,
    ConvexHullCenter = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtractcentroidOutput {
    Points = 0,
    Attribute = 1,
}

#[derive(Debug, Clone)]
pub struct SopExtractcentroid {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExtractcentroid {
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

    /// Connects to input 0: "Geometry"
    pub fn set_input_geometry<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Geometry" and specifies the output index of the target node.
    pub fn set_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_partitiontype(mut self, val: SopExtractcentroidPartitiontype) -> Self {
        self.params.insert(
            "partitiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_partitiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_class(mut self, val: SopExtractcentroidClass) -> Self {
        self.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_class_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: SopExtractcentroidMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_output(mut self, val: SopExtractcentroidOutput) -> Self {
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

    // --- String parameters ---
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_centroidattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "centroidattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_centroidattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "centroidattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transferattributes(mut self, val: &str) -> Self {
        self.params.insert(
            "transferattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transferattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transferattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transfergroups(mut self, val: &str) -> Self {
        self.params.insert(
            "transfergroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transfergroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transfergroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExtractcentroid {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extractcentroid"
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
pub enum SopExtractcontoursMode {
    GenerateCurves = 0,
    AddToEdgeGroup = 1,
}

#[derive(Debug, Clone)]
pub struct SopExtractcontours {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExtractcontours {
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

    /// Connects to input 0: "Input Geometry"
    pub fn set_input_input_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_geometry_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_normaltolerance(mut self, val: f32) -> Self {
        self.params.insert(
            "normaltolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaltolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaltolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: SopExtractcontoursMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_campath(mut self, val: &str) -> Self {
        self.params.insert(
            "campath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_campath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "campath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputedgegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "outputedgegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputedgegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputedgegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_frustumculling(mut self, val: bool) -> Self {
        self.params.insert(
            "frustumculling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_frustumculling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frustumculling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExtractcontours {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extractcontours"
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
pub enum SopExtractpointfromcurveCutvaluefrom {
    ConstantValue = 0,
    AttributeValue = 1,
    CurrentTime = 2,
}

#[derive(Debug, Clone)]
pub struct SopExtractpointfromcurve {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExtractpointfromcurve {
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

    /// Connects to input 0: "Curves with Cut Attribute"
    pub fn set_input_curves_with_cut_attribute<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Curves with Cut Attribute" and specifies the output index of the target node.
    pub fn set_input_curves_with_cut_attribute_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_curves_with_cut_attribute_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_cutvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "cutvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cutvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_cutvaluefrom(mut self, val: SopExtractpointfromcurveCutvaluefrom) -> Self {
        self.params.insert(
            "cutvaluefrom".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cutvaluefrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutvaluefrom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_cutattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "cutattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cutattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutvalueattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "cutvalueattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cutvalueattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutvalueattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extractattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "extractattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extractattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extractattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curveuattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "curveuattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curveuattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curveuattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ncutattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "ncutattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ncutattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ncutattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvenumattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "curvenumattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curvenumattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvenumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyprimattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "copyprimattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copyprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyprimattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_docurveuattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "docurveuattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docurveuattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docurveuattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doncutattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "doncutattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doncutattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doncutattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docurvenumattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "docurvenumattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docurvenumattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docurvenumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doattribcopy(mut self, val: bool) -> Self {
        self.params.insert(
            "doattribcopy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doattribcopy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doattribcopy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExtractpointfromcurve {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extractpointfromcurve"
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
pub enum SopExtracttransformPieceattribclass {
    Primitive = 0,
    Point = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtracttransformExtractionmethod {
    TranslationAndRotation = 0,
    /// Translation, Rotation, and Uniform Scale
    TranslationRotationAndUniformScale = 1,
    /// Translation, Rotation, and Non-Uniform Scale
    TranslationRotationAndNonMinusUniformScale = 2,
    FullTransform = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtracttransformOutputattribs {
    StandardInstanceAttributes = 0,
    /// Transform Matrix (3x3) and Translation
    TransformMatrix3x3AndTranslation = 1,
    /// Transform Matrix (4x4)
    TransformMatrix4x4 = 2,
}

#[derive(Debug, Clone)]
pub struct SopExtracttransform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExtracttransform {
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

    /// Connects to input 0: "Reference Geometry"
    pub fn set_input_reference_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Reference Geometry" and specifies the output index of the target node.
    pub fn set_input_reference_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_reference_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Target Geometry"
    pub fn set_input_target_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Target Geometry" and specifies the output index of the target node.
    pub fn set_input_target_geometry_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_target_geometry_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Menu parameters ---
    pub fn with_pieceattribclass(mut self, val: SopExtracttransformPieceattribclass) -> Self {
        self.params.insert(
            "pieceattribclass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pieceattribclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattribclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extractionmethod(mut self, val: SopExtracttransformExtractionmethod) -> Self {
        self.params.insert(
            "extractionmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extractionmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extractionmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputattribs(mut self, val: SopExtracttransformOutputattribs) -> Self {
        self.params.insert(
            "outputattribs".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distortionattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "distortionattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distortionattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distortionattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usepieceattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usepieceattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computedistortion(mut self, val: bool) -> Self {
        self.params.insert(
            "computedistortion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computedistortion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computedistortion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExtracttransform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extracttransform"
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
pub enum SopExtrudeDofuse {
    NoFusion = 0,
    ClampAllPoints = 1,
    ClampMinimalSet = 2,
    ClampIndividualFace = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtrudeFronttype {
    NoOutput = 0,
    OutputFace = 1,
    ConvexFace = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtrudeBacktype {
    NoOutput = 0,
    OutputFace = 1,
    ConvexFace = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopExtrudeSidetype {
    NoOutput = 0,
    Rows = 1,
    Columns = 2,
    RowsAndColumns = 3,
    Triangles = 4,
    Quadrilaterals = 5,
    AlternatingTriangles = 6,
}

#[derive(Debug, Clone)]
pub struct SopExtrude {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExtrude {
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

    /// Connects to input 0: "Primitives To Extrude"
    pub fn set_input_primitives_to_extrude<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Primitives To Extrude" and specifies the output index of the target node.
    pub fn set_input_primitives_to_extrude_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_primitives_to_extrude_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Optional Cross-section"
    pub fn set_input_optional_cross_section<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Optional Cross-section" and specifies the output index of the target node.
    pub fn set_input_optional_cross_section_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_optional_cross_section_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Button parameters ---
    pub fn trigger_initextrude(mut self) -> Self {
        self.params.insert(
            "initExtrude".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_thickxlate(mut self, val: f32) -> Self {
        self.params.insert(
            "thickxlate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickxlate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickxlate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thickscale(mut self, val: f32) -> Self {
        self.params.insert(
            "thickscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthxlate(mut self, val: f32) -> Self {
        self.params.insert(
            "depthxlate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthxlate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthxlate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthscale(mut self, val: f32) -> Self {
        self.params.insert(
            "depthscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_vertex(mut self, val: i32) -> Self {
        self.params.insert(
            "vertex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vertex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vertex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_dofuse(mut self, val: SopExtrudeDofuse) -> Self {
        self.params.insert(
            "doFuse".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dofuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doFuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fronttype(mut self, val: SopExtrudeFronttype) -> Self {
        self.params.insert(
            "frontType".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fronttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frontType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_backtype(mut self, val: SopExtrudeBacktype) -> Self {
        self.params.insert(
            "backType".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_backtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "backType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidetype(mut self, val: SopExtrudeSidetype) -> Self {
        self.params.insert(
            "sideType".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sidetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sideType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcegrp(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceGrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceGrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xsectiongrp(mut self, val: &str) -> Self {
        self.params.insert(
            "xsectionGrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xsectiongrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xsectionGrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frontgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "frontgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_frontgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frontgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_backgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "backgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_backgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "backgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidegrp(mut self, val: &str) -> Self {
        self.params.insert(
            "sidegrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sidegrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sidegrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_polys(mut self, val: bool) -> Self {
        self.params.insert(
            "polys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docusp(mut self, val: bool) -> Self {
        self.params.insert(
            "doCusp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docusp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doCusp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharefaces(mut self, val: bool) -> Self {
        self.params.insert(
            "shareFaces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sharefaces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shareFaces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removesharedsides(mut self, val: bool) -> Self {
        self.params.insert(
            "removeSharedSides".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removesharedsides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removeSharedSides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dohole(mut self, val: bool) -> Self {
        self.params.insert(
            "dohole".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dohole_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dohole".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeunusedpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "removeunusedpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeunusedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removeunusedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newg(mut self, val: bool) -> Self {
        self.params.insert(
            "newg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_newg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExtrude {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extrude"
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
pub struct SopExtrudevolume {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopExtrudevolume {
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

    /// Connects to input 0: "Polygons to Extrude into a Solid Volume"
    pub fn set_input_polygons_to_extrude_into_a_solid_volume<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Polygons to Extrude into a Solid Volume" and specifies the output index of the target node.
    pub fn set_input_polygons_to_extrude_into_a_solid_volume_from<
        N: crate::core::types::HoudiniNode,
    >(
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

    pub fn set_input_polygons_to_extrude_into_a_solid_volume_by_name<
        N: crate::core::types::HoudiniNode,
    >(
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
    pub fn with_depth(mut self, val: f32) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basepadding(mut self, val: f32) -> Self {
        self.params.insert(
            "basepadding".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basepadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basepadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baselift(mut self, val: f32) -> Self {
        self.params.insert(
            "baselift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_baselift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baselift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_basenormal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "basenormal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basenormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_topgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "topgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basegrp(mut self, val: &str) -> Self {
        self.params.insert(
            "basegrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basegrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basegrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidegrp(mut self, val: &str) -> Self {
        self.params.insert(
            "sidegrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sidegrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sidegrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topseamgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "topseamgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topseamgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topseamgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baseseamgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "baseseamgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_baseseamgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseseamgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_flat(mut self, val: bool) -> Self {
        self.params.insert(
            "flat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtopgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtopgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtopgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtopgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputbasegrp(mut self, val: bool) -> Self {
        self.params.insert(
            "outputbasegrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputbasegrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputbasegrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputsidegrp(mut self, val: bool) -> Self {
        self.params.insert(
            "outputsidegrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputsidegrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputsidegrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtopseamgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtopseamgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtopseamgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtopseamgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputbaseseamgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "outputbaseseamgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputbaseseamgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputbaseseamgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopExtrudevolume {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extrudevolume"
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
