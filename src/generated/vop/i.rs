#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopIfCondition {
    /// True ( != 0 )
    True0 = 0,
    /// False ( == 0 )
    False0 = 1,
}

#[derive(Debug, Clone)]
pub struct VopIf {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIf {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_condition(mut self, val: VopIfCondition) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIf {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "if"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIfconnected {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIfconnected {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_notconnected(mut self, val: f32) -> Self {
        self.params.insert(
            "notconnected".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_notconnected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_uf(mut self, val: f32) -> Self {
        self.params.insert(
            "notconnected_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_notconnected_uf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_notconnected_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "notconnected_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_notconnected_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_notconnected_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_p".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_n".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_n".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_c(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_c".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_c".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_un(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_un".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_un_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_un".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_uc(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "notconnected_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_notconnected_uc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_notconnected_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "notconnected_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_notconnected_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_notconnected_m3(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "notconnected_m3".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_notconnected_m3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_m3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_m(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "notconnected_m".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_notconnected_m_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_m".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_notconnected_um(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "notconnected_um".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_notconnected_um_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_um".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_notconnected_i(mut self, val: i32) -> Self {
        self.params.insert(
            "notconnected_i".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_notconnected_i_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_i".to_string(),
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
    pub fn with_notconnected_s(mut self, val: &str) -> Self {
        self.params.insert(
            "notconnected_s".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_notconnected_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notconnected_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIfconnected {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ifconnected"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIlluminance {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIlluminance {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for VopIlluminance {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "illuminance"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImage3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImage3d {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopImage3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "image3d"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
#[allow(clippy::wrong_self_convention)]
pub trait VopImage3dInnerExt {
    fn global1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> VopImage3dInnerExt for crate::core::graph::InnerGraph<'a> {
    fn global1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("global1")
    }
    fn output1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("output1")
    }
}

#[derive(Debug, Clone)]
pub struct VopImage3dvolume {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImage3dvolume {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
}

impl crate::core::types::HoudiniNode for VopImage3dvolume {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "image3dvolume"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImportanceRemap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImportanceRemap {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_maxres(mut self, val: i32) -> Self {
        self.params.insert(
            "maxres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_map(mut self, val: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopImportanceRemap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "importance_remap"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImportattrib {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImportattrib {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_input_index(mut self, val: i32) -> Self {
        self.params.insert(
            "input_index".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input_index".to_string(),
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
}

impl crate::core::types::HoudiniNode for VopImportattrib {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "importattrib"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImportdetail {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImportdetail {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
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
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
}

impl crate::core::types::HoudiniNode for VopImportdetail {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "importdetail"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImportpoint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImportpoint {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
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
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
}

impl crate::core::types::HoudiniNode for VopImportpoint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "importpoint"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImportprim {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImportprim {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
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
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
}

impl crate::core::types::HoudiniNode for VopImportprim {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "importprim"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopImportvertex {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopImportvertex {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
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
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
}

impl crate::core::types::HoudiniNode for VopImportvertex {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "importvertex"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIndirectlighting {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIndirectlighting {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_samplingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "samplingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samplingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_raystyle(mut self, val: &str) -> Self {
        self.params.insert(
            "raystyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_raystyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raystyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doscope(mut self, val: bool) -> Self {
        self.params.insert(
            "doscope".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxdist(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxdist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIndirectlighting {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "indirectlighting"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIngroup {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIngroup {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_ptnum(mut self, val: i32) -> Self {
        self.params.insert(
            "ptnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ptnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptnum".to_string(),
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
}

impl crate::core::types::HoudiniNode for VopIngroup {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ingroup"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopInline {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopInline {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_code(mut self, val: &str) -> Self {
        self.params.insert(
            "code".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_code_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "code".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_includes(mut self, val: &str) -> Self {
        self.params.insert(
            "includes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_includes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outercode(mut self, val: &str) -> Self {
        self.params.insert(
            "outercode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outercode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outercode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name1(mut self, val: &str) -> Self {
        self.params.insert(
            "name1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name2(mut self, val: &str) -> Self {
        self.params.insert(
            "name2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name3(mut self, val: &str) -> Self {
        self.params.insert(
            "name3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name4(mut self, val: &str) -> Self {
        self.params.insert(
            "name4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name5(mut self, val: &str) -> Self {
        self.params.insert(
            "name5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label5(mut self, val: &str) -> Self {
        self.params.insert(
            "label5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name6(mut self, val: &str) -> Self {
        self.params.insert(
            "name6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label6(mut self, val: &str) -> Self {
        self.params.insert(
            "label6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name7(mut self, val: &str) -> Self {
        self.params.insert(
            "name7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label7(mut self, val: &str) -> Self {
        self.params.insert(
            "label7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name8(mut self, val: &str) -> Self {
        self.params.insert(
            "name8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label8(mut self, val: &str) -> Self {
        self.params.insert(
            "label8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name9(mut self, val: &str) -> Self {
        self.params.insert(
            "name9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label9(mut self, val: &str) -> Self {
        self.params.insert(
            "label9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name10(mut self, val: &str) -> Self {
        self.params.insert(
            "name10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label10(mut self, val: &str) -> Self {
        self.params.insert(
            "label10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name11(mut self, val: &str) -> Self {
        self.params.insert(
            "name11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label11(mut self, val: &str) -> Self {
        self.params.insert(
            "label11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name12(mut self, val: &str) -> Self {
        self.params.insert(
            "name12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label12(mut self, val: &str) -> Self {
        self.params.insert(
            "label12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name13(mut self, val: &str) -> Self {
        self.params.insert(
            "name13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label13(mut self, val: &str) -> Self {
        self.params.insert(
            "label13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name14(mut self, val: &str) -> Self {
        self.params.insert(
            "name14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label14(mut self, val: &str) -> Self {
        self.params.insert(
            "label14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name15(mut self, val: &str) -> Self {
        self.params.insert(
            "name15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label15(mut self, val: &str) -> Self {
        self.params.insert(
            "label15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name16(mut self, val: &str) -> Self {
        self.params.insert(
            "name16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label16(mut self, val: &str) -> Self {
        self.params.insert(
            "label16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name17(mut self, val: &str) -> Self {
        self.params.insert(
            "name17".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name17_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label17(mut self, val: &str) -> Self {
        self.params.insert(
            "label17".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label17_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name18(mut self, val: &str) -> Self {
        self.params.insert(
            "name18".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name18_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label18(mut self, val: &str) -> Self {
        self.params.insert(
            "label18".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label18_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name19(mut self, val: &str) -> Self {
        self.params.insert(
            "name19".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name19_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label19(mut self, val: &str) -> Self {
        self.params.insert(
            "label19".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label19_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name20(mut self, val: &str) -> Self {
        self.params.insert(
            "name20".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name20_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label20(mut self, val: &str) -> Self {
        self.params.insert(
            "label20".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label20_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name21(mut self, val: &str) -> Self {
        self.params.insert(
            "name21".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name21_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label21(mut self, val: &str) -> Self {
        self.params.insert(
            "label21".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label21_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name22(mut self, val: &str) -> Self {
        self.params.insert(
            "name22".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name22_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label22(mut self, val: &str) -> Self {
        self.params.insert(
            "label22".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label22_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name23(mut self, val: &str) -> Self {
        self.params.insert(
            "name23".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name23_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label23(mut self, val: &str) -> Self {
        self.params.insert(
            "label23".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label23_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name24(mut self, val: &str) -> Self {
        self.params.insert(
            "name24".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name24_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label24(mut self, val: &str) -> Self {
        self.params.insert(
            "label24".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label24_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name25(mut self, val: &str) -> Self {
        self.params.insert(
            "name25".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name25_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label25(mut self, val: &str) -> Self {
        self.params.insert(
            "label25".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label25_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name26(mut self, val: &str) -> Self {
        self.params.insert(
            "name26".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name26_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label26(mut self, val: &str) -> Self {
        self.params.insert(
            "label26".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label26_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name27(mut self, val: &str) -> Self {
        self.params.insert(
            "name27".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name27_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label27(mut self, val: &str) -> Self {
        self.params.insert(
            "label27".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label27_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name28(mut self, val: &str) -> Self {
        self.params.insert(
            "name28".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name28_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label28(mut self, val: &str) -> Self {
        self.params.insert(
            "label28".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label28_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name29(mut self, val: &str) -> Self {
        self.params.insert(
            "name29".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name29_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label29(mut self, val: &str) -> Self {
        self.params.insert(
            "label29".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label29_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name30(mut self, val: &str) -> Self {
        self.params.insert(
            "name30".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name30_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label30(mut self, val: &str) -> Self {
        self.params.insert(
            "label30".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label30_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name31(mut self, val: &str) -> Self {
        self.params.insert(
            "name31".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name31_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label31(mut self, val: &str) -> Self {
        self.params.insert(
            "label31".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label31_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name32(mut self, val: &str) -> Self {
        self.params.insert(
            "name32".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name32_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label32(mut self, val: &str) -> Self {
        self.params.insert(
            "label32".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label32_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name33(mut self, val: &str) -> Self {
        self.params.insert(
            "name33".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name33_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name33".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label33(mut self, val: &str) -> Self {
        self.params.insert(
            "label33".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label33_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label33".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name34(mut self, val: &str) -> Self {
        self.params.insert(
            "name34".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name34_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name34".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label34(mut self, val: &str) -> Self {
        self.params.insert(
            "label34".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label34_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label34".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name35(mut self, val: &str) -> Self {
        self.params.insert(
            "name35".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name35_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name35".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label35(mut self, val: &str) -> Self {
        self.params.insert(
            "label35".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label35_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label35".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name36(mut self, val: &str) -> Self {
        self.params.insert(
            "name36".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name36_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name36".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label36(mut self, val: &str) -> Self {
        self.params.insert(
            "label36".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label36_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label36".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name37(mut self, val: &str) -> Self {
        self.params.insert(
            "name37".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name37_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name37".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label37(mut self, val: &str) -> Self {
        self.params.insert(
            "label37".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label37_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label37".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name38(mut self, val: &str) -> Self {
        self.params.insert(
            "name38".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name38_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name38".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label38(mut self, val: &str) -> Self {
        self.params.insert(
            "label38".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label38_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label38".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name39(mut self, val: &str) -> Self {
        self.params.insert(
            "name39".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name39_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name39".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label39(mut self, val: &str) -> Self {
        self.params.insert(
            "label39".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label39_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label39".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name40(mut self, val: &str) -> Self {
        self.params.insert(
            "name40".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name40_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name40".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label40(mut self, val: &str) -> Self {
        self.params.insert(
            "label40".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label40_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label40".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name41(mut self, val: &str) -> Self {
        self.params.insert(
            "name41".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name41_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name41".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label41(mut self, val: &str) -> Self {
        self.params.insert(
            "label41".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label41_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label41".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name42(mut self, val: &str) -> Self {
        self.params.insert(
            "name42".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name42_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name42".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label42(mut self, val: &str) -> Self {
        self.params.insert(
            "label42".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label42_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label42".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name43(mut self, val: &str) -> Self {
        self.params.insert(
            "name43".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name43_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name43".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label43(mut self, val: &str) -> Self {
        self.params.insert(
            "label43".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label43_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label43".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name44(mut self, val: &str) -> Self {
        self.params.insert(
            "name44".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name44_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name44".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label44(mut self, val: &str) -> Self {
        self.params.insert(
            "label44".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label44_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label44".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name45(mut self, val: &str) -> Self {
        self.params.insert(
            "name45".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name45_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name45".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label45(mut self, val: &str) -> Self {
        self.params.insert(
            "label45".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label45_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label45".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name46(mut self, val: &str) -> Self {
        self.params.insert(
            "name46".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name46_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name46".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label46(mut self, val: &str) -> Self {
        self.params.insert(
            "label46".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label46_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label46".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name47(mut self, val: &str) -> Self {
        self.params.insert(
            "name47".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name47_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name47".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label47(mut self, val: &str) -> Self {
        self.params.insert(
            "label47".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label47_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label47".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name48(mut self, val: &str) -> Self {
        self.params.insert(
            "name48".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name48_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name48".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label48(mut self, val: &str) -> Self {
        self.params.insert(
            "label48".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label48_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label48".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name49(mut self, val: &str) -> Self {
        self.params.insert(
            "name49".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name49_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name49".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label49(mut self, val: &str) -> Self {
        self.params.insert(
            "label49".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label49_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label49".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name50(mut self, val: &str) -> Self {
        self.params.insert(
            "name50".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name50_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name50".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label50(mut self, val: &str) -> Self {
        self.params.insert(
            "label50".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label50_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label50".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name51(mut self, val: &str) -> Self {
        self.params.insert(
            "name51".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name51_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name51".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label51(mut self, val: &str) -> Self {
        self.params.insert(
            "label51".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label51_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label51".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name52(mut self, val: &str) -> Self {
        self.params.insert(
            "name52".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name52_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name52".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label52(mut self, val: &str) -> Self {
        self.params.insert(
            "label52".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label52_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label52".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name53(mut self, val: &str) -> Self {
        self.params.insert(
            "name53".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name53_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name53".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label53(mut self, val: &str) -> Self {
        self.params.insert(
            "label53".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label53_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label53".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name54(mut self, val: &str) -> Self {
        self.params.insert(
            "name54".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name54_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name54".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label54(mut self, val: &str) -> Self {
        self.params.insert(
            "label54".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label54_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label54".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name55(mut self, val: &str) -> Self {
        self.params.insert(
            "name55".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name55_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name55".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label55(mut self, val: &str) -> Self {
        self.params.insert(
            "label55".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label55_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label55".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name56(mut self, val: &str) -> Self {
        self.params.insert(
            "name56".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name56_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name56".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label56(mut self, val: &str) -> Self {
        self.params.insert(
            "label56".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label56_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label56".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name57(mut self, val: &str) -> Self {
        self.params.insert(
            "name57".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name57_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name57".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label57(mut self, val: &str) -> Self {
        self.params.insert(
            "label57".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label57_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label57".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name58(mut self, val: &str) -> Self {
        self.params.insert(
            "name58".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name58_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name58".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label58(mut self, val: &str) -> Self {
        self.params.insert(
            "label58".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label58_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label58".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name59(mut self, val: &str) -> Self {
        self.params.insert(
            "name59".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name59_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name59".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label59(mut self, val: &str) -> Self {
        self.params.insert(
            "label59".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label59_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label59".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name60(mut self, val: &str) -> Self {
        self.params.insert(
            "name60".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name60_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name60".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label60(mut self, val: &str) -> Self {
        self.params.insert(
            "label60".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label60_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label60".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name61(mut self, val: &str) -> Self {
        self.params.insert(
            "name61".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name61_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name61".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label61(mut self, val: &str) -> Self {
        self.params.insert(
            "label61".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label61_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label61".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name62(mut self, val: &str) -> Self {
        self.params.insert(
            "name62".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name62_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name62".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label62(mut self, val: &str) -> Self {
        self.params.insert(
            "label62".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label62_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label62".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name63(mut self, val: &str) -> Self {
        self.params.insert(
            "name63".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name63_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name63".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label63(mut self, val: &str) -> Self {
        self.params.insert(
            "label63".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label63_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label63".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name64(mut self, val: &str) -> Self {
        self.params.insert(
            "name64".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name64_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name64".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label64(mut self, val: &str) -> Self {
        self.params.insert(
            "label64".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label64_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label64".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype1(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname1(mut self, val: &str) -> Self {
        self.params.insert(
            "outname1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel1(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype2(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname2(mut self, val: &str) -> Self {
        self.params.insert(
            "outname2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel2(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype3(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname3(mut self, val: &str) -> Self {
        self.params.insert(
            "outname3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel3(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype4(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname4(mut self, val: &str) -> Self {
        self.params.insert(
            "outname4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel4(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype5(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname5(mut self, val: &str) -> Self {
        self.params.insert(
            "outname5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel5(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype6(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname6(mut self, val: &str) -> Self {
        self.params.insert(
            "outname6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel6(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype7(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname7(mut self, val: &str) -> Self {
        self.params.insert(
            "outname7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel7(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype8(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname8(mut self, val: &str) -> Self {
        self.params.insert(
            "outname8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel8(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype9(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname9(mut self, val: &str) -> Self {
        self.params.insert(
            "outname9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel9(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype10(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname10(mut self, val: &str) -> Self {
        self.params.insert(
            "outname10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel10(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype11(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname11(mut self, val: &str) -> Self {
        self.params.insert(
            "outname11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel11(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype12(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname12(mut self, val: &str) -> Self {
        self.params.insert(
            "outname12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel12(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype13(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname13(mut self, val: &str) -> Self {
        self.params.insert(
            "outname13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel13(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype14(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname14(mut self, val: &str) -> Self {
        self.params.insert(
            "outname14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel14(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype15(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname15(mut self, val: &str) -> Self {
        self.params.insert(
            "outname15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel15(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype16(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname16(mut self, val: &str) -> Self {
        self.params.insert(
            "outname16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel16(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype17(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype17".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype17_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname17(mut self, val: &str) -> Self {
        self.params.insert(
            "outname17".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname17_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel17(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel17".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel17_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype18(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype18".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype18_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname18(mut self, val: &str) -> Self {
        self.params.insert(
            "outname18".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname18_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel18(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel18".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel18_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype19(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype19".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype19_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname19(mut self, val: &str) -> Self {
        self.params.insert(
            "outname19".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname19_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel19(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel19".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel19_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype20(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype20".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype20_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname20(mut self, val: &str) -> Self {
        self.params.insert(
            "outname20".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname20_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel20(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel20".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel20_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype21(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype21".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype21_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname21(mut self, val: &str) -> Self {
        self.params.insert(
            "outname21".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname21_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel21(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel21".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel21_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype22(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype22".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype22_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname22(mut self, val: &str) -> Self {
        self.params.insert(
            "outname22".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname22_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel22(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel22".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel22_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype23(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype23".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype23_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname23(mut self, val: &str) -> Self {
        self.params.insert(
            "outname23".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname23_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel23(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel23".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel23_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype24(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype24".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype24_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname24(mut self, val: &str) -> Self {
        self.params.insert(
            "outname24".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname24_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel24(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel24".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel24_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype25(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype25".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype25_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname25(mut self, val: &str) -> Self {
        self.params.insert(
            "outname25".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname25_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel25(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel25".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel25_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype26(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype26".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype26_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname26(mut self, val: &str) -> Self {
        self.params.insert(
            "outname26".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname26_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel26(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel26".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel26_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype27(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype27".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype27_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname27(mut self, val: &str) -> Self {
        self.params.insert(
            "outname27".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname27_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel27(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel27".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel27_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype28(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype28".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype28_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname28(mut self, val: &str) -> Self {
        self.params.insert(
            "outname28".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname28_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel28(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel28".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel28_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype29(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype29".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype29_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname29(mut self, val: &str) -> Self {
        self.params.insert(
            "outname29".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname29_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel29(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel29".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel29_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype30(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype30".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype30_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname30(mut self, val: &str) -> Self {
        self.params.insert(
            "outname30".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname30_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel30(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel30".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel30_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype31(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype31".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype31_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname31(mut self, val: &str) -> Self {
        self.params.insert(
            "outname31".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname31_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel31(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel31".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel31_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtype32(mut self, val: &str) -> Self {
        self.params.insert(
            "outtype32".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtype32_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outtype32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outname32(mut self, val: &str) -> Self {
        self.params.insert(
            "outname32".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outname32_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outname32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlabel32(mut self, val: &str) -> Self {
        self.params.insert(
            "outlabel32".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outlabel32_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlabel32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_expandcode(mut self, val: bool) -> Self {
        self.params.insert(
            "expandcode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopInline {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "inline"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopInprimorptgroup {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopInprimorptgroup {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_primorpt(mut self, val: i32) -> Self {
        self.params.insert(
            "primorpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primorpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primorpt".to_string(),
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
    pub fn with_class(mut self, val: &str) -> Self {
        self.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
}

impl crate::core::types::HoudiniNode for VopInprimorptgroup {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "inprimorptgroup"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopInsert {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopInsert {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_value(mut self, val: f32) -> Self {
        self.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_value_s5(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "value_s5".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_value_s5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_value_s4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "value_s4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value_s4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_value_s3(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "value_s3".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_value_s3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_s6(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "value_s6".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_value_s6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_value_s7(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "value_s7".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_value_s7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_s8(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "value_s8".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_value_s8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_s1(mut self, val: i32) -> Self {
        self.params.insert(
            "value_s1".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_value_s1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s1".to_string(),
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
    pub fn with_value_s2(mut self, val: &str) -> Self {
        self.params.insert(
            "value_s2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_value_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_s9(mut self, val: &str) -> Self {
        self.params.insert(
            "value_s9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_value_s9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_s9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopInsert {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "insert"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIntegrate3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIntegrate3d {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accuracy(mut self, val: f32) -> Self {
        self.params.insert(
            "accuracy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accuracy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accuracy".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_startpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "startpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_startpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "endpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_endpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endpos".to_string(),
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
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_image3d(mut self, val: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channel(mut self, val: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIntegrate3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "integrate3d"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIntegrate3dclip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIntegrate3dclip {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limit(mut self, val: f32) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accuracy(mut self, val: f32) -> Self {
        self.params.insert(
            "accuracy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accuracy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accuracy".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_startpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "startpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_startpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "endpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_endpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_image3d(mut self, val: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channel(mut self, val: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIntegrate3dclip {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "integrate3dclip"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIntersect {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIntersect {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_rayorig(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rayorig".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rayorig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayorig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raydir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "raydir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_raydir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raydir".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_farthest(mut self, val: bool) -> Self {
        self.params.insert(
            "farthest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_farthest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "farthest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIntersect {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "intersect"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIntersect3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIntersect3d {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_startpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "startpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_startpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "endpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_endpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_image3d(mut self, val: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channel(mut self, val: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIntersect3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "intersect3d"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIntersectall {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIntersectall {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_rayorig(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rayorig".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rayorig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayorig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raydir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "raydir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_raydir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raydir".to_string(),
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
}

impl crate::core::types::HoudiniNode for VopIntersectall {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "intersectall"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopInttofloat {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopInttofloat {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_ival(mut self, val: i32) -> Self {
        self.params
            .insert("ival".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_ival_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ival".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopInttofloat {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "inttofloat"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopInttovec {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopInttovec {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_int1(mut self, val: i32) -> Self {
        self.params
            .insert("int1".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_int1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "int1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int2(mut self, val: i32) -> Self {
        self.params
            .insert("int2".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_int2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "int2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int3(mut self, val: i32) -> Self {
        self.params
            .insert("int3".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_int3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "int3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopInttovec {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "inttovec"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopInvert {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopInvert {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val_f(mut self, val: f32) -> Self {
        self.params.insert(
            "val_f".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_val_v2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "val_v2".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_val_v2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_v2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_val_v3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "val_v3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_v3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_v3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_val_m2(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "val_m2".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_val_m2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_m2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "val_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_val_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_val(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_m3(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "val_m3".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_val_m3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val_m3".to_string(),
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

impl crate::core::types::HoudiniNode for VopInvert {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "invert"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIrradiance {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIrradiance {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_envtint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "envtint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_envtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envtint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bgcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bgcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bgcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envobj(mut self, val: &str) -> Self {
        self.params.insert(
            "envobj".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envobj".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_categories(mut self, val: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_categories_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIrradiance {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "irradiance"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsalpha {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsalpha {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_signature(mut self, val: f32) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- String parameters ---
    pub fn with_string(mut self, val: &str) -> Self {
        self.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIsalpha {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isalpha"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsconnected {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsconnected {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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

impl crate::core::types::HoudiniNode for VopIsconnected {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isconnected"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsdigit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsdigit {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_signature(mut self, val: f32) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- String parameters ---
    pub fn with_string(mut self, val: &str) -> Self {
        self.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIsdigit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isdigit"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsfinite {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsfinite {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val(mut self, val: f32) -> Self {
        self.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIsfinite {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isfinite"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsfog {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsfog {
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
}

impl crate::core::types::HoudiniNode for VopIsfog {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isfog"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsfrontface {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsfrontface {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for VopIsfrontface {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isfrontface"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsnan {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsnan {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val(mut self, val: f32) -> Self {
        self.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopIsnan {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isnan"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct VopIsshadow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopIsshadow {
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
}

impl crate::core::types::HoudiniNode for VopIsshadow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "isshadow"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
