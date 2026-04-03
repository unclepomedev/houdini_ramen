#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUnixFormat {
    Ascii = 0,
    Binary = 1,
}

#[derive(Debug, Clone)]
pub struct SopUnix {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnix {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_forcecook(mut self) -> Self {
        self.params.insert("forcecook".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Menu parameters ---
    pub fn with_format(mut self, val: SopUnixFormat) -> Self {
        self.params.insert("format".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_format_expr(mut self, expr: &str) -> Self {
        self.params.insert("format".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_cmd(mut self, val: &str) -> Self {
        self.params.insert("cmd".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cmd_expr(mut self, expr: &str) -> Self {
        self.params.insert("cmd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUnix {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unix"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUnpackDetailAttributes {
    PromotionToPrimitiveIfDifferent = 0,
    PromoteToPrimitiveAttributes = 1,
    PromoteToPointAttributes = 2,
    PromoteToVertexAttributes = 3,
}

#[derive(Debug, Clone)]
pub struct SopUnpack {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnpack {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_detail_attributes(mut self, val: SopUnpackDetailAttributes) -> Self {
        self.params.insert("detail_attributes".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_detail_attributes_expr(mut self, expr: &str) -> Self {
        self.params.insert("detail_attributes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_transfer_attributes(mut self, val: &str) -> Self {
        self.params.insert("transfer_attributes".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_transfer_attributes_expr(mut self, expr: &str) -> Self {
        self.params.insert("transfer_attributes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_transfer_groups(mut self, val: &str) -> Self {
        self.params.insert("transfer_groups".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_transfer_groups_expr(mut self, expr: &str) -> Self {
        self.params.insert("transfer_groups".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scene_style_sheet(mut self, val: &str) -> Self {
        self.params.insert("scene_style_sheet".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scene_style_sheet_expr(mut self, expr: &str) -> Self {
        self.params.insert("scene_style_sheet".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_obj_style_sheet(mut self, val: &str) -> Self {
        self.params.insert("obj_style_sheet".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_obj_style_sheet_expr(mut self, expr: &str) -> Self {
        self.params.insert("obj_style_sheet".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_limit_iterations(mut self, val: bool) -> Self {
        self.params.insert("limit_iterations".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_limit_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("limit_iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_apply_style_sheets(mut self, val: bool) -> Self {
        self.params.insert("apply_style_sheets".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_apply_style_sheets_expr(mut self, expr: &str) -> Self {
        self.params.insert("apply_style_sheets".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dotransform(mut self, val: bool) -> Self {
        self.params.insert("dotransform".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dotransform_expr(mut self, expr: &str) -> Self {
        self.params.insert("dotransform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_convertpolysoup(mut self, val: bool) -> Self {
        self.params.insert("convertpolysoup".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_convertpolysoup_expr(mut self, expr: &str) -> Self {
        self.params.insert("convertpolysoup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUnpack {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unpack"
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
pub struct SopUnpackfolder {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnpackfolder {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Packed Folders"
    pub fn set_input_packed_folders<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Packed Folders" and specifies the output index of the target node.
    pub fn set_input_packed_folders_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- String parameters ---
    pub fn with_pattern(mut self, val: &str) -> Self {
        self.params.insert("pattern".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pattern_expr(mut self, expr: &str) -> Self {
        self.params.insert("pattern".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_unpack(mut self, val: bool) -> Self {
        self.params.insert("unpack".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_unpack_expr(mut self, expr: &str) -> Self {
        self.params.insert("unpack".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_invert(mut self, val: bool) -> Self {
        self.params.insert("invert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert("invert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUnpackfolder {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unpackfolder"
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
pub struct SopUnpackgroom {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnpackgroom {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Packed Groom"
    pub fn set_input_packed_groom<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Packed Groom" and specifies the output index of the target node.
    pub fn set_input_packed_groom_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

}

impl crate::core::types::HoudiniNode for SopUnpackgroom {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unpackgroom"
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
pub struct SopUnpackpoints {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnpackpoints {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Primitives to Unpack"
    pub fn set_input_primitives_to_unpack<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Primitives to Unpack" and specifies the output index of the target node.
    pub fn set_input_primitives_to_unpack_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Regions of Interest"
    pub fn set_input_regions_of_interest<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Regions of Interest" and specifies the output index of the target node.
    pub fn set_input_regions_of_interest_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_fineculling(mut self, val: bool) -> Self {
        self.params.insert("fineculling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fineculling_expr(mut self, expr: &str) -> Self {
        self.params.insert("fineculling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usebbox(mut self, val: bool) -> Self {
        self.params.insert("usebbox".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usebbox_expr(mut self, expr: &str) -> Self {
        self.params.insert("usebbox".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUnpackpoints {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unpackpoints"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUnpackusdOutput {
    PackedPrims = 0,
    Polygons = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUnpackusdPivot {
    Origin = 0,
    Centroid = 1,
}

#[derive(Debug, Clone)]
pub struct SopUnpackusd {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnpackusd {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Packed USD Primitives"
    pub fn set_input_packed_usd_primitives<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Packed USD Primitives" and specifies the output index of the target node.
    pub fn set_input_packed_usd_primitives_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: SopUnpackusdOutput) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pivot(mut self, val: SopUnpackusdPivot) -> Self {
        self.params.insert("pivot".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert("pivot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unpacktraversal(mut self, val: &str) -> Self {
        self.params.insert("unpacktraversal".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unpacktraversal_expr(mut self, expr: &str) -> Self {
        self.params.insert("unpacktraversal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathattrib(mut self, val: &str) -> Self {
        self.params.insert("pathattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nameattrib(mut self, val: &str) -> Self {
        self.params.insert("nameattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("nameattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filepathattrib(mut self, val: &str) -> Self {
        self.params.insert("filepathattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filepathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("filepathattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_instancelevelattrib(mut self, val: &str) -> Self {
        self.params.insert("instancelevelattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_instancelevelattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("instancelevelattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_transferattributes(mut self, val: &str) -> Self {
        self.params.insert("transferattributes".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_transferattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert("transferattributes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_transfergroups(mut self, val: &str) -> Self {
        self.params.insert("transfergroups".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_transfergroups_expr(mut self, expr: &str) -> Self {
        self.params.insert("transfergroups".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_importprimvars(mut self, val: &str) -> Self {
        self.params.insert("importprimvars".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_importprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert("importprimvars".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_importattributes(mut self, val: &str) -> Self {
        self.params.insert("importattributes".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_importattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert("importattributes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nontransformingprimvars(mut self, val: &str) -> Self {
        self.params.insert("nontransformingprimvars".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nontransformingprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert("nontransformingprimvars".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_deleteorig(mut self, val: bool) -> Self {
        self.params.insert("deleteorig".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_deleteorig_expr(mut self, expr: &str) -> Self {
        self.params.insert("deleteorig".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limititerations(mut self, val: bool) -> Self {
        self.params.insert("limititerations".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_limititerations_expr(mut self, expr: &str) -> Self {
        self.params.insert("limititerations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addpathattrib(mut self, val: bool) -> Self {
        self.params.insert("addpathattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addpathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("addpathattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addnameattrib(mut self, val: bool) -> Self {
        self.params.insert("addnameattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addnameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("addnameattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addfilepathattrib(mut self, val: bool) -> Self {
        self.params.insert("addfilepathattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addfilepathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("addfilepathattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addinstancelevelattrib(mut self, val: bool) -> Self {
        self.params.insert("addinstancelevelattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addinstancelevelattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("addinstancelevelattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_importinheritedprimvars(mut self, val: bool) -> Self {
        self.params.insert("importinheritedprimvars".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_importinheritedprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert("importinheritedprimvars".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_importcomputedvisibility(mut self, val: bool) -> Self {
        self.params.insert("importcomputedvisibility".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_importcomputedvisibility_expr(mut self, expr: &str) -> Self {
        self.params.insert("importcomputedvisibility".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_translatesttouv(mut self, val: bool) -> Self {
        self.params.insert("translatesttouv".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_translatesttouv_expr(mut self, expr: &str) -> Self {
        self.params.insert("translatesttouv".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUnpackusd {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unpackusd"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUnsubdivideAlgorithm {
    /// Catmull-Clark
    CatmullMinusClark = 0,
    Bilinear = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUnsubdivideStrategy {
    Heuristics = 0,
    ExplicitSeed = 1,
}

#[derive(Debug, Clone)]
pub struct SopUnsubdivide {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUnsubdivide {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_tol(mut self, val: f32) -> Self {
        self.params.insert("tol".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert("tol".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_connectivitytol(mut self, val: f32) -> Self {
        self.params.insert("connectivitytol".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_connectivitytol_expr(mut self, expr: &str) -> Self {
        self.params.insert("connectivitytol".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_algorithm(mut self, val: SopUnsubdivideAlgorithm) -> Self {
        self.params.insert("algorithm".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_algorithm_expr(mut self, expr: &str) -> Self {
        self.params.insert("algorithm".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strategy(mut self, val: SopUnsubdivideStrategy) -> Self {
        self.params.insert("strategy".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strategy_expr(mut self, expr: &str) -> Self {
        self.params.insert("strategy".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert("pieceattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("pieceattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_connectivityattribute(mut self, val: &str) -> Self {
        self.params.insert("connectivityattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_connectivityattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("connectivityattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputdepthattribute(mut self, val: &str) -> Self {
        self.params.insert("outputdepthattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputdepthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputdepthattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usepieceattrib(mut self, val: bool) -> Self {
        self.params.insert("usepieceattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usepieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("usepieceattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useiter(mut self, val: bool) -> Self {
        self.params.insert("useiter".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useiter_expr(mut self, expr: &str) -> Self {
        self.params.insert("useiter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_allowpartialunsub(mut self, val: bool) -> Self {
        self.params.insert("allowpartialunsub".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_allowpartialunsub_expr(mut self, expr: &str) -> Self {
        self.params.insert("allowpartialunsub".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_repairmesh(mut self, val: bool) -> Self {
        self.params.insert("repairmesh".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_repairmesh_expr(mut self, expr: &str) -> Self {
        self.params.insert("repairmesh".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preferuniform(mut self, val: bool) -> Self {
        self.params.insert("preferuniform".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_preferuniform_expr(mut self, expr: &str) -> Self {
        self.params.insert("preferuniform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useconnectivityattribute(mut self, val: bool) -> Self {
        self.params.insert("useconnectivityattribute".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useconnectivityattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("useconnectivityattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputdepthattribute(mut self, val: bool) -> Self {
        self.params.insert("useoutputdepthattribute".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputdepthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputdepthattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUnsubdivide {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "unsubdivide"
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
pub struct SopUsdconfigure {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUsdconfigure {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry"
    pub fn set_input_geometry<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry" and specifies the output index of the target node.
    pub fn set_input_geometry_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_sampleframe(mut self, val: f32) -> Self {
        self.params.insert("sampleframe".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sampleframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("sampleframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert("setmissingwidths".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert("setmissingwidths".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_grouptype(mut self, val: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathprefix(mut self, val: &str) -> Self {
        self.params.insert("pathprefix".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathprefix".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_savepath(mut self, val: &str) -> Self {
        self.params.insert("savepath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("savepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usdprims(mut self, val: &str) -> Self {
        self.params.insert("usdprims".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_usdprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("usdprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_otherprims(mut self, val: &str) -> Self {
        self.params.insert("otherprims".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_otherprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("otherprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_packedprims(mut self, val: &str) -> Self {
        self.params.insert("packedprims".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_packedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("packedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nurbscurves(mut self, val: &str) -> Self {
        self.params.insert("nurbscurves".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nurbscurves_expr(mut self, expr: &str) -> Self {
        self.params.insert("nurbscurves".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nurbssurfs(mut self, val: &str) -> Self {
        self.params.insert("nurbssurfs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nurbssurfs_expr(mut self, expr: &str) -> Self {
        self.params.insert("nurbssurfs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert("kindschema".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert("kindschema".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert("pathattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert("subdgroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("subdgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topology(mut self, val: &str) -> Self {
        self.params.insert("topology".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topology_expr(mut self, expr: &str) -> Self {
        self.params.insert("topology".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert("attribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("attribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert("indexattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("indexattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert("constantattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("constantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert("scalarconstantattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalarconstantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert("boolattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("boolattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uintattribs(mut self, val: &str) -> Self {
        self.params.insert("uintattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("uintattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uint64attribs(mut self, val: &str) -> Self {
        self.params.insert("uint64attribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("uint64attribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_assetpathattribs(mut self, val: &str) -> Self {
        self.params.insert("assetpathattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("assetpathattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert("staticattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("staticattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert("partitionattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("partitionattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert("subsetgroups".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert("subsetgroups".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert("customattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("customattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_combinemergeableattribs(mut self, val: bool) -> Self {
        self.params.insert("combinemergeableattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_combinemergeableattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("combinemergeableattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_group(mut self, val: bool) -> Self {
        self.params.insert("enable_group".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_grouptype(mut self, val: bool) -> Self {
        self.params.insert("enable_grouptype".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_pathprefix(mut self, val: bool) -> Self {
        self.params.insert("enable_pathprefix".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_pathprefix".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_savepath(mut self, val: bool) -> Self {
        self.params.insert("enable_savepath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_savepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_sampleframe(mut self, val: bool) -> Self {
        self.params.insert("enable_sampleframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_sampleframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_sampleframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_usdprims(mut self, val: bool) -> Self {
        self.params.insert("enable_usdprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_usdprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_usdprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_otherprims(mut self, val: bool) -> Self {
        self.params.insert("enable_otherprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_otherprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_otherprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert("enable_defineonlyleafprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_defineonlyleafprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert("defineonlyleafprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("defineonlyleafprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_packedprims(mut self, val: bool) -> Self {
        self.params.insert("enable_packedprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_packedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_packedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_nurbscurves(mut self, val: bool) -> Self {
        self.params.insert("enable_nurbscurves".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_nurbscurves_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_nurbscurves".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_nurbssurfs(mut self, val: bool) -> Self {
        self.params.insert("enable_nurbssurfs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_nurbssurfs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_nurbssurfs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert("enable_kindschema".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_kindschema".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_pathattr(mut self, val: bool) -> Self {
        self.params.insert("enable_pathattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_pathattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert("enable_heightfieldconvert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_heightfieldconvert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert("heightfieldconvert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightfieldconvert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert("enable_polygonsassubd".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_polygonsassubd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert("polygonsassubd".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert("polygonsassubd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert("enable_subdgroup".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_subdgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert("enable_reversepolygons".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_reversepolygons".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert("reversepolygons".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert("reversepolygons".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert("enable_setmissingwidths".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_setmissingwidths".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert("enable_setdefaultprim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_setdefaultprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert("setdefaultprim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("setdefaultprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_topology(mut self, val: bool) -> Self {
        self.params.insert("enable_topology".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_topology_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_topology".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_attribs(mut self, val: bool) -> Self {
        self.params.insert("enable_attribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_attribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_indexattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_indexattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_constantattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_constantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_scalarconstantattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_scalarconstantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_boolattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_boolattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_uintattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_uintattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_uintattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_uint64attribs(mut self, val: bool) -> Self {
        self.params.insert("enable_uint64attribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_uint64attribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_assetpathattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_assetpathattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_assetpathattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_staticattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_staticattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_partitionattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_partitionattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert("enable_prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert("prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert("prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert("enable_subsetgroups".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_subsetgroups".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_customattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_customattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert("enable_translateuvtost".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_translateuvtost".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert("translateuvtost".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert("translateuvtost".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUsdconfigure {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdconfigure"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfiguregeometryGeotype {
    /// ![SCENEGRAPH_primtype_mesh]Mesh
    Mesh = 0,
    /// ![SCENEGRAPH_primtype_mesh_subdiv]Subdiv Mesh
    SubdivMesh = 1,
    /// ![SCENEGRAPH_primtype_pointinstancer]Packed Geometry
    PackedGeometry = 2,
    /// ![SCENEGRAPH_primtype_volume]Volume
    Volume = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfiguregeometryType {
    Float = 0,
    Integer = 1,
    Vector = 2,
    String = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfiguregeometryTypeinfo {
    GuessFromName = 0,
    None = 1,
    Position = 2,
    Vector = 3,
    Normal = 4,
    Color = 5,
    Quaternion = 6,
    TransformMatrix = 7,
    TextureCoordinate = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfiguregeometryPrecision {
    /// 8-bit
    N8MinusBit = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
    Auto = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfiguregeometryClass2 {
    Detail = 0,
    Primitive = 1,
    Point = 2,
    Vertex = 3,
}

#[derive(Debug, Clone)]
pub struct SopUsdconfiguregeometry {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUsdconfiguregeometry {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float4 parameters ---
    pub fn with_valuev_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(format!("value{}v", index1), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_valuev_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("value{}v", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_geotype(mut self, val: SopUsdconfiguregeometryGeotype) -> Self {
        self.params.insert("geotype".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_geotype_expr(mut self, expr: &str) -> Self {
        self.params.insert("geotype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_size_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("size{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_size_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("size{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: SopUsdconfiguregeometryType) -> Self {
        self.params.insert(format!("type{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("type{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_typeinfo_inst(mut self, index1: usize, val: SopUsdconfiguregeometryTypeinfo) -> Self {
        self.params.insert(format!("typeinfo{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_typeinfo_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("typeinfo{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: SopUsdconfiguregeometryPrecision) -> Self {
        self.params.insert(format!("precision{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("precision{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_class_2_inst(mut self, index1: usize, val: SopUsdconfiguregeometryClass2) -> Self {
        self.params.insert(format!("class{}_2", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_class_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("class{}_2", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert("name".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert("name".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_volfieldname(mut self, val: &str) -> Self {
        self.params.insert("volfieldname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_volfieldname_expr(mut self, expr: &str) -> Self {
        self.params.insert("volfieldname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_apischemas(mut self, val: &str) -> Self {
        self.params.insert("apischemas".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_apischemas_expr(mut self, expr: &str) -> Self {
        self.params.insert("apischemas".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_path(mut self, val: &str) -> Self {
        self.params.insert("path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_subdivscheme(mut self, val: &str) -> Self {
        self.params.insert("subdivscheme".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_subdivscheme_expr(mut self, expr: &str) -> Self {
        self.params.insert("subdivscheme".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_subdivholegroup(mut self, val: &str) -> Self {
        self.params.insert("subdivholegroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_subdivholegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("subdivholegroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pointinstancerpath(mut self, val: &str) -> Self {
        self.params.insert("pointinstancerpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pointinstancerpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pointinstancerpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_volprimpath(mut self, val: &str) -> Self {
        self.params.insert("volprimpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_volprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("volprimpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_volsavepath(mut self, val: &str) -> Self {
        self.params.insert("volsavepath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_volsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("volsavepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibility(mut self, val: &str) -> Self {
        self.params.insert("visibility".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibility_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibility".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert("purpose".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert("purpose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("name{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("name{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_string_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("string{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_string_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("string{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_name(mut self, val: bool) -> Self {
        self.params.insert("enable_name".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_name_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_name".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_volfieldname(mut self, val: bool) -> Self {
        self.params.insert("enable_volfieldname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_volfieldname_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_volfieldname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_apischemas(mut self, val: bool) -> Self {
        self.params.insert("enable_apischemas".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_apischemas_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_apischemas".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_path(mut self, val: bool) -> Self {
        self.params.insert("enable_path".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_subdivscheme(mut self, val: bool) -> Self {
        self.params.insert("enable_subdivscheme".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_subdivscheme_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_subdivscheme".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_subdivholegroup(mut self, val: bool) -> Self {
        self.params.insert("enable_subdivholegroup".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_subdivholegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_subdivholegroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_pointinstancerpath(mut self, val: bool) -> Self {
        self.params.insert("enable_pointinstancerpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_pointinstancerpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_pointinstancerpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_volprimpath(mut self, val: bool) -> Self {
        self.params.insert("enable_volprimpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_volprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_volprimpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_volsavepath(mut self, val: bool) -> Self {
        self.params.insert("enable_volsavepath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_volsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_volsavepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_active(mut self, val: bool) -> Self {
        self.params.insert("enable_active".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_active_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_active".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_active(mut self, val: bool) -> Self {
        self.params.insert("active".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.params.insert("active".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_visibility(mut self, val: bool) -> Self {
        self.params.insert("enable_visibility".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_visibility_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_visibility".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_purpose(mut self, val: bool) -> Self {
        self.params.insert("enable_purpose".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_purpose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("enable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("enable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUsdconfiguregeometry {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdconfiguregeometry"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfigureprimsfrompointsPrimvartype {
    Float = 0,
    Integer = 1,
    Vector = 2,
    String = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfigureprimsfrompointsPrimvartypeinfo {
    GuessFromName = 0,
    None = 1,
    Position = 2,
    Vector = 3,
    Normal = 4,
    Color = 5,
    Quaternion = 6,
    TransformMatrix = 7,
    TextureCoordinate = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfigureprimsfrompointsPrimvarprecision {
    /// 8-bit
    N8MinusBit = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
    Auto = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfigureprimsfrompointsType {
    Float = 0,
    Integer = 1,
    Vector = 2,
    String = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfigureprimsfrompointsTypeinfo {
    GuessFromName = 0,
    None = 1,
    Position = 2,
    Vector = 3,
    Normal = 4,
    Color = 5,
    Quaternion = 6,
    TransformMatrix = 7,
    TextureCoordinate = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdconfigureprimsfrompointsPrecision {
    /// 8-bit
    N8MinusBit = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
    Auto = 4,
}

#[derive(Debug, Clone)]
pub struct SopUsdconfigureprimsfrompoints {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUsdconfigureprimsfrompoints {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_radiustop(mut self, val: f32) -> Self {
        self.params.insert("radiusTop".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radiustop_expr(mut self, expr: &str) -> Self {
        self.params.insert("radiusTop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_radiusbottom(mut self, val: f32) -> Self {
        self.params.insert("radiusBottom".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radiusbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert("radiusBottom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert("height".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert("height".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert("width".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert("width".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_length(mut self, val: f32) -> Self {
        self.params.insert("length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.params.insert("intensity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert("intensity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_exposure(mut self, val: f32) -> Self {
        self.params.insert("exposure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("exposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert("color".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert("color".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float4 parameters ---
    pub fn with_primvarvalue_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(format!("primvarvalue{}", index1), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_primvarvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvarvalue{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_valuev_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(format!("value{}v", index1), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_valuev_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("value{}v", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_primvarsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("primvarsize{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_primvarsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvarsize{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_size_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("size{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_size_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("size{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_primvartype_inst(mut self, index1: usize, val: SopUsdconfigureprimsfrompointsPrimvartype) -> Self {
        self.params.insert(format!("primvartype{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_primvartype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvartype{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primvartypeinfo_inst(mut self, index1: usize, val: SopUsdconfigureprimsfrompointsPrimvartypeinfo) -> Self {
        self.params.insert(format!("primvartypeinfo{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_primvartypeinfo_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvartypeinfo{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primvarprecision_inst(mut self, index1: usize, val: SopUsdconfigureprimsfrompointsPrimvarprecision) -> Self {
        self.params.insert(format!("primvarprecision{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_primvarprecision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvarprecision{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: SopUsdconfigureprimsfrompointsType) -> Self {
        self.params.insert(format!("type{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("type{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_typeinfo_inst(mut self, index1: usize, val: SopUsdconfigureprimsfrompointsTypeinfo) -> Self {
        self.params.insert(format!("typeinfo{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_typeinfo_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("typeinfo{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: SopUsdconfigureprimsfrompointsPrecision) -> Self {
        self.params.insert(format!("precision{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("precision{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert("primtype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("primtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_path(mut self, val: &str) -> Self {
        self.params.insert("path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert("name".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert("name".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibility(mut self, val: &str) -> Self {
        self.params.insert("visibility".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibility_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibility".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert("purpose".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert("purpose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_kind(mut self, val: &str) -> Self {
        self.params.insert("kind".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_kind_expr(mut self, expr: &str) -> Self {
        self.params.insert("kind".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_apischemas(mut self, val: &str) -> Self {
        self.params.insert("apischemas".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_apischemas_expr(mut self, expr: &str) -> Self {
        self.params.insert("apischemas".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_materialpath(mut self, val: &str) -> Self {
        self.params.insert("materialpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("materialpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_materialreffile(mut self, val: &str) -> Self {
        self.params.insert("materialreffile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_materialreffile_expr(mut self, expr: &str) -> Self {
        self.params.insert("materialreffile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_materialrefprim(mut self, val: &str) -> Self {
        self.params.insert("materialrefprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_materialrefprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("materialrefprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primvarname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("primvarname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_primvarname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvarname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primvarstring_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("primvarstring{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_primvarstring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvarstring{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("name{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("name{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_string_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("string{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_string_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("string{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_primtype(mut self, val: bool) -> Self {
        self.params.insert("enable_primtype".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_primtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_path(mut self, val: bool) -> Self {
        self.params.insert("enable_path".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_name(mut self, val: bool) -> Self {
        self.params.insert("enable_name".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_name_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_name".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_color(mut self, val: bool) -> Self {
        self.params.insert("enable_color".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_color_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_color".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_radius(mut self, val: bool) -> Self {
        self.params.insert("enable_radius".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_size(mut self, val: bool) -> Self {
        self.params.insert("enable_size".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_radiustop(mut self, val: bool) -> Self {
        self.params.insert("enable_radiusTop".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_radiustop_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_radiusTop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_radiusbottom(mut self, val: bool) -> Self {
        self.params.insert("enable_radiusBottom".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_radiusbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_radiusBottom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_height(mut self, val: bool) -> Self {
        self.params.insert("enable_height".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_height_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_height".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_width(mut self, val: bool) -> Self {
        self.params.insert("enable_width".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_width_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_width".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_length(mut self, val: bool) -> Self {
        self.params.insert("enable_length".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_intensity(mut self, val: bool) -> Self {
        self.params.insert("enable_intensity".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_intensity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_exposure(mut self, val: bool) -> Self {
        self.params.insert("enable_exposure".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_exposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_color(mut self, val: bool) -> Self {
        self.params.insert("hide_color".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_color_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_color".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_radius(mut self, val: bool) -> Self {
        self.params.insert("hide_radius".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_size(mut self, val: bool) -> Self {
        self.params.insert("hide_size".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_radiustop(mut self, val: bool) -> Self {
        self.params.insert("hide_radiusTop".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_radiustop_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_radiusTop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_radiusbottom(mut self, val: bool) -> Self {
        self.params.insert("hide_radiusBottom".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_radiusbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_radiusBottom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_height(mut self, val: bool) -> Self {
        self.params.insert("hide_height".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_height_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_height".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_width(mut self, val: bool) -> Self {
        self.params.insert("hide_width".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_width_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_width".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_length(mut self, val: bool) -> Self {
        self.params.insert("hide_length".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_intensity(mut self, val: bool) -> Self {
        self.params.insert("hide_intensity".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_intensity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hide_exposure(mut self, val: bool) -> Self {
        self.params.insert("hide_exposure".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hide_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("hide_exposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_active(mut self, val: bool) -> Self {
        self.params.insert("enable_active".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_active_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_active".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_active(mut self, val: bool) -> Self {
        self.params.insert("active".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.params.insert("active".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_visibility(mut self, val: bool) -> Self {
        self.params.insert("enable_visibility".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_visibility_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_visibility".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_purpose(mut self, val: bool) -> Self {
        self.params.insert("enable_purpose".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_purpose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_kind(mut self, val: bool) -> Self {
        self.params.insert("enable_kind".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_kind_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_kind".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_apischemas(mut self, val: bool) -> Self {
        self.params.insert("enable_apischemas".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_apischemas_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_apischemas".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_materialpath(mut self, val: bool) -> Self {
        self.params.insert("enable_materialpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_materialpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_materialreffile(mut self, val: bool) -> Self {
        self.params.insert("enable_materialreffile".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_materialreffile_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_materialreffile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_materialrefprim(mut self, val: bool) -> Self {
        self.params.insert("enable_materialrefprim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_materialrefprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_materialrefprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primvarenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("primvarenable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_primvarenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvarenable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("enable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("enable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUsdconfigureprimsfrompoints {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdconfigureprimsfrompoints"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdexportXformtype {
    None = 0,
    IntoWorldSpace = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdexportTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone)]
pub struct SopUsdexport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUsdexport {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert("executebackground".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert("setmissingwidths".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert("setmissingwidths".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(format!("optionfloatvalue{}", index1), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("optionfloatvalue{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_xformtype(mut self, val: SopUsdexportXformtype) -> Self {
        self.params.insert("xformtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xformtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("xformtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_trange(mut self, val: SopUsdexportTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_grouptype(mut self, val: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathprefix(mut self, val: &str) -> Self {
        self.params.insert("pathprefix".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathprefix".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_packedusdhandling(mut self, val: &str) -> Self {
        self.params.insert("packedusdhandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("packedusdhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_otherprimhandling(mut self, val: &str) -> Self {
        self.params.insert("otherprimhandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("otherprimhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert("packedhandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("packedhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_agenthandling(mut self, val: &str) -> Self {
        self.params.insert("agenthandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_agenthandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("agenthandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nurbscurvehandling(mut self, val: &str) -> Self {
        self.params.insert("nurbscurvehandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("nurbscurvehandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nurbssurfhandling(mut self, val: &str) -> Self {
        self.params.insert("nurbssurfhandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("nurbssurfhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert("kindschema".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert("kindschema".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert("pathattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert("subdgroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("subdgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert("authortimesamples".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert("authortimesamples".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert("topologyhandling".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("topologyhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert("attribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("attribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert("indexattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("indexattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert("constantattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("constantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert("scalarconstantattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalarconstantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert("boolattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("boolattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert("staticattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("staticattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert("partitionattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("partitionattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert("subsetgroups".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert("subsetgroups".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert("customattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("customattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert("loppath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert("loppath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lopoutput(mut self, val: &str) -> Self {
        self.params.insert("lopoutput".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert("lopoutput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defaultprim(mut self, val: &str) -> Self {
        self.params.insert("defaultprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_defaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("defaultprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("optionname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("optionname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("optiontype{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("optiontype{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("optionstrvalue{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("optionstrvalue{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_group(mut self, val: bool) -> Self {
        self.params.insert("enable_group".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_grouptype(mut self, val: bool) -> Self {
        self.params.insert("enable_grouptype".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_pathprefix(mut self, val: bool) -> Self {
        self.params.insert("enable_pathprefix".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_pathprefix".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_packedusdhandling(mut self, val: bool) -> Self {
        self.params.insert("enable_packedusdhandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_packedusdhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_otherprimhandling(mut self, val: bool) -> Self {
        self.params.insert("enable_otherprimhandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_otherprimhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert("enable_defineonlyleafprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_defineonlyleafprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert("defineonlyleafprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("defineonlyleafprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_packedhandling(mut self, val: bool) -> Self {
        self.params.insert("enable_packedhandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_packedhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_agenthandling(mut self, val: bool) -> Self {
        self.params.insert("enable_agenthandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_agenthandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_agenthandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_nurbscurvehandling(mut self, val: bool) -> Self {
        self.params.insert("enable_nurbscurvehandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_nurbscurvehandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_nurbssurfhandling(mut self, val: bool) -> Self {
        self.params.insert("enable_nurbssurfhandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_nurbssurfhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert("enable_kindschema".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_kindschema".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_pathattr(mut self, val: bool) -> Self {
        self.params.insert("enable_pathattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_pathattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert("enable_heightfieldconvert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_heightfieldconvert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert("heightfieldconvert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightfieldconvert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert("enable_polygonsassubd".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_polygonsassubd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert("polygonsassubd".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert("polygonsassubd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert("enable_subdgroup".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_subdgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert("enable_reversepolygons".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_reversepolygons".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert("reversepolygons".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert("reversepolygons".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert("enable_setmissingwidths".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_setmissingwidths".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert("enable_setdefaultprim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_setdefaultprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert("setdefaultprim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("setdefaultprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_topologyhandling(mut self, val: bool) -> Self {
        self.params.insert("enable_topologyhandling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_topologyhandling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_attribs(mut self, val: bool) -> Self {
        self.params.insert("enable_attribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_attribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_indexattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_indexattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_constantattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_constantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_scalarconstantattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_scalarconstantattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_boolattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_boolattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_staticattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_staticattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_partitionattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_partitionattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert("enable_prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert("prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert("prefixpartitionsubsets".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert("enable_subsetgroups".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_subsetgroups".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert("enable_customattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_customattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert("enable_translateuvtost".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert("enable_translateuvtost".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert("translateuvtost".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert("translateuvtost".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fileperframe(mut self, val: bool) -> Self {
        self.params.insert("fileperframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fileperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("fileperframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_trackprimexistence(mut self, val: bool) -> Self {
        self.params.insert("trackprimexistence".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_trackprimexistence_expr(mut self, expr: &str) -> Self {
        self.params.insert("trackprimexistence".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usenetworksafesave(mut self, val: bool) -> Self {
        self.params.insert("usenetworksafesave".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usenetworksafesave_expr(mut self, expr: &str) -> Self {
        self.params.insert("usenetworksafesave".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths(mut self, val: bool) -> Self {
        self.params.insert("enableoutputprocessor_simplerelativepaths".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert("enableoutputprocessor_simplerelativepaths".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_requiredefaultprim(mut self, val: bool) -> Self {
        self.params.insert("requiredefaultprim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_requiredefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("requiredefaultprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_savetimeinfo(mut self, val: bool) -> Self {
        self.params.insert("savetimeinfo".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_savetimeinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert("savetimeinfo".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clearhoudinicustomdata(mut self, val: bool) -> Self {
        self.params.insert("clearhoudinicustomdata".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_clearhoudinicustomdata_expr(mut self, expr: &str) -> Self {
        self.params.insert("clearhoudinicustomdata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ensuremetricsset(mut self, val: bool) -> Self {
        self.params.insert("ensuremetricsset".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_ensuremetricsset_expr(mut self, expr: &str) -> Self {
        self.params.insert("ensuremetricsset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_setropcook(mut self, val: bool) -> Self {
        self.params.insert("setropcook".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_setropcook_expr(mut self, expr: &str) -> Self {
        self.params.insert("setropcook".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert("initsim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert("initsim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert("alfprogress".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert("alfprogress".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert("reportnetwork".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert("reportnetwork".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUsdexport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdexport"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdimportMissingfile {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdimportViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdimportPivot {
    Origin = 0,
    Centroid = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUsdimportUnpackGeomtype {
    PackedPrims = 0,
    Polygons = 1,
}

#[derive(Debug, Clone)]
pub struct SopUsdimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUsdimport {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn trigger_reload(mut self) -> Self {
        self.params.insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_timeoffset1(mut self, val: f32) -> Self {
        self.params.insert("timeoffset1".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_timeoffset1_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeoffset1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_timescale1(mut self, val: f32) -> Self {
        self.params.insert("timescale1".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_timescale1_expr(mut self, expr: &str) -> Self {
        self.params.insert("timescale1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_importtime(mut self, val: f32) -> Self {
        self.params.insert("importtime".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_importtime_expr(mut self, expr: &str) -> Self {
        self.params.insert("importtime".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_variantsetindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("variantsetindex{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_variantsetindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantsetindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantnameindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("variantnameindex{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_variantnameindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantnameindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_missingfile(mut self, val: SopUsdimportMissingfile) -> Self {
        self.params.insert("missingfile".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_missingfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("missingfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_viewportlod(mut self, val: SopUsdimportViewportlod) -> Self {
        self.params.insert("viewportlod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.params.insert("viewportlod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pivot(mut self, val: SopUsdimportPivot) -> Self {
        self.params.insert("pivot".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert("pivot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unpack_geomtype(mut self, val: SopUsdimportUnpackGeomtype) -> Self {
        self.params.insert("unpack_geomtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_unpack_geomtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("unpack_geomtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_filepath1(mut self, val: &str) -> Self {
        self.params.insert("filepath1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filepath1_expr(mut self, expr: &str) -> Self {
        self.params.insert("filepath1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantprimpattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("variantprimpattern{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_variantprimpattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantprimpattern{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("variantset{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_variantset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantset{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("variantname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_variantname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert("primpattern".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert("primpattern".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathattrib(mut self, val: &str) -> Self {
        self.params.insert("pathattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nameattrib(mut self, val: &str) -> Self {
        self.params.insert("nameattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("nameattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_importtraversal(mut self, val: &str) -> Self {
        self.params.insert("importtraversal".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_importtraversal_expr(mut self, expr: &str) -> Self {
        self.params.insert("importtraversal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert("purpose".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert("purpose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_setvariants(mut self, val: bool) -> Self {
        self.params.insert("setvariants".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_setvariants_expr(mut self, expr: &str) -> Self {
        self.params.insert("setvariants".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantsetuseindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("variantsetuseindex{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_variantsetuseindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantsetuseindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantnameuseindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("variantnameuseindex{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_variantnameuseindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantnameuseindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_input_unpack(mut self, val: bool) -> Self {
        self.params.insert("input_unpack".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_input_unpack_expr(mut self, expr: &str) -> Self {
        self.params.insert("input_unpack".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUsdimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdimport"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvautoseamBasesplit {
    /// Curvature-Based
    CurvatureMinusBased = 0,
    ExistingIslands = 1,
}

#[derive(Debug, Clone)]
pub struct SopUvautoseam {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvautoseam {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry to Seam"
    pub fn set_input_geometry_to_seam<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Seam" and specifies the output index of the target node.
    pub fn set_input_geometry_to_seam_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_avoidanceweight(mut self, val: f32) -> Self {
        self.params.insert("avoidanceweight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_avoidanceweight_expr(mut self, expr: &str) -> Self {
        self.params.insert("avoidanceweight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_graintol(mut self, val: f32) -> Self {
        self.params.insert("graintol".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_graintol_expr(mut self, expr: &str) -> Self {
        self.params.insert("graintol".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mergethreshold(mut self, val: f32) -> Self {
        self.params.insert("mergethreshold".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mergethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert("mergethreshold".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvtolerance(mut self, val: f32) -> Self {
        self.params.insert("uvtolerance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_uvtolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvtolerance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_basesplit(mut self, val: SopUvautoseamBasesplit) -> Self {
        self.params.insert("basesplit".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_basesplit_expr(mut self, expr: &str) -> Self {
        self.params.insert("basesplit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preseams(mut self, val: &str) -> Self {
        self.params.insert("preseams".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_preseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("preseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_islandattr(mut self, val: &str) -> Self {
        self.params.insert("islandattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_islandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("islandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nonseams(mut self, val: &str) -> Self {
        self.params.insert("nonseams".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nonseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("nonseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_avoidattr(mut self, val: &str) -> Self {
        self.params.insert("avoidattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_avoidattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("avoidattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seamsgroupname(mut self, val: &str) -> Self {
        self.params.insert("seamsgroupname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_seamsgroupname_expr(mut self, expr: &str) -> Self {
        self.params.insert("seamsgroupname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputislandattr(mut self, val: &str) -> Self {
        self.params.insert("outputislandattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_useoutputislandattr(mut self, val: bool) -> Self {
        self.params.insert("useoutputislandattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvautoseam {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvautoseam"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvbrushForeback {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvbrushOp {
    Drag = 0,
    /// Dilate/Contract
    DilateContract = 1,
    Smooth = 2,
    EraseChanges = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvbrushShape {
    Circle = 0,
    Square = 1,
    Bitmap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvbrushBmpchan {
    Alpha = 0,
    Luminance = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvbrushUptype {
    StrokeDirection = 0,
    Fixed = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvbrushEvent {
    BeginStroke = 0,
    ActiveStroke = 1,
    EndStroke = 2,
    Click = 3,
    /// No-op
    NoMinusOp = 4,
}

#[derive(Debug, Clone)]
pub struct SopUvbrush {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvbrush {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry to UVBrush"
    pub fn set_input_geometry_to_uvbrush<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to UVBrush" and specifies the output index of the target node.
    pub fn set_input_geometry_to_uvbrush_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_flood(mut self) -> Self {
        self.params.insert("flood".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_clearall(mut self) -> Self {
        self.params.insert("clearall".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_fs(mut self, val: f32) -> Self {
        self.params.insert("fs".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_fs_expr(mut self, expr: &str) -> Self {
        self.params.insert("fs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bs(mut self, val: f32) -> Self {
        self.params.insert("bs".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bs_expr(mut self, expr: &str) -> Self {
        self.params.insert("bs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvradius(mut self, val: f32) -> Self {
        self.params.insert("uvradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_uvradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_radiuspressure(mut self, val: f32) -> Self {
        self.params.insert("radiuspressure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radiuspressure_expr(mut self, expr: &str) -> Self {
        self.params.insert("radiuspressure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_brushangle(mut self, val: f32) -> Self {
        self.params.insert("brushangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_brushangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("brushangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_squash(mut self, val: f32) -> Self {
        self.params.insert("squash".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_squash_expr(mut self, expr: &str) -> Self {
        self.params.insert("squash".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_opacity(mut self, val: f32) -> Self {
        self.params.insert("opacity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert("opacity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_opacitypresure(mut self, val: f32) -> Self {
        self.params.insert("opacitypresure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_opacitypresure_expr(mut self, expr: &str) -> Self {
        self.params.insert("opacitypresure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_brushsplatter(mut self, val: f32) -> Self {
        self.params.insert("brushsplatter".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_brushsplatter_expr(mut self, expr: &str) -> Self {
        self.params.insert("brushsplatter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_papergrain(mut self, val: f32) -> Self {
        self.params.insert("papergrain".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_papergrain_expr(mut self, expr: &str) -> Self {
        self.params.insert("papergrain".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_softedge(mut self, val: f32) -> Self {
        self.params.insert("softedge".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_softedge_expr(mut self, expr: &str) -> Self {
        self.params.insert("softedge".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_symdist(mut self, val: f32) -> Self {
        self.params.insert("symdist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_symdist_expr(mut self, expr: &str) -> Self {
        self.params.insert("symdist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_symuvangle(mut self, val: f32) -> Self {
        self.params.insert("symuvangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_symuvangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("symuvangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hitpressure(mut self, val: f32) -> Self {
        self.params.insert("hitpressure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hitpressure_expr(mut self, expr: &str) -> Self {
        self.params.insert("hitpressure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_depth(mut self, val: [f32; 2]) -> Self {
        self.params.insert("depth".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert("depth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_symuvorig(mut self, val: [f32; 2]) -> Self {
        self.params.insert("symuvorig".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_symuvorig_expr(mut self, expr: &str) -> Self {
        self.params.insert("symuvorig".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert("upvector".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.params.insert("upvector".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_symaxis(mut self, val: [f32; 3]) -> Self {
        self.params.insert("symaxis".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_symaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert("symaxis".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_symorig(mut self, val: [f32; 3]) -> Self {
        self.params.insert("symorig".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_symorig_expr(mut self, expr: &str) -> Self {
        self.params.insert("symorig".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dir".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert("dir".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hitpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert("hitpos".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_hitpos_expr(mut self, expr: &str) -> Self {
        self.params.insert("hitpos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hituvw(mut self, val: [f32; 3]) -> Self {
        self.params.insert("hituvw".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_hituvw_expr(mut self, expr: &str) -> Self {
        self.params.insert("hituvw".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_symrot(mut self, val: i32) -> Self {
        self.params.insert("symrot".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_symrot_expr(mut self, expr: &str) -> Self {
        self.params.insert("symrot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hitprim(mut self, val: i32) -> Self {
        self.params.insert("hitprim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hitprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("hitprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hitpt(mut self, val: i32) -> Self {
        self.params.insert("hitpt".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hitpt_expr(mut self, expr: &str) -> Self {
        self.params.insert("hitpt".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_foreback(mut self, val: SopUvbrushForeback) -> Self {
        self.params.insert("foreback".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_foreback_expr(mut self, expr: &str) -> Self {
        self.params.insert("foreback".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_op(mut self, val: SopUvbrushOp) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shape(mut self, val: SopUvbrushShape) -> Self {
        self.params.insert("shape".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_shape_expr(mut self, expr: &str) -> Self {
        self.params.insert("shape".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bmpchan(mut self, val: SopUvbrushBmpchan) -> Self {
        self.params.insert("bmpchan".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bmpchan_expr(mut self, expr: &str) -> Self {
        self.params.insert("bmpchan".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uptype(mut self, val: SopUvbrushUptype) -> Self {
        self.params.insert("uptype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_event(mut self, val: SopUvbrushEvent) -> Self {
        self.params.insert("event".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_event_expr(mut self, expr: &str) -> Self {
        self.params.insert("event".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bitmap(mut self, val: &str) -> Self {
        self.params.insert("bitmap".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bitmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("bitmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_kernel(mut self, val: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedepth(mut self, val: bool) -> Self {
        self.params.insert("usedepth".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedepth_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedepth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doreflect(mut self, val: bool) -> Self {
        self.params.insert("doreflect".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doreflect_expr(mut self, expr: &str) -> Self {
        self.params.insert("doreflect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dorotate(mut self, val: bool) -> Self {
        self.params.insert("dorotate".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dorotate_expr(mut self, expr: &str) -> Self {
        self.params.insert("dorotate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_douvreflect(mut self, val: bool) -> Self {
        self.params.insert("douvreflect".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_douvreflect_expr(mut self, expr: &str) -> Self {
        self.params.insert("douvreflect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_projtype(mut self, val: bool) -> Self {
        self.params.insert("projtype".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_projtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("projtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useconnectivity(mut self, val: bool) -> Self {
        self.params.insert("useconnectivity".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useconnectivity_expr(mut self, expr: &str) -> Self {
        self.params.insert("useconnectivity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usenormals(mut self, val: bool) -> Self {
        self.params.insert("usenormals".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usenormals_expr(mut self, expr: &str) -> Self {
        self.params.insert("usenormals".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_realtime(mut self, val: bool) -> Self {
        self.params.insert("realtime".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_realtime_expr(mut self, expr: &str) -> Self {
        self.params.insert("realtime".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvbrush {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvbrush"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUveditGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUveditXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUveditRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUveditType {
    Linear = 0,
    Quadratic = 1,
    Cubic = 2,
    /// Meta-ball
    MetaMinusBall = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUveditMetric {
    Uv = 0,
    Uvw = 1,
    Xyz = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUveditVisualizefalloff {
    Never = 0,
    Always = 1,
    WhenViewportToolIsActive = 2,
}

#[derive(Debug, Clone)]
pub struct SopUvedit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvedit {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "UVs to Edit"
    pub fn set_input_uvs_to_edit<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "UVs to Edit" and specifies the output index of the target node.
    pub fn set_input_uvs_to_edit_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_apply(mut self) -> Self {
        self.params.insert("apply".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_clearall(mut self) -> Self {
        self.params.insert("clearall".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_tandeg(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tandeg".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tandeg_expr(mut self, expr: &str) -> Self {
        self.params.insert("tandeg".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert("shear".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert("shear".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopUveditGrouptype) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: SopUveditXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: SopUveditRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type(mut self, val: SopUveditType) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_metric(mut self, val: SopUveditMetric) -> Self {
        self.params.insert("metric".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert("metric".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visualizefalloff(mut self, val: SopUveditVisualizefalloff) -> Self {
        self.params.insert("visualizefalloff".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_visualizefalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("visualizefalloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_kernel(mut self, val: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_global(mut self, val: bool) -> Self {
        self.params.insert("global".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_global_expr(mut self, expr: &str) -> Self {
        self.params.insert("global".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvglobal(mut self, val: bool) -> Self {
        self.params.insert("uvglobal".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uvglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvglobal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvsew(mut self, val: bool) -> Self {
        self.params.insert("uvsew".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uvsew_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvsew".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvedit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvedit"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenMethod {
    /// Spectral (SCP)
    SpectralScp = 0,
    /// Angle-Based (ABF)
    AngleMinusBasedAbf = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenAlignAxis {
    UAxis = 0,
    VAxis = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenLalignAxis {
    UAxis = 0,
    VAxis = 1,
}

#[derive(Debug, Clone)]
pub struct SopUvflatten {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvflatten {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry to Flatten"
    pub fn set_input_geometry_to_flatten<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Flatten" and specifies the output index of the target node.
    pub fn set_input_geometry_to_flatten_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float2 parameters ---
    pub fn with_primvert_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("primvert{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_primvert_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("primvert{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pinuv_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("pinuv{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_pinuv_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("pinuv{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pinrefuv_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("pinrefuv{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_pinrefuv_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("pinrefuv{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lprimvert_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("lprimvert{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_lprimvert_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("lprimvert{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpinuv_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("lpinuv{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_lpinuv_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("lpinuv{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpinrefuv_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("lpinrefuv{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_lpinrefuv_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("lpinrefuv{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bboxcenter(mut self, val: [f32; 2]) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_bboxcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bboxsize(mut self, val: [f32; 2]) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_bboxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopUvflattenMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_align_axis_inst(mut self, index1: usize, val: SopUvflattenAlignAxis) -> Self {
        self.params.insert(format!("align_axis{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_align_axis_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("align_axis{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lalign_axis_inst(mut self, index1: usize, val: SopUvflattenLalignAxis) -> Self {
        self.params.insert(format!("lalign_axis{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_lalign_axis_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("lalign_axis{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seamgroup(mut self, val: &str) -> Self {
        self.params.insert("seamgroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_seamgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("seamgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rectifygroup(mut self, val: &str) -> Self {
        self.params.insert("rectifygroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_rectifygroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("rectifygroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_align_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("align_group{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_align_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("align_group{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_straight_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("straight_group{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_straight_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("straight_group{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layoutseamgroup(mut self, val: &str) -> Self {
        self.params.insert("layoutseamgroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layoutseamgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("layoutseamgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layoutrectifygroup(mut self, val: &str) -> Self {
        self.params.insert("layoutrectifygroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layoutrectifygroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("layoutrectifygroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lalign_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("lalign_group{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lalign_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("lalign_group{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lstraight_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("lstraight_group{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lstraight_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("lstraight_group{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputseams(mut self, val: &str) -> Self {
        self.params.insert("outputseams".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputconstrislands(mut self, val: &str) -> Self {
        self.params.insert("outputconstrislands".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputconstrislands_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputconstrislands".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepexistingseams(mut self, val: bool) -> Self {
        self.params.insert("keepexistingseams".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepexistingseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepexistingseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keepexistinglayout(mut self, val: bool) -> Self {
        self.params.insert("keepexistinglayout".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepexistinglayout_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepexistinglayout".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pinboundaries(mut self, val: bool) -> Self {
        self.params.insert("pinboundaries".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pinboundaries_expr(mut self, expr: &str) -> Self {
        self.params.insert("pinboundaries".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usepins(mut self, val: bool) -> Self {
        self.params.insert("usepins".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usepins_expr(mut self, expr: &str) -> Self {
        self.params.insert("usepins".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usepin_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("usepin{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usepin_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("usepin{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usealignments(mut self, val: bool) -> Self {
        self.params.insert("usealignments".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usealignments_expr(mut self, expr: &str) -> Self {
        self.params.insert("usealignments".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_align_enabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("align_enabled{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_align_enabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("align_enabled{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usestraightenings(mut self, val: bool) -> Self {
        self.params.insert("usestraightenings".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usestraightenings_expr(mut self, expr: &str) -> Self {
        self.params.insert("usestraightenings".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_straight_enabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("straight_enabled{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_straight_enabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("straight_enabled{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_manuallayout(mut self, val: bool) -> Self {
        self.params.insert("manuallayout".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_manuallayout_expr(mut self, expr: &str) -> Self {
        self.params.insert("manuallayout".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputseams(mut self, val: bool) -> Self {
        self.params.insert("useoutputseams".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputconstrislands(mut self, val: bool) -> Self {
        self.params.insert("useoutputconstrislands".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputconstrislands_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputconstrislands".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvflatten {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvflatten"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenfrompointsMethod {
    Fast = 0,
    Accurate = 1,
    OnlySeams = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenfrompointsCenterpointsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenfrompointsAutomaticmethod {
    FurthestPointSampling = 0,
    DistortionBased = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvflattenfrompointsPackingmethod {
    Rectangle = 0,
    Udims = 1,
    None = 2,
}

#[derive(Debug, Clone)]
pub struct SopUvflattenfrompoints {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvflattenfrompoints {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry to Parameterize"
    pub fn set_input_geometry_to_parameterize<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Parameterize" and specifies the output index of the target node.
    pub fn set_input_geometry_to_parameterize_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Source Point Cloud"
    pub fn set_input_source_point_cloud<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Source Point Cloud" and specifies the output index of the target node.
    pub fn set_input_source_point_cloud_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_automaticdistortionthreshold(mut self, val: f32) -> Self {
        self.params.insert("automaticdistortionthreshold".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_automaticdistortionthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert("automaticdistortionthreshold".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distortionthreshold(mut self, val: f32) -> Self {
        self.params.insert("distortionthreshold".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_distortionthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert("distortionthreshold".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_bboxcenter(mut self, val: [f32; 2]) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_bboxcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bboxsize(mut self, val: [f32; 2]) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_bboxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvcenter(mut self, val: [f32; 2]) -> Self {
        self.params.insert("uvcenter".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_uvcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvcenter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_nautomaticsources(mut self, val: i32) -> Self {
        self.params.insert("nautomaticsources".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_nautomaticsources_expr(mut self, expr: &str) -> Self {
        self.params.insert("nautomaticsources".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopUvflattenfrompointsMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_centerpointsource(mut self, val: SopUvflattenfrompointsCenterpointsource) -> Self {
        self.params.insert("centerpointsource".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_centerpointsource_expr(mut self, expr: &str) -> Self {
        self.params.insert("centerpointsource".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_automaticmethod(mut self, val: SopUvflattenfrompointsAutomaticmethod) -> Self {
        self.params.insert("automaticmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_automaticmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("automaticmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_packingmethod(mut self, val: SopUvflattenfrompointsPackingmethod) -> Self {
        self.params.insert("packingmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_packingmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("packingmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srcpoints(mut self, val: &str) -> Self {
        self.params.insert("srcpoints".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_srcpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("srcpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_up(mut self, val: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pscale(mut self, val: &str) -> Self {
        self.params.insert("pscale".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("pscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_udimtarget(mut self, val: &str) -> Self {
        self.params.insert("udimtarget".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_udimtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert("udimtarget".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distortedprims(mut self, val: &str) -> Self {
        self.params.insert("distortedprims".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_distortedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("distortedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unprocessedprims(mut self, val: &str) -> Self {
        self.params.insert("unprocessedprims".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unprocessedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("unprocessedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputseams(mut self, val: &str) -> Self {
        self.params.insert("outputseams".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputislandnumber(mut self, val: &str) -> Self {
        self.params.insert("outputislandnumber".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputislandnumber_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputislandnumber".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_copypointattribs(mut self, val: &str) -> Self {
        self.params.insert("copypointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_copypointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copypointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvsetprefix(mut self, val: &str) -> Self {
        self.params.insert("uvsetprefix".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvsetprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvsetprefix".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_addautomaticsources(mut self, val: bool) -> Self {
        self.params.insert("addautomaticsources".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addautomaticsources_expr(mut self, expr: &str) -> Self {
        self.params.insert("addautomaticsources".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useup(mut self, val: bool) -> Self {
        self.params.insert("useup".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useup_expr(mut self, expr: &str) -> Self {
        self.params.insert("useup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usescale(mut self, val: bool) -> Self {
        self.params.insert("usescale".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usescale_expr(mut self, expr: &str) -> Self {
        self.params.insert("usescale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usepscale(mut self, val: bool) -> Self {
        self.params.insert("usepscale".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("usepscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useudimtarget(mut self, val: bool) -> Self {
        self.params.insert("useudimtarget".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useudimtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert("useudimtarget".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usedistortionpruning(mut self, val: bool) -> Self {
        self.params.insert("usedistortionpruning".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedistortionpruning_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedistortionpruning".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputdistortedprims(mut self, val: bool) -> Self {
        self.params.insert("useoutputdistortedprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputdistortedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputdistortedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preserveexistinguvs(mut self, val: bool) -> Self {
        self.params.insert("preserveexistinguvs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_preserveexistinguvs_expr(mut self, expr: &str) -> Self {
        self.params.insert("preserveexistinguvs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputunprocessedprims(mut self, val: bool) -> Self {
        self.params.insert("useoutputunprocessedprims".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputunprocessedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputunprocessedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputseams(mut self, val: bool) -> Self {
        self.params.insert("useoutputseams".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useoutputislandnumber(mut self, val: bool) -> Self {
        self.params.insert("useoutputislandnumber".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useoutputislandnumber_expr(mut self, expr: &str) -> Self {
        self.params.insert("useoutputislandnumber".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usecopypointattribs(mut self, val: bool) -> Self {
        self.params.insert("usecopypointattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usecopypointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("usecopypointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createuvsetperpoint(mut self, val: bool) -> Self {
        self.params.insert("createuvsetperpoint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createuvsetperpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert("createuvsetperpoint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvflattenfrompoints {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvflattenfrompoints"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvfuseGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvfusePostype {
    Average = 0,
    FirstInGroup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvfuseMetric {
    Uv = 0,
    Uvw = 1,
    Xyz = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvfuseGridtype {
    GridSpacing = 0,
    GridLines = 1,
    PowerOf2GridLines = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvfuseGridround {
    Nearest = 0,
    Down = 1,
    Up = 2,
}

#[derive(Debug, Clone)]
pub struct SopUvfuse {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvfuse {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "UVs to Fuse"
    pub fn set_input_uvs_to_fuse<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "UVs to Fuse" and specifies the output index of the target node.
    pub fn set_input_uvs_to_fuse_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert("dist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert("dist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridtol(mut self, val: f32) -> Self {
        self.params.insert("gridtol".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridtol_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridtol".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_uvw(mut self, val: [f32; 3]) -> Self {
        self.params.insert("uvw".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_uvw_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvw".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridspacing(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gridspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridlines(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gridlines".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gridlines_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridlines".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gridoffset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gridoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int3 parameters ---
    pub fn with_gridpow2(mut self, val: [i32; 3]) -> Self {
        self.params.insert("gridpow2".to_string(), crate::core::types::ParamValue::Int3(val));
        self
    }
    pub fn with_gridpow2_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridpow2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopUvfuseGrouptype) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postype(mut self, val: SopUvfusePostype) -> Self {
        self.params.insert("postype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_postype_expr(mut self, expr: &str) -> Self {
        self.params.insert("postype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_metric(mut self, val: SopUvfuseMetric) -> Self {
        self.params.insert("metric".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert("metric".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridtype(mut self, val: SopUvfuseGridtype) -> Self {
        self.params.insert("gridtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_gridtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridround(mut self, val: SopUvfuseGridround) -> Self {
        self.params.insert("gridround".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_gridround_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridround".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedist(mut self, val: bool) -> Self {
        self.params.insert("usedist".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedist_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvfuse {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvfuse"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutProjplane {
    XyProjection = 0,
    YzProjection = 1,
    ZxProjection = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutAxisalignislands {
    None = 0,
    ByIslandSymmetry = 1,
    ByIslandPositionIn3d = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutScaling {
    Maximum = 0,
    Fixed = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutRotstep {
    NoRotations = 0,
    /// 180º
    N180 = 1,
    /// 90º
    N90 = 2,
    /// 45º
    N45 = 3,
    /// 22.5º
    N225 = 4,
    /// 11.25º
    N1125 = 5,
    /// 5.626º
    N5626 = 6,
    Custom = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutResolution {
    /// 256
    N256 = 0,
    /// 512
    N512 = 1,
    /// 1024
    N1024 = 2,
    /// 2048
    N2048 = 3,
    /// 4096
    N4096 = 4,
    Custom = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutTargettype {
    Rectangles = 0,
    UdimTiles = 1,
    IslandsFromSecondInput = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutUdimtilemethod {
    Range = 0,
    Grid = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvlayoutTargetprojplane {
    XyProjection = 0,
    YzProjection = 1,
    ZxProjection = 2,
}

#[derive(Debug, Clone)]
pub struct SopUvlayout {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvlayout {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry To Pack"
    pub fn set_input_geometry_to_pack<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry To Pack" and specifies the output index of the target node.
    pub fn set_input_geometry_to_pack_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Packing Target Geometry"
    pub fn set_input_packing_target_geometry<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Packing Target Geometry" and specifies the output index of the target node.
    pub fn set_input_packing_target_geometry_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvtolerance(mut self, val: f32) -> Self {
        self.params.insert("uvtolerance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_uvtolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvtolerance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scaletolerance(mut self, val: f32) -> Self {
        self.params.insert("scaletolerance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scaletolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert("scaletolerance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_overlaytolerance(mut self, val: f32) -> Self {
        self.params.insert("overlaytolerance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_overlaytolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert("overlaytolerance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_rect_center_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("rect_center{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_rect_center_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("rect_center{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rect_size_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(format!("rect_size{}", index1), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_rect_size_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("rect_size{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tilesize(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tilesize".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tilesize_expr(mut self, expr: &str) -> Self {
        self.params.insert("tilesize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_circledivs(mut self, val: i32) -> Self {
        self.params.insert("circledivs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_circledivs_expr(mut self, expr: &str) -> Self {
        self.params.insert("circledivs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_padding(mut self, val: i32) -> Self {
        self.params.insert("padding".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_padding_expr(mut self, expr: &str) -> Self {
        self.params.insert("padding".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_customresolution(mut self, val: i32) -> Self {
        self.params.insert("customresolution".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_customresolution_expr(mut self, expr: &str) -> Self {
        self.params.insert("customresolution".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_randseed(mut self, val: i32) -> Self {
        self.params.insert("randseed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_randseed_expr(mut self, expr: &str) -> Self {
        self.params.insert("randseed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defaulttarget(mut self, val: i32) -> Self {
        self.params.insert("defaulttarget".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_defaulttarget_expr(mut self, expr: &str) -> Self {
        self.params.insert("defaulttarget".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defaultudimtarget(mut self, val: i32) -> Self {
        self.params.insert("defaultudimtarget".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_defaultudimtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert("defaultudimtarget".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_numcolumns(mut self, val: i32) -> Self {
        self.params.insert("numcolumns".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numcolumns_expr(mut self, expr: &str) -> Self {
        self.params.insert("numcolumns".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_startingudim(mut self, val: i32) -> Self {
        self.params.insert("startingudim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_startingudim_expr(mut self, expr: &str) -> Self {
        self.params.insert("startingudim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_udimtilestart(mut self, val: i32) -> Self {
        self.params.insert("udimtilestart".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udimtilestart_expr(mut self, expr: &str) -> Self {
        self.params.insert("udimtilestart".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_udimtilecount(mut self, val: i32) -> Self {
        self.params.insert("udimtilecount".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udimtilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert("udimtilecount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_projplane(mut self, val: SopUvlayoutProjplane) -> Self {
        self.params.insert("projplane".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_projplane_expr(mut self, expr: &str) -> Self {
        self.params.insert("projplane".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_axisalignislands(mut self, val: SopUvlayoutAxisalignislands) -> Self {
        self.params.insert("axisalignislands".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_axisalignislands_expr(mut self, expr: &str) -> Self {
        self.params.insert("axisalignislands".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scaling(mut self, val: SopUvlayoutScaling) -> Self {
        self.params.insert("scaling".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_scaling_expr(mut self, expr: &str) -> Self {
        self.params.insert("scaling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rotstep(mut self, val: SopUvlayoutRotstep) -> Self {
        self.params.insert("rotstep".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rotstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("rotstep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resolution(mut self, val: SopUvlayoutResolution) -> Self {
        self.params.insert("resolution".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert("resolution".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targettype(mut self, val: SopUvlayoutTargettype) -> Self {
        self.params.insert("targettype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.params.insert("targettype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_udimtilemethod(mut self, val: SopUvlayoutUdimtilemethod) -> Self {
        self.params.insert("udimtilemethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_udimtilemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("udimtilemethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetprojplane(mut self, val: SopUvlayoutTargetprojplane) -> Self {
        self.params.insert("targetprojplane".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_targetprojplane_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetprojplane".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_separatoredges(mut self, val: &str) -> Self {
        self.params.insert("separatoredges".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_separatoredges_expr(mut self, expr: &str) -> Self {
        self.params.insert("separatoredges".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_islandattr(mut self, val: &str) -> Self {
        self.params.insert("islandattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_islandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("islandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetattr(mut self, val: &str) -> Self {
        self.params.insert("targetattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_targetattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetoverrides(mut self, val: &str) -> Self {
        self.params.insert("targetoverrides".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_targetoverrides_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetoverrides".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_islandscaleattr(mut self, val: &str) -> Self {
        self.params.insert("islandscaleattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_islandscaleattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("islandscaleattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scaleoverrides(mut self, val: &str) -> Self {
        self.params.insert("scaleoverrides".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scaleoverrides_expr(mut self, expr: &str) -> Self {
        self.params.insert("scaleoverrides".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_islandoffsetattr(mut self, val: &str) -> Self {
        self.params.insert("islandoffsetattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_islandoffsetattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("islandoffsetattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_offsetoverrides(mut self, val: &str) -> Self {
        self.params.insert("offsetoverrides".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_offsetoverrides_expr(mut self, expr: &str) -> Self {
        self.params.insert("offsetoverrides".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetuvattrib(mut self, val: &str) -> Self {
        self.params.insert("targetuvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_targetuvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetuvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetgroup(mut self, val: &str) -> Self {
        self.params.insert("targetgroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_targetgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetseparatoredges(mut self, val: &str) -> Self {
        self.params.insert("targetseparatoredges".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_targetseparatoredges_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetseparatoredges".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetislandattr(mut self, val: &str) -> Self {
        self.params.insert("targetislandattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_targetislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nonpackedpolys(mut self, val: &str) -> Self {
        self.params.insert("nonpackedpolys".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_nonpackedpolys_expr(mut self, expr: &str) -> Self {
        self.params.insert("nonpackedpolys".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputislandattr(mut self, val: &str) -> Self {
        self.params.insert("outputislandattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputtargetattr(mut self, val: &str) -> Self {
        self.params.insert("outputtargetattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputtargetattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputtargetattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_useislandattr(mut self, val: bool) -> Self {
        self.params.insert("useislandattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("useislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usetargetattr(mut self, val: bool) -> Self {
        self.params.insert("usetargetattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usetargetattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("usetargetattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useislandscaleattr(mut self, val: bool) -> Self {
        self.params.insert("useislandscaleattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useislandscaleattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("useislandscaleattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useislandsetattr(mut self, val: bool) -> Self {
        self.params.insert("useislandsetattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useislandsetattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("useislandsetattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_correctareas(mut self, val: bool) -> Self {
        self.params.insert("correctareas".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_correctareas_expr(mut self, expr: &str) -> Self {
        self.params.insert("correctareas".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_packbetween(mut self, val: bool) -> Self {
        self.params.insert("packbetween".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_packbetween_expr(mut self, expr: &str) -> Self {
        self.params.insert("packbetween".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_packincavities(mut self, val: bool) -> Self {
        self.params.insert("packincavities".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_packincavities_expr(mut self, expr: &str) -> Self {
        self.params.insert("packincavities".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_paddingboundary(mut self, val: bool) -> Self {
        self.params.insert("paddingboundary".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_paddingboundary_expr(mut self, expr: &str) -> Self {
        self.params.insert("paddingboundary".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_expandpadding(mut self, val: bool) -> Self {
        self.params.insert("expandpadding".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_expandpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert("expandpadding".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usedefaulttarget(mut self, val: bool) -> Self {
        self.params.insert("usedefaulttarget".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedefaulttarget_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedefaulttarget".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usedefaultudimtarget(mut self, val: bool) -> Self {
        self.params.insert("usedefaultudimtarget".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedefaultudimtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedefaultudimtarget".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rect_use_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("rect_use{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_rect_use_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("rect_use{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_udimextendfixedislands(mut self, val: bool) -> Self {
        self.params.insert("udimextendfixedislands".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_udimextendfixedislands_expr(mut self, expr: &str) -> Self {
        self.params.insert("udimextendfixedislands".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usetargetislandattr(mut self, val: bool) -> Self {
        self.params.insert("usetargetislandattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usetargetislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("usetargetislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stackislands(mut self, val: bool) -> Self {
        self.params.insert("stackislands".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_stackislands_expr(mut self, expr: &str) -> Self {
        self.params.insert("stackislands".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_invertedoverlays(mut self, val: bool) -> Self {
        self.params.insert("invertedoverlays".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertedoverlays_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertedoverlays".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stackonnongroup(mut self, val: bool) -> Self {
        self.params.insert("stackonnongroup".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_stackonnongroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("stackonnongroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_generatenonpackedpoly(mut self, val: bool) -> Self {
        self.params.insert("generatenonpackedpoly".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_generatenonpackedpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert("generatenonpackedpoly".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_generateislandattr(mut self, val: bool) -> Self {
        self.params.insert("generateislandattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_generateislandattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("generateislandattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_generatetargetattr(mut self, val: bool) -> Self {
        self.params.insert("generatetargetattr".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_generatetargetattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("generatetargetattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvlayout {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvlayout"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvpeltMethod {
    SpringRelaxation = 0,
    TutteBarycentric = 1,
    DiscreteHarmonic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvpeltUfrom {
    XCoordinate = 0,
    YCoordinate = 1,
    ZCoordinate = 2,
    UCoordinate = 3,
    VCoordinate = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvpeltVfrom {
    XCoordinate = 0,
    YCoordinate = 1,
    ZCoordinate = 2,
    UCoordinate = 3,
    VCoordinate = 4,
}

#[derive(Debug, Clone)]
pub struct SopUvpelt {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvpelt {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry for operation"
    pub fn set_input_geometry_for_operation<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry for operation" and specifies the output index of the target node.
    pub fn set_input_geometry_for_operation_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Frame"
    pub fn set_input_frame<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Frame" and specifies the output index of the target node.
    pub fn set_input_frame_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_k(mut self, val: f32) -> Self {
        self.params.insert("k".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_k_expr(mut self, expr: &str) -> Self {
        self.params.insert("k".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orientation(mut self, val: f32) -> Self {
        self.params.insert("orientation".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_orientation_expr(mut self, expr: &str) -> Self {
        self.params.insert("orientation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_urange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("urange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_urange_expr(mut self, expr: &str) -> Self {
        self.params.insert("urange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("vrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_vrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("vrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iters(mut self, val: i32) -> Self {
        self.params.insert("iters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iters_expr(mut self, expr: &str) -> Self {
        self.params.insert("iters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hintprim(mut self, val: i32) -> Self {
        self.params.insert("hintprim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hintprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("hintprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopUvpeltMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ufrom(mut self, val: SopUvpeltUfrom) -> Self {
        self.params.insert("ufrom".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_ufrom_expr(mut self, expr: &str) -> Self {
        self.params.insert("ufrom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vfrom(mut self, val: SopUvpeltVfrom) -> Self {
        self.params.insert("vfrom".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vfrom_expr(mut self, expr: &str) -> Self {
        self.params.insert("vfrom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputgroup(mut self, val: &str) -> Self {
        self.params.insert("outputgroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputgroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stiffness(mut self, val: &str) -> Self {
        self.params.insert("stiffness".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert("stiffness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_frame(mut self, val: &str) -> Self {
        self.params.insert("frame".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert("frame".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weight(mut self, val: &str) -> Self {
        self.params.insert("weight".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_weight_expr(mut self, expr: &str) -> Self {
        self.params.insert("weight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_outputtoggle(mut self, val: bool) -> Self {
        self.params.insert("outputtoggle".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_outputtoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputtoggle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_userange(mut self, val: bool) -> Self {
        self.params.insert("userange".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_userange_expr(mut self, expr: &str) -> Self {
        self.params.insert("userange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvpelt {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvpelt"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvprojectGrouptype {
    Vertices = 0,
    Points = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvprojectProjtype {
    Orthographic = 0,
    Polar = 1,
    Cylindrical = 2,
    Toroidal = 3,
    PlasticWrap = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvprojectXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvprojectRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvprojectInittype {
    ToXyPlane = 0,
    ToYzPlane = 1,
    ToZxPlane = 2,
    ToBestPlane = 3,
}

#[derive(Debug, Clone)]
pub struct SopUvproject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvproject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry on which to project UV coordinates"
    pub fn set_input_geometry_on_which_to_project_uv_coordina<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry on which to project UV coordinates" and specifies the output index of the target node.
    pub fn set_input_geometry_on_which_to_project_uv_coordina_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_initbbox(mut self) -> Self {
        self.params.insert("initbbox".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_torrad(mut self, val: f32) -> Self {
        self.params.insert("torrad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_torrad_expr(mut self, expr: &str) -> Self {
        self.params.insert("torrad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_polerad(mut self, val: f32) -> Self {
        self.params.insert("polerad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_polerad_expr(mut self, expr: &str) -> Self {
        self.params.insert("polerad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_urange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("urange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_urange_expr(mut self, expr: &str) -> Self {
        self.params.insert("urange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("vrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_vrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("vrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopUvprojectGrouptype) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_projtype(mut self, val: SopUvprojectProjtype) -> Self {
        self.params.insert("projtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_projtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("projtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: SopUvprojectXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: SopUvprojectRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inittype(mut self, val: SopUvprojectInittype) -> Self {
        self.params.insert("inittype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_inittype_expr(mut self, expr: &str) -> Self {
        self.params.insert("inittype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_fixseams(mut self, val: bool) -> Self {
        self.params.insert("fixseams".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fixseams_expr(mut self, expr: &str) -> Self {
        self.params.insert("fixseams".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fixpolar(mut self, val: bool) -> Self {
        self.params.insert("fixpolar".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fixpolar_expr(mut self, expr: &str) -> Self {
        self.params.insert("fixpolar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvproject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvproject"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvquickshadeTextureAxis {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone)]
pub struct SopUvquickshade {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvquickshade {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry to Quickly Apply UVs and Shaders"
    pub fn set_input_geometry_to_quickly_apply_uvs_and_shader<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Quickly Apply UVs and Shaders" and specifies the output index of the target node.
    pub fn set_input_geometry_to_quickly_apply_uvs_and_shader_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float3 parameters ---
    pub fn with_texture_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("texture_s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_texture_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("texture_s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_texture_axis(mut self, val: SopUvquickshadeTextureAxis) -> Self {
        self.params.insert("texture_axis".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_texture_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert("texture_axis".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_texture(mut self, val: &str) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvquickshade {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvquickshade"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvtransformGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvtransformXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvtransformRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvtransformType {
    Linear = 0,
    Quadratic = 1,
    Cubic = 2,
    /// Meta-ball
    MetaMinusBall = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvtransformMetric {
    Uv = 0,
    Uvw = 1,
    Xyz = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvtransformVisualizefalloff {
    Never = 0,
    Always = 1,
    WhenViewportToolIsActive = 2,
}

#[derive(Debug, Clone)]
pub struct SopUvtransform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvtransform {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "UVs to Transform"
    pub fn set_input_uvs_to_transform<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "UVs to Transform" and specifies the output index of the target node.
    pub fn set_input_uvs_to_transform_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_tandeg(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tandeg".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tandeg_expr(mut self, expr: &str) -> Self {
        self.params.insert("tandeg".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert("shear".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert("shear".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopUvtransformGrouptype) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert("grouptype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: SopUvtransformXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: SopUvtransformRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type(mut self, val: SopUvtransformType) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_metric(mut self, val: SopUvtransformMetric) -> Self {
        self.params.insert("metric".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert("metric".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visualizefalloff(mut self, val: SopUvtransformVisualizefalloff) -> Self {
        self.params.insert("visualizefalloff".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_visualizefalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("visualizefalloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_kernel(mut self, val: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_global(mut self, val: bool) -> Self {
        self.params.insert("global".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_global_expr(mut self, expr: &str) -> Self {
        self.params.insert("global".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvglobal(mut self, val: bool) -> Self {
        self.params.insert("uvglobal".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uvglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvglobal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvtransform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvtransform"
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvunwrapNplanes {
    /// 4
    N4 = 0,
    /// 5
    N5 = 1,
    /// 6
    N6 = 2,
    /// 8
    N8 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvunwrapLayout {
    Strip = 0,
    Square = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvunwrapScale {
    None = 0,
    Uniform = 1,
    Stretch = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopUvunwrapRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct SopUvunwrap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopUvunwrap {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Texture Source"
    pub fn set_input_texture_source<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Texture Source" and specifies the output index of the target node.
    pub fn set_input_texture_source_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Planes"
    pub fn set_input_planes<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Planes" and specifies the output index of the target node.
    pub fn set_input_planes_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_spacing(mut self, val: f32) -> Self {
        self.params.insert("spacing".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_spacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("spacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_nplanes(mut self, val: SopUvunwrapNplanes) -> Self {
        self.params.insert("nplanes".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_nplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert("nplanes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layout(mut self, val: SopUvunwrapLayout) -> Self {
        self.params.insert("layout".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_layout_expr(mut self, expr: &str) -> Self {
        self.params.insert("layout".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: SopUvunwrapScale) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: SopUvunwrapRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_planegroup(mut self, val: &str) -> Self {
        self.params.insert("planegroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_planegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("planegroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopUvunwrap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "uvunwrap"
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
