#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectViewportisolatorPreXform {
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
pub enum ObjectViewportisolatorXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectViewportisolatorRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectViewportisolatorUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectViewportisolatorMode {
    Isolate = 0,
    Exclude = 1,
    Append = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectViewportisolator {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectViewportisolator {
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
    pub fn trigger_isolate(mut self) -> Self {
        self.params.insert(
            "isolate".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_append(mut self) -> Self {
        self.params.insert(
            "append".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_exclude(mut self) -> Self {
        self.params.insert(
            "exclude".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_apply(mut self) -> Self {
        self.params.insert(
            "apply".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_clear(mut self) -> Self {
        self.params.insert(
            "clear".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_recompute(mut self) -> Self {
        self.params.insert(
            "recompute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_save(mut self) -> Self {
        self.params.insert(
            "save".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_recall_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("recall{}", index1),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode_inst(mut self, index1: usize, val: ObjectViewportisolatorMode) -> Self {
        self.params.insert(
            format!("mode{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mode{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectViewportisolatorPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectViewportisolatorXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectViewportisolatorRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectViewportisolatorUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_load(mut self, val: i32) -> Self {
        self.params.insert(
            "load".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_load_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "load".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isolatestring(mut self, val: &str) -> Self {
        self.params.insert(
            "isolatestring".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_isolatestring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isolatestring".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewports(mut self, val: &str) -> Self {
        self.params.insert(
            "viewports".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_viewports_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewports".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label(mut self, val: &str) -> Self {
        self.params.insert(
            "label".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mask_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("mask{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mask{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_history_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("history{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_history_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("history{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_more(mut self, val: bool) -> Self {
        self.params.insert(
            "more".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_more_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "more".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectViewportisolator {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "viewportisolator"
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
pub struct ObjectVopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectVopnet {
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

impl houdini_ramen_core::types::HoudiniNode for ObjectVopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopnet"
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
pub enum ObjectVrcamXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectVrcamRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectVrcamPreXform {
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
pub enum ObjectVrcamUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectVrcamProjection {
    Perspective = 0,
    Orthographic = 1,
    /// Polar (panoramic)
    PolarPanoramic = 2,
    /// Cylindrical (panoramic)
    CylindricalPanoramic = 3,
    LensShader = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectVrcamVrlayout {
    /// Left/Right
    LeftRight = 0,
    /// Left Over/Right Under
    LeftOverRightUnder = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectVrcamVrprojection {
    Latlong = 0,
    Perspective = 1,
    /// Cube Map - NVIDIA
    CubeMapMinusNvidia = 2,
    /// Cube Map - 3x2
    CubeMapMinus3x2 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectVrcamVrmergemode {
    None = 0,
    Linear = 1,
    Smooth = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectVrcam {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectVrcam {
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

    /// Connects to input 0: "parent"
    pub fn set_input_parent<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iconscale(mut self, val: f32) -> Self {
        self.params.insert(
            "iconscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iconscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iconscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focal(mut self, val: f32) -> Self {
        self.params.insert(
            "focal".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperture(mut self, val: f32) -> Self {
        self.params.insert(
            "aperture".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orthowidth(mut self, val: f32) -> Self {
        self.params.insert(
            "orthowidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_orthowidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orthowidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_near(mut self, val: f32) -> Self {
        self.params.insert(
            "near".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "near".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_far(mut self, val: f32) -> Self {
        self.params.insert(
            "far".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "far".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cropl(mut self, val: f32) -> Self {
        self.params.insert(
            "cropl".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cropr(mut self, val: f32) -> Self {
        self.params.insert(
            "cropr".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cropb(mut self, val: f32) -> Self {
        self.params.insert(
            "cropb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cropt(mut self, val: f32) -> Self {
        self.params.insert(
            "cropt".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropt".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shutter(mut self, val: f32) -> Self {
        self.params.insert(
            "shutter".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shutter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focus(mut self, val: f32) -> Self {
        self.params.insert(
            "focus".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focus".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fstop(mut self, val: f32) -> Self {
        self.params.insert(
            "fstop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fstop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fstop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_bokehrotation(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_bokehrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_bokehrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_bokehrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrmergeangle(mut self, val: f32) -> Self {
        self.params.insert(
            "vrmergeangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrmergeangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrmergeangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrhorizontalfov(mut self, val: f32) -> Self {
        self.params.insert(
            "vrhorizontalfov".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrhorizontalfov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrhorizontalfov".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrverticalfov(mut self, val: f32) -> Self {
        self.params.insert(
            "vrverticalfov".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrverticalfov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrverticalfov".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrperspectivefov(mut self, val: f32) -> Self {
        self.params.insert(
            "vrperspectivefov".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrperspectivefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrperspectivefov".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrperspectiveclipnear(mut self, val: f32) -> Self {
        self.params.insert(
            "vrperspectiveclipnear".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrperspectiveclipnear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrperspectiveclipnear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrperspectiveclipfar(mut self, val: f32) -> Self {
        self.params.insert(
            "vrperspectiveclipfar".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrperspectiveclipfar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrperspectiveclipfar".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrperspectivedistort(mut self, val: f32) -> Self {
        self.params.insert(
            "vrperspectivedistort".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrperspectivedistort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrperspectivedistort".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrperspectivedistortcubic(mut self, val: f32) -> Self {
        self.params.insert(
            "vrperspectivedistortcubic".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrperspectivedistortcubic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrperspectivedistortcubic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vreyeseparation(mut self, val: f32) -> Self {
        self.params.insert(
            "vreyeseparation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vreyeseparation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vreyeseparation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vreyetoneckdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "vreyetoneckdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vreyetoneckdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vreyetoneckdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_win(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "win".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_win_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "win".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_winsize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "winsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_winsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrlayout(mut self, val: ObjectVrcamVrlayout) -> Self {
        self.params.insert(
            "vrlayout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vrlayout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrlayout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrprojection(mut self, val: ObjectVrcamVrprojection) -> Self {
        self.params.insert(
            "vrprojection".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vrprojection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrprojection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrmergemode(mut self, val: ObjectVrcamVrmergemode) -> Self {
        self.params.insert(
            "vrmergemode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vrmergemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrmergemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectVrcamXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectVrcamRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectVrcamPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectVrcamUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projection(mut self, val: ObjectVrcamProjection) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_lensshader(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_lensshader".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_lensshader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_lensshader".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focalunits(mut self, val: &str) -> Self {
        self.params.insert(
            "focalunits".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_focalunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focalunits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_background(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_background".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_background_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_background".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_winmask(mut self, val: &str) -> Self {
        self.params.insert(
            "winmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_winmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cropmask(mut self, val: &str) -> Self {
        self.params.insert(
            "cropmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cropmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_bokeh(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_bokeh".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_bokeh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_bokeh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_bokehfile(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_bokehfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_bokehfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_bokehfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_bgenable(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_bgenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_bgenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_bgenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrswapleftright(mut self, val: bool) -> Self {
        self.params.insert(
            "vrswapleftright".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vrswapleftright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrswapleftright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrpreserveaspectratio(mut self, val: bool) -> Self {
        self.params.insert(
            "vrpreserveaspectratio".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vrpreserveaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrpreserveaspectratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrusestereoeye(mut self, val: bool) -> Self {
        self.params.insert(
            "vrusestereoeye".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vrusestereoeye_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrusestereoeye".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectVrcam {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vrcam"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectVrcamInnerExt {
    fn camorigin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn constraints(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn file1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectVrcamInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectVrcam> {
    fn camorigin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("camOrigin")
    }
    fn constraints(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("constraints")
    }
    fn file1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("file1")
    }
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform1")
    }
}
