#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopObjnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopObjnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopObjnetPreXform {
    CleanTransform = 0,
    CleanTranslates = 1,
    CleanRotates = 2,
    CleanScales = 3,
    /// Extract Pre-transform
    ExtractPreMinusTransform = 4,
    /// Reset Pre-transform
    ResetPreMinusTransform = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopObjnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct CopObjnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopObjnet {
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: CopObjnetXord) -> Self {
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
    pub fn with_rord(mut self, val: CopObjnetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: CopObjnetPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: CopObjnetUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
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
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopObjnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "objnet"
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
pub enum CopOciotransformMethod {
    BetweenSpaces = 0,
    SpaceToView = 1,
}

#[derive(Debug, Clone)]
pub struct CopOciotransform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopOciotransform {
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

    // --- Menu parameters ---
    pub fn with_method(mut self, val: CopOciotransformMethod) -> Self {
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
    pub fn with_sourcespace(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcespace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcespace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcespace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tospace(mut self, val: &str) -> Self {
        self.params.insert(
            "tospace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tospace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tospace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_todisplay(mut self, val: &str) -> Self {
        self.params.insert(
            "todisplay".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_todisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "todisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toview(mut self, val: &str) -> Self {
        self.params.insert(
            "toview".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toview".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_invertxform(mut self, val: bool) -> Self {
        self.params.insert(
            "invertxform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invertxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invertxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopOciotransform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ociotransform"
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
pub enum CopOnnxModelInputVolorder {
    Yx = 0,
    Xy = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOnnxModelOutputVolorder {
    Yx = 0,
    Xy = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOnnxInputType {
    Mono = 0,
    Uv = 1,
    Rgb = 2,
    Rgba = 3,
    Geometry = 4,
    Any = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOnnxNodeInputVolorder {
    Yx = 0,
    Xy = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOnnxResampleFilter {
    Point = 0,
    Bilinear = 1,
    Box = 2,
    Bartlett = 3,
    /// Catmull-Rom
    CatmullMinusRom = 4,
    Mitchell = 5,
    /// B-spline
    BMinusSpline = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOnnxOutputType {
    Infer = 0,
    Mono = 1,
    Uv = 2,
    Rgb = 3,
    Rgba = 4,
    Geometry = 5,
    Any = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOnnxNodeOutputVolorder {
    Yx = 0,
    Xy = 1,
}

#[derive(Debug, Clone)]
pub struct CopOnnx {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopOnnx {
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

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_setupshapes(mut self) -> Self {
        self.params.insert(
            "setupshapes".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_output_deduceshape(mut self) -> Self {
        self.params.insert(
            "output_deduceshape".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_input_brightscale_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("input_brightscale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_input_brightscale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_brightscale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_brightscale_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("output_brightscale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_output_brightscale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_brightscale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxbatch(mut self, val: i32) -> Self {
        self.params.insert(
            "maxbatch".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_input_channelsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("model_input_channelsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_model_input_channelsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_channelsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_output_channelsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("model_output_channelsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_model_output_channelsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_output_channelsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_channelsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("node_output_channelsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_node_output_channelsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_channelsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_resample_size_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.params.insert(
            format!("resample_size{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_resample_size_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resample_size{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- IntArray parameters ---
    pub fn with_model_input_shape_inst(mut self, index1: usize, val: Vec<i32>) -> Self {
        self.params.insert(
            format!("model_input_shape{}", index1),
            crate::core::types::ParamValue::IntArray(val),
        );
        self
    }
    pub fn with_model_input_shape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_shape{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_output_shape_inst(mut self, index1: usize, val: Vec<i32>) -> Self {
        self.params.insert(
            format!("model_output_shape{}", index1),
            crate::core::types::ParamValue::IntArray(val),
        );
        self
    }
    pub fn with_model_output_shape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_output_shape{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_shape_inst(mut self, index1: usize, val: Vec<i32>) -> Self {
        self.params.insert(
            format!("node_output_shape{}", index1),
            crate::core::types::ParamValue::IntArray(val),
        );
        self
    }
    pub fn with_node_output_shape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_shape{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_provider(mut self, val: i32) -> Self {
        self.params.insert(
            "provider".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_provider_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "provider".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_input_volorder_inst(
        mut self,
        index1: usize,
        val: CopOnnxModelInputVolorder,
    ) -> Self {
        self.params.insert(
            format!("model_input_volorder{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_model_input_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_volorder{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_output_volorder_inst(
        mut self,
        index1: usize,
        val: CopOnnxModelOutputVolorder,
    ) -> Self {
        self.params.insert(
            format!("model_output_volorder{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_model_output_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_output_volorder{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_type_inst(mut self, index1: usize, val: CopOnnxInputType) -> Self {
        self.params.insert(
            format!("input{}_type", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_input_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input{}_type", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_input_volorder_inst(
        mut self,
        index1: usize,
        val: CopOnnxNodeInputVolorder,
    ) -> Self {
        self.params.insert(
            format!("node_input_volorder{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_node_input_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_input_volorder{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resample_filter_inst(mut self, index1: usize, val: CopOnnxResampleFilter) -> Self {
        self.params.insert(
            format!("resample_filter{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resample_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resample_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_type_inst(mut self, index1: usize, val: CopOnnxOutputType) -> Self {
        self.params.insert(
            format!("output{}_type", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_type", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_volorder_inst(
        mut self,
        index1: usize,
        val: CopOnnxNodeOutputVolorder,
    ) -> Self {
        self.params.insert(
            format!("node_output_volorder{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_node_output_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_volorder{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_modelfile(mut self, val: &str) -> Self {
        self.params.insert(
            "modelfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modelfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_input_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("model_input_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_model_input_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_input_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("model_input_data{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_model_input_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_data{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_output_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("model_output_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_model_output_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_output_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_input_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("node_input_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_node_input_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_input_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_scalemenu_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("input_scalemenu{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_input_scalemenu_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_scalemenu{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("node_output_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_node_output_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("node_output_data{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_node_output_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_data{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_scalemenu_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("output_scalemenu{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_scalemenu_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_scalemenu{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_domaxbatch(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxbatch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_input_channelfirst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("model_input_channelfirst{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_model_input_channelfirst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_channelfirst{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_input_customchannel_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("model_input_customchannel{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_model_input_customchannel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_input_customchannel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_output_channelfirst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("model_output_channelfirst{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_model_output_channelfirst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_output_channelfirst{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_model_output_customchannel_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("model_output_customchannel{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_model_output_customchannel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("model_output_customchannel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_flip_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("input_flip{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_input_flip_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_flip{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_input_channelfirst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("node_input_channelfirst{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_node_input_channelfirst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_input_channelfirst{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resample_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resample_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resample_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resample_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_brightenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("input_brightenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_input_brightenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_brightenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_autodeduce_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("node_output_autodeduce{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_node_output_autodeduce_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_autodeduce{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_flip_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("output_flip{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_output_flip_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_flip{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_channelfirst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("node_output_channelfirst{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_node_output_channelfirst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_channelfirst{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_output_customchannel_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("node_output_customchannel{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_node_output_customchannel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("node_output_customchannel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_brightenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("output_brightenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_output_brightenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_brightenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopOnnx {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "onnx"
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
pub enum CopOpenclOptionsRunover {
    FirstWriteableLayer = 0,
    FirstWriteableAttribute = 1,
    FirstWriteableVolume = 2,
    FirstWriteableVdb = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclOptionsTimemethod {
    Timestep = 0,
    /// e ^ Timestep
    ETimestep = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclFilter {
    Point = 0,
    Bilinear = 1,
    Box = 2,
    Bartlett = 3,
    /// Catmull-Rom
    CatmullMinusRom = 4,
    Mitchell = 5,
    /// B-spline
    BMinusSpline = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclOptionsPrecision {
    Auto = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclInputType {
    VaryingLayer = 0,
    Id = 1,
    Mono = 2,
    Uv = 3,
    Rgb = 4,
    Rgba = 5,
    Geometry = 6,
    Metadata = 7,
    IntegerVdb = 8,
    FloatVdb = 9,
    VectorVdb = 10,
    VaryingVdb = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclOutputType {
    VaryingLayer = 0,
    Id = 1,
    Mono = 2,
    Uv = 3,
    Rgb = 4,
    Rgba = 5,
    Geometry = 6,
    IntegerVdb = 7,
    FloatVdb = 8,
    VectorVdb = 9,
    VaryingVdb = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclOutputMetadata {
    FirstInput = 0,
    MatchingName = 1,
    InputName = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclOutputPrecision {
    InputPrecision = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclOutputTypeinfo {
    Input = 0,
    None = 1,
    Color = 2,
    Position = 3,
    Vector = 4,
    SignedNormal = 5,
    OffsetNormal = 6,
    TextureCoordinate = 7,
    Id = 8,
    Mask = 9,
    Sdf = 10,
    Height = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsType {
    Integer = 0,
    Float = 1,
    Vector2 = 2,
    Vector = 3,
    Vector4 = 4,
    Ramp = 5,
    Layer = 6,
    Geometry = 7,
    Volume = 8,
    Vdb = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsRamptype {
    Float = 0,
    Vector = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsLayertype {
    Integer = 0,
    Float = 1,
    Float2 = 2,
    Float3 = 3,
    Float4 = 4,
    /// Float?
    Float1 = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsLayerborder {
    Input = 0,
    Constant = 1,
    Clamp = 2,
    Mirror = 3,
    Wrap = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsVdbtype {
    Any = 0,
    Float = 1,
    Vector = 2,
    Integer = 3,
    /// Float?
    Float1 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsAttribclass {
    Detail = 0,
    Primitive = 1,
    Point = 2,
    Vertex = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsAttribtype {
    Float = 0,
    Integer = 1,
    FloatArray = 2,
    IntegerArray = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsPrecision {
    Node = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopOpenclBindingsTimescale {
    None = 0,
    Timestep = 1,
    /// 1 / Timestep
    N1Timestep = 2,
    /// e ^ Timestep
    ETimestep = 3,
}

#[derive(Debug, Clone)]
pub struct CopOpencl {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopOpencl {
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

    // --- Button parameters ---
    pub fn trigger_displaycode(mut self) -> Self {
        self.params.insert(
            "displaycode".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_options_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "options_timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_options_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_fval_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("bindings{}_fval", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bindings_fval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_fval", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_bindings_v2val_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("bindings{}_v2val", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bindings_v2val_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_v2val", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_bindings_v3val_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("bindings{}_v3val", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bindings_v3val_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_v3val", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_bindings_v4val_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("bindings{}_v4val", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_bindings_v4val_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_v4val", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_options_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "options_iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_options_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_rampsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("bindings{}_rampsize", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bindings_rampsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_rampsize", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_attribsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("bindings{}_attribsize", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bindings_attribsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_attribsize", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_intval_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("bindings{}_intval", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bindings_intval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_intval", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_options_tile(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "options_tile".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_options_tile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_tile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_options_vdbtile(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "options_vdbtile".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_options_vdbtile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_vdbtile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_options_runover(mut self, val: CopOpenclOptionsRunover) -> Self {
        self.params.insert(
            "options_runover".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_options_runover_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_runover".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_timemethod(mut self, val: CopOpenclOptionsTimemethod) -> Self {
        self.params.insert(
            "options_timemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_options_timemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_timemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: CopOpenclFilter) -> Self {
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
    pub fn with_options_precision(mut self, val: CopOpenclOptionsPrecision) -> Self {
        self.params.insert(
            "options_precision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_options_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_type_inst(mut self, index1: usize, val: CopOpenclInputType) -> Self {
        self.params.insert(
            format!("input{}_type", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_input_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input{}_type", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_type_inst(mut self, index1: usize, val: CopOpenclOutputType) -> Self {
        self.params.insert(
            format!("output{}_type", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_type", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_metadata_inst(
        mut self,
        index1: usize,
        val: CopOpenclOutputMetadata,
    ) -> Self {
        self.params.insert(
            format!("output{}_metadata", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_metadata_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_metadata", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_precision_inst(
        mut self,
        index1: usize,
        val: CopOpenclOutputPrecision,
    ) -> Self {
        self.params.insert(
            format!("output{}_precision", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_precision", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_typeinfo_inst(
        mut self,
        index1: usize,
        val: CopOpenclOutputTypeinfo,
    ) -> Self {
        self.params.insert(
            format!("output{}_typeinfo", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_typeinfo_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_typeinfo", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_type_inst(mut self, index1: usize, val: CopOpenclBindingsType) -> Self {
        self.params.insert(
            format!("bindings{}_type", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_type", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_ramptype_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsRamptype,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_ramptype", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_ramptype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_ramptype", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_layertype_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsLayertype,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_layertype", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_layertype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_layertype", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_layerborder_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsLayerborder,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_layerborder", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_layerborder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_layerborder", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_vdbtype_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsVdbtype,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_vdbtype", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_vdbtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_vdbtype", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_attribclass_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsAttribclass,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_attribclass", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_attribclass_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_attribclass", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_attribtype_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsAttribtype,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_attribtype", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_attribtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_attribtype", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_precision_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsPrecision,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_precision", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_precision", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_timescale_inst(
        mut self,
        index1: usize,
        val: CopOpenclBindingsTimescale,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_timescale", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindings_timescale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_timescale", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_bindings_ramp_inst(
        mut self,
        index1: usize,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_ramp", index1),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_bindings_ramp_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_ramp", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_ramp_rgb_inst(
        mut self,
        index1: usize,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            format!("bindings{}_ramp_rgb", index1),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_bindings_ramp_rgb_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_ramp_rgb", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_kernelcode(mut self, val: &str) -> Self {
        self.params.insert(
            "kernelcode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernelcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernelcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernelname(mut self, val: &str) -> Self {
        self.params.insert(
            "kernelname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_writebackkernelname(mut self, val: &str) -> Self {
        self.params.insert(
            "writebackkernelname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_writebackkernelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writebackkernelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kerneloptions(mut self, val: &str) -> Self {
        self.params.insert(
            "kerneloptions".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kerneloptions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kerneloptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("input{}_name", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_input_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input{}_name", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("output{}_name", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_name", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_metaname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("output{}_metaname", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_metaname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output{}_metaname", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("bindings{}_name", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindings_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_name", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_portname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("bindings{}_portname", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindings_portname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_portname", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_volume_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("bindings{}_volume", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindings_volume_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_volume", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_attribute_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("bindings{}_attribute", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindings_attribute_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_attribute", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatedcode(mut self, val: &str) -> Self {
        self.params.insert(
            "generatedcode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_generatedcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatedcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_options_usevdbtile(mut self, val: bool) -> Self {
        self.params.insert(
            "options_usevdbtile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_usevdbtile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_usevdbtile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_iteration(mut self, val: bool) -> Self {
        self.params.insert(
            "options_iteration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_iteration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_iteration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usewritebackkernel(mut self, val: bool) -> Self {
        self.params.insert(
            "usewritebackkernel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usewritebackkernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usewritebackkernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_time(mut self, val: bool) -> Self {
        self.params.insert(
            "options_time".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_timeinccontext(mut self, val: bool) -> Self {
        self.params.insert(
            "options_timeinccontext".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_timeinccontext_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_timeinccontext".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_timeinc(mut self, val: bool) -> Self {
        self.params.insert(
            "options_timeinc".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_timeinc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_timeinc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_importprequel(mut self, val: bool) -> Self {
        self.params.insert(
            "options_importprequel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_importprequel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_importprequel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_options_timing(mut self, val: bool) -> Self {
        self.params.insert(
            "options_timing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_options_timing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "options_timing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_optional_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("input{}_optional", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_input_optional_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input{}_optional", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_forcealign_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_forcealign", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_forcealign_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_forcealign", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_resolution_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_resolution", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_resolution_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_resolution", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_voxelsize_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_voxelsize", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_voxelsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_voxelsize", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_xformtoworld_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_xformtoworld", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_xformtoworld_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_xformtoworld", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_xformtovoxel_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_xformtovoxel", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_xformtovoxel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_xformtovoxel", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_readable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_readable", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_readable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_readable", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_writeable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_writeable", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_writeable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_writeable", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_optional_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_optional", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_optional_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_optional", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindings_defval_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindings{}_defval", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindings_defval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_defval", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopOpencl {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "opencl"
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
pub struct CopOutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopOutput {
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

impl crate::core::types::HoudiniNode for CopOutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "output"
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
