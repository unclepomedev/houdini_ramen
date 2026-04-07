#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopObjectMergeXformtype {
    None = 0,
    IntoThisObject = 1,
    IntoSpecifiedObject = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopObjectMergePivot {
    Origin = 0,
    Centroid = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopObjectMergeViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone)]
pub struct SopObjectMerge {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopObjectMerge {
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

    // --- Menu parameters ---
    pub fn with_xformtype(mut self, val: SopObjectMergeXformtype) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pivot(mut self, val: SopObjectMergePivot) -> Self {
        self.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: SopObjectMergeViewportlod) -> Self {
        self.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("objpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("objpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("group{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("group{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformpath(mut self, val: &str) -> Self {
        self.params.insert(
            "xformpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptgroupprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "ptgroupprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptgroupprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptgroupprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primgroupprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "primgroupprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primgroupprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primgroupprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
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
    pub fn with_expand_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("expand{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expand_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("expand{}", index1),
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
    pub fn with_createptgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "createptgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createptgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createptgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createprimgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "createprimgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprimgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbosegroups(mut self, val: bool) -> Self {
        self.params.insert(
            "verbosegroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbosegroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbosegroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_suffixfirstgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "suffixfirstgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_suffixfirstgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "suffixfirstgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createptstring(mut self, val: bool) -> Self {
        self.params.insert(
            "createptstring".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createptstring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createptstring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createprimstring(mut self, val: bool) -> Self {
        self.params.insert(
            "createprimstring".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimstring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprimstring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pack(mut self, val: bool) -> Self {
        self.params.insert(
            "pack".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pack_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addpath(mut self, val: bool) -> Self {
        self.params.insert(
            "addpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopObjectMerge {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "object_merge"
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
pub enum SopObjnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopObjnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopObjnetPreXform {
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
pub enum SopObjnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct SopObjnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopObjnet {
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
    pub fn with_xord(mut self, val: SopObjnetXord) -> Self {
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
    pub fn with_rord(mut self, val: SopObjnetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: SopObjnetPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: SopObjnetUparmtype) -> Self {
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

impl crate::core::types::HoudiniNode for SopObjnet {
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
pub enum SopOceanevaluateDepthfalloff {
    None = 0,
    Exponential = 1,
    ExponentialByFrequency = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanevaluateSurfacemethod {
    PointSampling = 0,
    Projection = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanevaluateUniformsamples {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanevaluateTrange {
    SaveCurrentFrame = 0,
    SaveFrameRange = 1,
    /// Save Frame Range Only (Strict)
    SaveFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanevaluateChannels {
    DisplacementInRgb = 0,
    /// Displacement in RGB, Cusp in Alpha
    DisplacementInRgbCuspInAlpha = 1,
    CuspInAlpha = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanevaluateDepth {
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 0,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 1,
}

#[derive(Debug, Clone)]
pub struct SopOceanevaluate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOceanevaluate {
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

    /// Connects to input 0: "Geometry to Deform"
    pub fn set_input_geometry_to_deform<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Deform" and specifies the output index of the target node.
    pub fn set_input_geometry_to_deform_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Ocean Spectrum"
    pub fn set_input_ocean_spectrum<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Ocean Spectrum" and specifies the output index of the target node.
    pub fn set_input_ocean_spectrum_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacedepth(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacedepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacedepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacedepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volsmooth(mut self, val: f32) -> Self {
        self.params.insert(
            "volsmooth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volsmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volsmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gravity(mut self, val: f32) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxextrapolate(mut self, val: f32) -> Self {
        self.params.insert(
            "maxextrapolate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxextrapolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxextrapolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterseed(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_minrestdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "minrestdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minrestdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minrestdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxrestdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "maxrestdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxrestdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxrestdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_ptsize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ptsize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ptsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptt(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ptt".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ptt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptpadscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ptpadscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ptpadscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptpadscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_downsample(mut self, val: i32) -> Self {
        self.params.insert(
            "downsample".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_downsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "downsample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_depthdivs(mut self, val: i32) -> Self {
        self.params.insert(
            "depthdivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthdivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sololayer(mut self, val: i32) -> Self {
        self.params.insert(
            "sololayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sololayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sololayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplediv(mut self, val: i32) -> Self {
        self.params.insert(
            "samplediv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samplediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdisplaceframe(mut self, val: i32) -> Self {
        self.params.insert(
            "maxdisplaceframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxdisplaceframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdisplaceframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_bakeres(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bakeres".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bakeres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_divs(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_divs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_depthfalloff(mut self, val: SopOceanevaluateDepthfalloff) -> Self {
        self.params.insert(
            "depthfalloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depthfalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthfalloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacemethod(mut self, val: SopOceanevaluateSurfacemethod) -> Self {
        self.params.insert(
            "surfacemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surfacemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformsamples(mut self, val: SopOceanevaluateUniformsamples) -> Self {
        self.params.insert(
            "uniformsamples".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trange(mut self, val: SopOceanevaluateTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channels(mut self, val: SopOceanevaluateChannels) -> Self {
        self.params.insert(
            "channels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_channels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: SopOceanevaluateDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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

    // --- Ramp parameters ---
    pub fn with_cuspramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "cuspramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_cuspramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_outputgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "outputgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "copoutput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_domaxres(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosololayer(mut self, val: bool) -> Self {
        self.params.insert(
            "dosololayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosololayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosololayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "deformgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deformgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deformgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewgrid(mut self, val: bool) -> Self {
        self.params.insert(
            "previewgrid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_previewgrid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewgrid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointvel(mut self, val: bool) -> Self {
        self.params.insert(
            "pointvel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pointvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uv(mut self, val: bool) -> Self {
        self.params.insert(
            "uv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cusp(mut self, val: bool) -> Self {
        self.params.insert(
            "cusp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cusp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cusp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspdir(mut self, val: bool) -> Self {
        self.params.insert(
            "cuspdir".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cuspdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vizcusp(mut self, val: bool) -> Self {
        self.params.insert(
            "vizcusp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vizcusp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vizcusp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface(mut self, val: bool) -> Self {
        self.params.insert(
            "surface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightfield(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfield".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: bool) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxextrapolate(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxextrapolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxextrapolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxextrapolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_estimatemaxdisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "estimatemaxdisplace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_estimatemaxdisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "estimatemaxdisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "restdisplace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restdisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restvel(mut self, val: bool) -> Self {
        self.params.insert(
            "restvel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restderivs(mut self, val: bool) -> Self {
        self.params.insert(
            "restderivs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restderivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restderivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptfromvol(mut self, val: bool) -> Self {
        self.params.insert(
            "ptfromvol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ptfromvol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptfromvol".to_string(),
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
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakesingle(mut self, val: bool) -> Self {
        self.params.insert(
            "bakesingle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakesingle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakesingle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOceanevaluate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "oceanevaluate"
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
pub trait SopOceanevaluateInnerExt {
    fn n_2d_grid_points(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn n_2d_grid_polysoup(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn heightfield(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_amplitude(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_first_amplitude(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn max_displace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn restcusp_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn restderivs_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn restdisplace_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn restvel_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vol_input(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn add_pscale1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn align_3d_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn align_downsampled2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn assign_layers(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bake_maps(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast_above_mindepth(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast_above_mindepth1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast_layer_0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast_outside(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bound_pointmask(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn build_displaced_height2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn build_sdf(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn build_vel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn calc_u_v_w(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn calc_x_y_z(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn calc_x_y_z_displace_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn calc_x_y_z_vel_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn calc_cusp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn calc_derivatives_xz(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compute_pressure(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy_attributes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy_attributes2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn create_3d_volume1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn create_layer_attrib(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn del_inside_group(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn derivatives_xz_fft(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displace3d_fft(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displace_fft(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displace_grid(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displace_lowres_grid(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displace_points_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displace_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn displaced_height_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn downsamped_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn downsample(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn exp_falloff_disp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn exp_falloff_vel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn find_max_displace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin12(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin13(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin14(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin16(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end12(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end13(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end15(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_input(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_input1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_input2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_input3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_input4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_input5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn generate_points(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn generate_points1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group_inside(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group_inside1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn heightfield1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hide_mask_primitives1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hide_mask_volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index_attributes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_box(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_box_vel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn intersect_grid1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_amplitude_1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_pointmasks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_restcusp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_restderivs(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_restdisplace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_restdisplace_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_restvel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_restvel_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn keep_spectrum(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lowres_ptgrid(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge_masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merged_displace_and_masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merged_displace_v_and_masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merged_v_and_masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merged_volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_calculation_volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_downsampled_volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_output_volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_restcusp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_restderivs(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_restdisplace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_restdisplace_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_restvel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_restvel_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null13(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null14(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointjitter1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointjitter2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pressure_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn preview_grid(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn promote_maxdisplace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn promote_maxpscale(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename_inside_group(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn reset_layer_0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ropnet1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sample_attributes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sample_restvelocity_and_displace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sample_restvelocity_and_displace1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surface_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_add_velocity2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_add_velocity3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_create_uvs(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_cusp2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_cusp3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_deform_geometry2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_displace_2d_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_do_preview(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_doheighfield(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_dopressure(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_dosurface(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_dovel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_downsample(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_estimate_at_frame(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_exponential1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_has_geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_has_pointmasks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_has_points(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_height_method(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_output_group(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_output_mask(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_output_restcusp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_output_restderivs(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_output_restdisplace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_output_restvel(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_per_wavenumber(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_per_wavenumber1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_ptfromvol(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_sample(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_single_layer(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_solo_layer(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_vel_2d_3d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_vizcusp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_vizcusp1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeshift_to_frame(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn uvproject2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn uvproject_single(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vel3d_fft(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vel_fft(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vel_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn viz_cusp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn viz_cusp1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume_from_displace_attrib(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume_maxdisplace(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumefromattrib1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumemix1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> SopOceanevaluateInnerExt for crate::core::graph::InnerGraph<'a> {
    fn n_2d_grid_points(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("2d_grid_points")
    }
    fn n_2d_grid_polysoup(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("2d_grid_polysoup")
    }
    fn heightfield(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("HEIGHTFIELD")
    }
    fn input_geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("INPUT_GEO")
    }
    fn keep_amplitude(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("KEEP_amplitude")
    }
    fn keep_first_amplitude(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("KEEP_first_amplitude")
    }
    fn masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MASKS")
    }
    fn max_displace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MAX_DISPLACE")
    }
    fn restcusp_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RESTCUSP_2D")
    }
    fn restderivs_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RESTDERIVS_2D")
    }
    fn restdisplace_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RESTDISPLACE_2D")
    }
    fn restvel_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RESTVEL_2D")
    }
    fn volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("VOLUMES")
    }
    fn vol_input(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("VOL_INPUT")
    }
    fn add_pscale1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("add_pscale1")
    }
    fn align_3d_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("align_3d_volume")
    }
    fn align_downsampled2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("align_downsampled2")
    }
    fn assign_layers(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("assign_layers")
    }
    fn bake_maps(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("bake_maps")
    }
    fn blast1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blast1")
    }
    fn blast_above_mindepth(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blast_above_mindepth")
    }
    fn blast_above_mindepth1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blast_above_mindepth1")
    }
    fn blast_layer_0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blast_layer_0")
    }
    fn blast_outside(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blast_outside")
    }
    fn bound_pointmask(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("bound_pointmask")
    }
    fn build_displaced_height2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("build_displaced_height2")
    }
    fn build_sdf(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("build_sdf")
    }
    fn build_vel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("build_vel")
    }
    fn calc_u_v_w(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("calc_U_V_W")
    }
    fn calc_x_y_z(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("calc_X_Y_Z")
    }
    fn calc_x_y_z_displace_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("calc_X_Y_Z_displace_3d")
    }
    fn calc_x_y_z_vel_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("calc_X_Y_Z_vel_3d")
    }
    fn calc_cusp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("calc_cusp")
    }
    fn calc_derivatives_xz(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("calc_derivatives_XZ")
    }
    fn compute_pressure(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("compute_pressure")
    }
    fn copy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("copy1")
    }
    fn copy2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("copy2")
    }
    fn copy3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("copy3")
    }
    fn copy_attributes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("copy_attributes")
    }
    fn copy_attributes2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("copy_attributes2")
    }
    fn create_3d_volume1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("create_3d_volume1")
    }
    fn create_layer_attrib(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("create_layer_attrib")
    }
    fn del_inside_group(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("del_inside_group")
    }
    fn derivatives_xz_fft(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("derivatives_XZ_fft")
    }
    fn displace3d_fft(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displace3d_fft")
    }
    fn displace_fft(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displace_fft")
    }
    fn displace_grid(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displace_grid")
    }
    fn displace_lowres_grid(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displace_lowres_grid")
    }
    fn displace_points_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displace_points_2d")
    }
    fn displace_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displace_volume")
    }
    fn displaced_height_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("displaced_height_2d")
    }
    fn downsamped_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("downsamped_volume")
    }
    fn downsample(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("downsample")
    }
    fn exp_falloff_disp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("exp_falloff_disp")
    }
    fn exp_falloff_vel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("exp_falloff_vel")
    }
    fn find_max_displace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("find_max_displace")
    }
    fn foreach_begin1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin1")
    }
    fn foreach_begin10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin10")
    }
    fn foreach_begin12(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin12")
    }
    fn foreach_begin13(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin13")
    }
    fn foreach_begin14(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin14")
    }
    fn foreach_begin16(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin16")
    }
    fn foreach_begin3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin3")
    }
    fn foreach_begin4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin4")
    }
    fn foreach_begin5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin5")
    }
    fn foreach_begin6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin6")
    }
    fn foreach_begin7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin7")
    }
    fn foreach_begin8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin8")
    }
    fn foreach_begin9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_begin9")
    }
    fn foreach_end1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end1")
    }
    fn foreach_end10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end10")
    }
    fn foreach_end12(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end12")
    }
    fn foreach_end13(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end13")
    }
    fn foreach_end15(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end15")
    }
    fn foreach_end3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end3")
    }
    fn foreach_end4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end4")
    }
    fn foreach_end5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end5")
    }
    fn foreach_end6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end6")
    }
    fn foreach_end7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end7")
    }
    fn foreach_end8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end8")
    }
    fn foreach_end9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_end9")
    }
    fn foreach_input(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_input")
    }
    fn foreach_input1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_input1")
    }
    fn foreach_input2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_input2")
    }
    fn foreach_input3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_input3")
    }
    fn foreach_input4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_input4")
    }
    fn foreach_input5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("foreach_input5")
    }
    fn generate_points(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("generate_points")
    }
    fn generate_points1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("generate_points1")
    }
    fn group_inside(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("group_inside")
    }
    fn group_inside1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("group_inside1")
    }
    fn heightfield1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("heightfield1")
    }
    fn hide_mask_primitives1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("hide_mask_primitives1")
    }
    fn hide_mask_volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("hide_mask_volumes")
    }
    fn index_attributes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index_attributes")
    }
    fn input_box(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("input_box")
    }
    fn input_box_vel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("input_box_vel")
    }
    fn intersect_grid1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("intersect_grid1")
    }
    fn keep_amplitude_1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_amplitude")
    }
    fn keep_masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_masks")
    }
    fn keep_pointmasks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_pointmasks")
    }
    fn keep_restcusp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_restcusp")
    }
    fn keep_restderivs(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_restderivs")
    }
    fn keep_restdisplace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_restdisplace")
    }
    fn keep_restdisplace_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_restdisplace_3d")
    }
    fn keep_restvel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_restvel")
    }
    fn keep_restvel_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_restvel_3d")
    }
    fn keep_spectrum(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("keep_spectrum")
    }
    fn lowres_ptgrid(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("lowres_ptgrid")
    }
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merge1")
    }
    fn merge6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merge6")
    }
    fn merge7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merge7")
    }
    fn merge_masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merge_masks")
    }
    fn merged_displace_and_masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merged_displace_and_masks")
    }
    fn merged_displace_v_and_masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merged_displace_v_and_masks")
    }
    fn merged_v_and_masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merged_v_and_masks")
    }
    fn merged_volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merged_volumes")
    }
    fn name_calculation_volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_calculation_volumes_")
    }
    fn name_downsampled_volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_downsampled_volumes")
    }
    fn name_output_volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_output_volumes")
    }
    fn name_restcusp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_restcusp")
    }
    fn name_restderivs(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_restderivs")
    }
    fn name_restdisplace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_restdisplace")
    }
    fn name_restdisplace_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_restdisplace_3d")
    }
    fn name_restvel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_restvel")
    }
    fn name_restvel_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("name_restvel_3d")
    }
    fn null1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null1")
    }
    fn null10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null10")
    }
    fn null13(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null13")
    }
    fn null14(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null14")
    }
    fn null2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null2")
    }
    fn null3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null3")
    }
    fn null5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("null5")
    }
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("output0")
    }
    fn pointjitter1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pointjitter1")
    }
    fn pointjitter2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pointjitter2")
    }
    fn pressure_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pressure_volume")
    }
    fn preview_grid(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("preview_grid")
    }
    fn promote_maxdisplace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("promote_maxdisplace")
    }
    fn promote_maxpscale(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("promote_maxpscale")
    }
    fn rename_inside_group(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("rename_inside_group")
    }
    fn reset_layer_0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("reset_layer_0")
    }
    fn ropnet1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ropnet1")
    }
    fn sample_attributes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sample_attributes")
    }
    fn sample_restvelocity_and_displace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sample_restvelocity_and_displace")
    }
    fn sample_restvelocity_and_displace1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sample_restvelocity_and_displace1")
    }
    fn surface_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surface_volume")
    }
    fn switch_add_velocity2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_add_velocity2")
    }
    fn switch_add_velocity3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_add_velocity3")
    }
    fn switch_create_uvs(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_create_uvs")
    }
    fn switch_cusp2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_cusp2")
    }
    fn switch_cusp3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_cusp3")
    }
    fn switch_deform_geometry2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_deform_geometry2")
    }
    fn switch_displace_2d_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_displace_2d_3d")
    }
    fn switch_do_preview(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_do_preview")
    }
    fn switch_doheighfield(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_doheighfield")
    }
    fn switch_dopressure(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_dopressure")
    }
    fn switch_dosurface(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_dosurface")
    }
    fn switch_dovel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_dovel")
    }
    fn switch_downsample(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_downsample")
    }
    fn switch_estimate_at_frame(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_estimate_at_frame")
    }
    fn switch_exponential1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_exponential1")
    }
    fn switch_has_geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_has_geo")
    }
    fn switch_has_pointmasks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_has_pointmasks")
    }
    fn switch_has_points(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_has_points")
    }
    fn switch_height_method(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_height_method")
    }
    fn switch_output_group(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_output_group")
    }
    fn switch_output_mask(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_output_mask")
    }
    fn switch_output_restcusp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_output_restcusp")
    }
    fn switch_output_restderivs(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_output_restderivs")
    }
    fn switch_output_restdisplace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_output_restdisplace")
    }
    fn switch_output_restvel(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_output_restvel")
    }
    fn switch_per_wavenumber(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_per_wavenumber")
    }
    fn switch_per_wavenumber1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_per_wavenumber1")
    }
    fn switch_ptfromvol(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_ptfromvol")
    }
    fn switch_sample(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_sample")
    }
    fn switch_single_layer(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_single_layer")
    }
    fn switch_solo_layer(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_solo_layer")
    }
    fn switch_vel_2d_3d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_vel_2d_3d")
    }
    fn switch_vizcusp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_vizcusp")
    }
    fn switch_vizcusp1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_vizcusp1")
    }
    fn timeshift_to_frame(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("timeshift_to_frame")
    }
    fn uvproject2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("uvproject2")
    }
    fn uvproject_single(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("uvproject_single")
    }
    fn vel3d_fft(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("vel3d_fft")
    }
    fn vel_fft(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("vel_fft")
    }
    fn vel_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("vel_volume")
    }
    fn viz_cusp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("viz_cusp")
    }
    fn viz_cusp1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("viz_cusp1")
    }
    fn volume_from_displace_attrib(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("volume_from_displace_attrib")
    }
    fn volume_maxdisplace(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("volume_maxdisplace")
    }
    fn volumefromattrib1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("volumefromattrib1")
    }
    fn volumemix1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("volumemix1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanfoamMode {
    Emitter = 0,
    Solver = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanfoamRegion {
    Grid = 0,
    Camera = 1,
}

#[derive(Debug, Clone)]
pub struct SopOceanfoam {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOceanfoam {
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

    /// Connects to input 0: "Ocean Spectrum"
    pub fn set_input_ocean_spectrum<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Ocean Spectrum" and specifies the output index of the target node.
    pub fn set_input_ocean_spectrum_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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

    // --- Float parameters ---
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
    pub fn with_zmin(mut self, val: f32) -> Self {
        self.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmax(mut self, val: f32) -> Self {
        self.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zscale(mut self, val: f32) -> Self {
        self.params.insert(
            "zscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mincusp(mut self, val: f32) -> Self {
        self.params.insert(
            "mincusp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mincusp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mincusp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minvel(mut self, val: f32) -> Self {
        self.params.insert(
            "minvel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minheight(mut self, val: f32) -> Self {
        self.params.insert(
            "minheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minheight".to_string(),
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
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_life(mut self, val: f32) -> Self {
        self.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_life_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lifevar(mut self, val: f32) -> Self {
        self.params.insert(
            "lifevar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifevar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lifevar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_driftrate(mut self, val: f32) -> Self {
        self.params.insert(
            "driftrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_driftrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "driftrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minfoamdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "minfoamdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minfoamdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minfoamdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxfoamdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "maxfoamdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxfoamdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxfoamdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foampreserverate(mut self, val: f32) -> Self {
        self.params.insert(
            "foampreserverate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foampreserverate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foampreserverate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foamreducerate(mut self, val: f32) -> Self {
        self.params.insert(
            "foamreducerate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foamreducerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foamreducerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float2(val),
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
    pub fn with_winx(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "winx".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_winx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_winy(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "winy".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_winy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_downsample(mut self, val: i32) -> Self {
        self.params.insert(
            "downsample".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_downsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "downsample".to_string(),
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
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: SopOceanfoamMode) -> Self {
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
    pub fn with_region(mut self, val: SopOceanfoamRegion) -> Self {
        self.params.insert(
            "region".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_region_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "region".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptkeep(mut self, val: &str) -> Self {
        self.params.insert(
            "ptkeep".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptkeep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptkeep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_domincusp(mut self, val: bool) -> Self {
        self.params.insert(
            "domincusp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domincusp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domincusp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dominvel(mut self, val: bool) -> Self {
        self.params.insert(
            "dominvel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dominvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dominvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dominheight(mut self, val: bool) -> Self {
        self.params.insert(
            "dominheight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dominheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dominheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "dodisplace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preservefoam(mut self, val: bool) -> Self {
        self.params.insert(
            "preservefoam".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservefoam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preservefoam".to_string(),
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

impl crate::core::types::HoudiniNode for SopOceanfoam {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "oceanfoam"
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
pub enum SopOceansourceInitialize {
    WaveTank = 0,
    GuidedOceanLayer = 1,
    FlatTank = 2,
    BeachTank = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceansourceSurftype {
    Ocean = 0,
    Flat = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceansourceDepthfalloff {
    None = 0,
    Exponential = 1,
    ExponentialByFrequency = 2,
}

#[derive(Debug, Clone)]
pub struct SopOceansource {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOceansource {
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

    /// Connects to input 0: "Ocean Spectrum"
    pub fn set_input_ocean_spectrum<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Ocean Spectrum" and specifies the output index of the target node.
    pub fn set_input_ocean_spectrum_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Collision Geometry"
    pub fn set_input_collision_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Collision Geometry" and specifies the output index of the target node.
    pub fn set_input_collision_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversampling(mut self, val: f32) -> Self {
        self.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversampling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridscale(mut self, val: f32) -> Self {
        self.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gridscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waterlevel(mut self, val: f32) -> Self {
        self.params.insert(
            "waterlevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterlevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_layersize(mut self, val: f32) -> Self {
        self.params.insert(
            "layersize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_layersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidingthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "guidingthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidingthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidingthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterseed(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_surfacedepth(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacedepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacedepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacedepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volsmooth(mut self, val: f32) -> Self {
        self.params.insert(
            "volsmooth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volsmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volsmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxextrapolate(mut self, val: f32) -> Self {
        self.params.insert(
            "maxextrapolate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxextrapolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxextrapolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatter(mut self, val: f32) -> Self {
        self.params.insert(
            "scatter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplingbandwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversamplingbandwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_lowerpadding(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lowerpadding".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lowerpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lowerpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upperpadding(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperpadding".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_downsample(mut self, val: i32) -> Self {
        self.params.insert(
            "downsample".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_downsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "downsample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_depthdivs(mut self, val: i32) -> Self {
        self.params.insert(
            "depthdivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthdivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdisplaceframe(mut self, val: i32) -> Self {
        self.params.insert(
            "maxdisplaceframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxdisplaceframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdisplaceframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_initialize(mut self, val: SopOceansourceInitialize) -> Self {
        self.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surftype(mut self, val: SopOceansourceSurftype) -> Self {
        self.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthfalloff(mut self, val: SopOceansourceDepthfalloff) -> Self {
        self.params.insert(
            "depthfalloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depthfalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthfalloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dooversampling(mut self, val: bool) -> Self {
        self.params.insert(
            "dooversampling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooversampling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dooversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particles(mut self, val: bool) -> Self {
        self.params.insert(
            "particles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_particles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fillvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "fillvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fillvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fillvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_narrowband(mut self, val: bool) -> Self {
        self.params.insert(
            "narrowband".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_narrowband_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "narrowband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partcol(mut self, val: bool) -> Self {
        self.params.insert(
            "partcol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_partcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: bool) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxextrapolate(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxextrapolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxextrapolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxextrapolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary(mut self, val: bool) -> Self {
        self.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boundary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxres(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doscatter(mut self, val: bool) -> Self {
        self.params.insert(
            "doscatter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doscatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doscatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_estimatemaxdisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "estimatemaxdisplace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_estimatemaxdisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "estimatemaxdisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOceansource {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "oceansource"
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
pub enum SopOceanspectrumSpectrumtype {
    Phillips = 0,
    Tma = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanspectrumDistribution {
    None = 0,
    Uniform = 1,
    Gaussian = 2,
    LogNormal = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanspectrumPointconfig {
    AutoDetect = 0,
    /// 2D Best Fit Plane
    N2dBestFitPlane = 1,
    /// 3D
    N3d = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanspectrumMasktype {
    Suppression = 0,
    Contribution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanspectrumDoviz {
    None = 0,
    Spectrum = 1,
    WaveInstancing = 2,
    Mask = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanspectrumVismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    Blackbody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone)]
pub struct SopOceanspectrum {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOceanspectrum {
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

    /// Connects to input 0: "Wave Instancing Points"
    pub fn set_input_wave_instancing_points<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Wave Instancing Points" and specifies the output index of the target node.
    pub fn set_input_wave_instancing_points_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_gridsize(mut self, val: f32) -> Self {
        self.params.insert(
            "gridsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gridsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gridsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_gravity(mut self, val: f32) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gravity".to_string(),
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
    pub fn with_loopperiod(mut self, val: f32) -> Self {
        self.params.insert(
            "loopperiod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loopperiod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopperiod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_winddir(mut self, val: f32) -> Self {
        self.params.insert(
            "winddir".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_winddir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winddir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "windspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirbias(mut self, val: f32) -> Self {
        self.params.insert(
            "dirbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dirbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirmove(mut self, val: f32) -> Self {
        self.params.insert(
            "dirmove".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dirmove_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirmove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swell(mut self, val: f32) -> Self {
        self.params.insert(
            "swell".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_swell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fetch(mut self, val: f32) -> Self {
        self.params.insert(
            "fetch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fetch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fetch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_chopscale(mut self, val: f32) -> Self {
        self.params.insert(
            "chopscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_chopscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chopscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_referencewind(mut self, val: f32) -> Self {
        self.params.insert(
            "referencewind".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_referencewind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "referencewind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ampscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ampscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ampscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ampscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mink(mut self, val: f32) -> Self {
        self.params.insert(
            "mink".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrad(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointradvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointradvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointradvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointradvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrot(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrot".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrotvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrotvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrotvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrotvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointamp(mut self, val: f32) -> Self {
        self.params.insert(
            "pointamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointampvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointampvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointampvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointampvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "pointoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointoffsetvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointoffsetvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointoffsetvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointoffsetvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointwavelen(mut self, val: f32) -> Self {
        self.params.insert(
            "pointwavelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointwavelen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointwavelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointwavelenvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointwavelenvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointwavelenvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointwavelenvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointwavechop(mut self, val: f32) -> Self {
        self.params.insert(
            "pointwavechop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointwavechop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointwavechop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointseed(mut self, val: f32) -> Self {
        self.params.insert(
            "pointseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiserough(mut self, val: f32) -> Self {
        self.params.insert(
            "noiserough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noiserough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiserough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisedir(mut self, val: f32) -> Self {
        self.params.insert(
            "noisedir".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noisedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisespeed(mut self, val: f32) -> Self {
        self.params.insert(
            "noisespeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noisespeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisespeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisepulse(mut self, val: f32) -> Self {
        self.params.insert(
            "noisepulse".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noisepulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisepulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_gridcenter(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "gridcenter".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_gridcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gridcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisesize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "noisesize".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_noisesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseinput(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "noiseinput".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_noiseinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseoutput(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "noiseoutput".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_noiseoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "visrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_visrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_noiseoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "noiseoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_noiseoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_res(mut self, val: i32) -> Self {
        self.params
            .insert("res".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params
            .insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filteraboveres(mut self, val: i32) -> Self {
        self.params.insert(
            "filteraboveres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_filteraboveres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filteraboveres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbelowres(mut self, val: i32) -> Self {
        self.params.insert(
            "filterbelowres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_filterbelowres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbelowres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxmapres(mut self, val: i32) -> Self {
        self.params.insert(
            "maxmapres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxmapres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxmapres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseturb(mut self, val: i32) -> Self {
        self.params.insert(
            "noiseturb".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_noiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseturb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visres(mut self, val: i32) -> Self {
        self.params.insert(
            "visres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_visres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_spectrumtype(mut self, val: SopOceanspectrumSpectrumtype) -> Self {
        self.params.insert(
            "spectrumtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_spectrumtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spectrumtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distribution(mut self, val: SopOceanspectrumDistribution) -> Self {
        self.params.insert(
            "distribution".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distribution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distribution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointconfig(mut self, val: SopOceanspectrumPointconfig) -> Self {
        self.params.insert(
            "pointconfig".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pointconfig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointconfig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_masktype(mut self, val: SopOceanspectrumMasktype) -> Self {
        self.params.insert(
            "masktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_masktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doviz(mut self, val: SopOceanspectrumDoviz) -> Self {
        self.params.insert(
            "doviz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_doviz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doviz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vismode(mut self, val: SopOceanspectrumVismode) -> Self {
        self.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_amplituderamp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "amplituderamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_amplituderamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amplituderamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_noiseblendspectrum(mut self, val: &str) -> Self {
        self.params.insert(
            "noiseblendspectrum".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_noiseblendspectrum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseblendspectrum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_loop(mut self, val: bool) -> Self {
        self.params.insert(
            "loop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalize(mut self, val: bool) -> Self {
        self.params.insert(
            "normalize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterabove(mut self, val: bool) -> Self {
        self.params.insert(
            "filterabove".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterabove_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterabove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbelow(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbelow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbelow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbelow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapamplitude(mut self, val: bool) -> Self {
        self.params.insert(
            "remapamplitude".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapamplitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapamplitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopointrad(mut self, val: bool) -> Self {
        self.params.insert(
            "dopointrad".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopointrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopointrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopointrot(mut self, val: bool) -> Self {
        self.params.insert(
            "dopointrot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopointrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopointrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopointamp(mut self, val: bool) -> Self {
        self.params.insert(
            "dopointamp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopointamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopointamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopointoffset(mut self, val: bool) -> Self {
        self.params.insert(
            "dopointoffset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopointoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopointoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopointwavelen(mut self, val: bool) -> Self {
        self.params.insert(
            "dopointwavelen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopointwavelen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopointwavelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "noiseenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_noiseenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseblend(mut self, val: bool) -> Self {
        self.params.insert(
            "noiseblend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_noiseblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOceanspectrum {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "oceanspectrum"
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
pub enum SopOceanwavesAnimtype {
    Center = 0,
    WaveSpeed = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOceanwavesVelapproximation {
    None = 0,
    BackwardDifference = 1,
    CentralDifference = 2,
    ForwardDifference = 3,
}

#[derive(Debug, Clone)]
pub struct SopOceanwaves {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOceanwaves {
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

    /// Connects to input 0: "Wave Instancing Points"
    pub fn set_input_wave_instancing_points<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Wave Instancing Points" and specifies the output index of the target node.
    pub fn set_input_wave_instancing_points_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_wavesize(mut self, val: f32) -> Self {
        self.params.insert(
            "wavesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wavesep(mut self, val: f32) -> Self {
        self.params.insert(
            "wavesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavesep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wavedir(mut self, val: f32) -> Self {
        self.params.insert(
            "wavedir".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterseed(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_wavespeed(mut self, val: f32) -> Self {
        self.params.insert(
            "wavespeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavespeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavespeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waveoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "waveoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waveoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waveoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crestwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "crestwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_crestwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crestwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointwavechop(mut self, val: f32) -> Self {
        self.params.insert(
            "pointwavechop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointwavechop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointwavechop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrad(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointradvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointradvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointradvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointradvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrot(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrot".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointrotvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointrotvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointrotvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointrotvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointamp(mut self, val: f32) -> Self {
        self.params.insert(
            "pointamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointampvar(mut self, val: f32) -> Self {
        self.params.insert(
            "pointampvar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointampvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointampvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointseed(mut self, val: f32) -> Self {
        self.params.insert(
            "pointseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointseed".to_string(),
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

    // --- Int parameters ---
    pub fn with_nwaves(mut self, val: i32) -> Self {
        self.params.insert(
            "nwaves".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_nwaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nwaves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_npts(mut self, val: i32) -> Self {
        self.params
            .insert("npts".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_npts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "npts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_animtype(mut self, val: SopOceanwavesAnimtype) -> Self {
        self.params.insert(
            "animtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_animtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velapproximation(mut self, val: SopOceanwavesVelapproximation) -> Self {
        self.params.insert(
            "velapproximation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velapproximation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velapproximation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dowaves(mut self, val: bool) -> Self {
        self.params.insert(
            "dowaves".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dowaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dowaves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOceanwaves {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "oceanwaves"
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
pub enum SopOnnxInputType {
    Volume = 0,
    DetailAttribute = 1,
    PointAttribute = 2,
    PrimitiveAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOnnxInputVolorder {
    Zyx = 0,
    Xyz = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOnnxOutputType {
    Volume = 0,
    DetailAttribute = 1,
    PointAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOnnxOutputVolorder {
    Zyx = 0,
    Xyz = 1,
}

#[derive(Debug, Clone)]
pub struct SopOnnx {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOnnx {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    // --- IntArray parameters ---
    pub fn with_input_shape_inst(mut self, index1: usize, val: Vec<i32>) -> Self {
        self.params.insert(
            format!("input_shape{}", index1),
            crate::core::types::ParamValue::IntArray(val),
        );
        self
    }
    pub fn with_input_shape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_shape{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_shape_inst(mut self, index1: usize, val: Vec<i32>) -> Self {
        self.params.insert(
            format!("output_shape{}", index1),
            crate::core::types::ParamValue::IntArray(val),
        );
        self
    }
    pub fn with_output_shape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_shape{}", index1),
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
    pub fn with_input_type_inst(mut self, index1: usize, val: SopOnnxInputType) -> Self {
        self.params.insert(
            format!("input_type{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_input_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_volorder_inst(mut self, index1: usize, val: SopOnnxInputVolorder) -> Self {
        self.params.insert(
            format!("input_volorder{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_input_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_volorder{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_type_inst(mut self, index1: usize, val: SopOnnxOutputType) -> Self {
        self.params.insert(
            format!("output_type{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_volorder_inst(mut self, index1: usize, val: SopOnnxOutputVolorder) -> Self {
        self.params.insert(
            format!("output_volorder{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_volorder{}", index1),
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
    pub fn with_input_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("input_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_input_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("input_data{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_input_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_data{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("output_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("output_data{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_data{}", index1),
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
    pub fn with_keepinput(mut self, val: bool) -> Self {
        self.params.insert(
            "keepinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input_channelfirst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("input_channelfirst{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_input_channelfirst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("input_channelfirst{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_channelfirst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("output_channelfirst{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_output_channelfirst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_channelfirst{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOnnx {
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
pub enum SopOpenclRunover {
    FirstWriteableAttribute = 0,
    FirstWriteableVolume = 1,
    DetailAttributeOfWorksets = 2,
    FirstWriteableVdb = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclTimemethod {
    Timestep = 0,
    /// e ^ Timestep
    ETimestep = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclPrecision {
    Auto = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclBindingsType {
    Integer = 0,
    Float = 1,
    FloatVec3 = 2,
    FloatVec4 = 3,
    Ramp = 4,
    Attribute = 5,
    Volume = 6,
    Vdb = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclBindingsVdbtype {
    Any = 0,
    Float = 1,
    Vector = 2,
    Integer = 3,
    /// Float?
    Float1 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclBindingsAttribclass {
    Detail = 0,
    Primitive = 1,
    Point = 2,
    Vertex = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclBindingsAttribtype {
    Float = 0,
    Integer = 1,
    FloatArray = 2,
    IntegerArray = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclBindingsPrecision {
    Node = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOpenclBindingsTimescale {
    None = 0,
    Timestep = 1,
    /// 1 / Timestep
    N1Timestep = 2,
    /// e ^ Timestep
    ETimestep = 3,
}

#[derive(Debug, Clone)]
pub struct SopOpencl {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOpencl {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Input 3"
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Input 3" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Input 4"
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Input 4" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_generatekernel(mut self) -> Self {
        self.params.insert(
            "generatekernel".to_string(),
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
    pub fn with_bindings_input_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("bindings{}_input", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bindings_input_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindings{}_input", index1),
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

    // --- Menu parameters ---
    pub fn with_runover(mut self, val: SopOpenclRunover) -> Self {
        self.params.insert(
            "runover".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_runover_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "runover".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timemethod(mut self, val: SopOpenclTimemethod) -> Self {
        self.params.insert(
            "timemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_timemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision(mut self, val: SopOpenclPrecision) -> Self {
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
    pub fn with_bindings_type_inst(mut self, index1: usize, val: SopOpenclBindingsType) -> Self {
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
    pub fn with_bindings_vdbtype_inst(
        mut self,
        index1: usize,
        val: SopOpenclBindingsVdbtype,
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
        val: SopOpenclBindingsAttribclass,
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
        val: SopOpenclBindingsAttribtype,
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
        val: SopOpenclBindingsPrecision,
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
        val: SopOpenclBindingsTimescale,
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

    // --- String parameters ---
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
    pub fn with_kernelfile(mut self, val: &str) -> Self {
        self.params.insert(
            "kernelfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernelfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernelfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_kerneloptionattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "kerneloptionattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kerneloptionattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kerneloptionattrib".to_string(),
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
    pub fn with_worksets_begin(mut self, val: &str) -> Self {
        self.params.insert(
            "worksets_begin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_worksets_begin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worksets_begin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worksets_length(mut self, val: &str) -> Self {
        self.params.insert(
            "worksets_length".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_worksets_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worksets_length".to_string(),
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
    pub fn with_usecode(mut self, val: bool) -> Self {
        self.params.insert(
            "usecode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atbinding(mut self, val: bool) -> Self {
        self.params.insert(
            "atbinding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_atbinding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atbinding".to_string(),
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
    pub fn with_recompile(mut self, val: bool) -> Self {
        self.params.insert(
            "recompile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_recompile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recompile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iteration(mut self, val: bool) -> Self {
        self.params.insert(
            "iteration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iteration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iteration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singleworkgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "singleworkgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleworkgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleworkgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finish(mut self, val: bool) -> Self {
        self.params.insert(
            "finish".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finish_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "finish".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_time(mut self, val: bool) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeinc(mut self, val: bool) -> Self {
        self.params.insert(
            "timeinc".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeinc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeinc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xnoise(mut self, val: bool) -> Self {
        self.params.insert(
            "xnoise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xnoise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importprequel(mut self, val: bool) -> Self {
        self.params.insert(
            "importprequel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importprequel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importprequel".to_string(),
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

impl crate::core::types::HoudiniNode for SopOpencl {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveTangenttype {
    AverageOfEdgeDirections = 0,
    CentralDifference = 1,
    PreviousEdge = 2,
    NextEdge = 3,
    /// Z Axis (Ignore Curve)
    ZAxisIgnoreCurve = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveUpvectortype {
    CurveNormal = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    Attribute = 4,
    Custom = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveEnablecurvaturescaleattrib {
    NoScaling = 0,
    ScaleByAttrib = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveRord {
    /// Pitch, Yaw, Roll
    PitchYawRoll = 0,
    /// Pitch, Roll, Yaw
    PitchRollYaw = 1,
    /// Yaw, Pitch, Roll
    YawPitchRoll = 2,
    /// Yaw, Roll, Pitch
    YawRollPitch = 3,
    /// Roll, Pitch, Yaw
    RollPitchYaw = 4,
    /// Roll, Yaw, Pitch
    RollYawPitch = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveRollper {
    PerEdge = 0,
    PerUnitDistance = 1,
    ScaleByAttribute = 2,
    PerFullCurveByEdges = 3,
    PerFullCurveByDistance = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveYawper {
    PerEdge = 0,
    PerUnitDistance = 1,
    ScaleByAttribute = 2,
    PerFullCurveByEdges = 3,
    PerFullCurveByDistance = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurvePitchper {
    PerEdge = 0,
    PerUnitDistance = 1,
    ScaleByAttribute = 2,
    PerFullCurveByEdges = 3,
    PerFullCurveByDistance = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOrientalongcurveClass {
    Point = 0,
    Vertex = 1,
}

#[derive(Debug, Clone)]
pub struct SopOrientalongcurve {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOrientalongcurve {
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

    /// Connects to input 0: "Curves"
    pub fn set_input_curves<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Curves" and specifies the output index of the target node.
    pub fn set_input_curves_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Banking Curves"
    pub fn set_input_banking_curves<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Banking Curves" and specifies the output index of the target node.
    pub fn set_input_banking_curves_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_curvaturescale(mut self, val: f32) -> Self {
        self.params.insert(
            "curvaturescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturescale".to_string(),
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
    pub fn with_incroll(mut self, val: f32) -> Self {
        self.params.insert(
            "incroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_incroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "incroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaw(mut self, val: f32) -> Self {
        self.params.insert(
            "yaw".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yaw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yaw".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_incyaw(mut self, val: f32) -> Self {
        self.params.insert(
            "incyaw".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_incyaw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "incyaw".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pitch(mut self, val: f32) -> Self {
        self.params.insert(
            "pitch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_incpitch(mut self, val: f32) -> Self {
        self.params.insert(
            "incpitch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_incpitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "incpitch".to_string(),
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
    pub fn with_maxstretcharoundturns(mut self, val: f32) -> Self {
        self.params.insert(
            "maxstretcharoundturns".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxstretcharoundturns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxstretcharoundturns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_endupvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "endupvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_endupvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endupvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_fulltwists(mut self, val: i32) -> Self {
        self.params.insert(
            "fulltwists".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fulltwists_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fulltwists".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_tangenttype(mut self, val: SopOrientalongcurveTangenttype) -> Self {
        self.params.insert(
            "tangenttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tangenttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangenttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectortype(mut self, val: SopOrientalongcurveUpvectortype) -> Self {
        self.params.insert(
            "upvectortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_upvectortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecurvaturescaleattrib(
        mut self,
        val: SopOrientalongcurveEnablecurvaturescaleattrib,
    ) -> Self {
        self.params.insert(
            "enablecurvaturescaleattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enablecurvaturescaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecurvaturescaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: SopOrientalongcurveRord) -> Self {
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
    pub fn with_rollper(mut self, val: SopOrientalongcurveRollper) -> Self {
        self.params.insert(
            "rollper".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rollper_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollper".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yawper(mut self, val: SopOrientalongcurveYawper) -> Self {
        self.params.insert(
            "yawper".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_yawper_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yawper".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pitchper(mut self, val: SopOrientalongcurvePitchper) -> Self {
        self.params.insert(
            "pitchper".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pitchper_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pitchper".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_class(mut self, val: SopOrientalongcurveClass) -> Self {
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
    pub fn with_upvectorattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "upvectorattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upvectorattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endupvectorattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "endupvectorattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_endupvectorattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endupvectorattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvaturescaleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "curvaturescaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curvaturescaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturescaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvatureattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "curvatureattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curvatureattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvatureattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rollattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "rollattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rollattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yawattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "yawattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_yawattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yawattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pitchattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pitchattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pitchattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pitchattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxisname(mut self, val: &str) -> Self {
        self.params.insert(
            "xaxisname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xaxisname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xaxisname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxisname(mut self, val: &str) -> Self {
        self.params.insert(
            "yaxisname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_yaxisname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yaxisname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxisname(mut self, val: &str) -> Self {
        self.params.insert(
            "zaxisname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_zaxisname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zaxisname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translationname(mut self, val: &str) -> Self {
        self.params.insert(
            "translationname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_translationname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translationname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quaternionname(mut self, val: &str) -> Self {
        self.params.insert(
            "quaternionname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_quaternionname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quaternionname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transform3name(mut self, val: &str) -> Self {
        self.params.insert(
            "transform3name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transform3name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transform3name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transform4name(mut self, val: &str) -> Self {
        self.params.insert(
            "transform4name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transform4name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transform4name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_continuousclosed(mut self, val: bool) -> Self {
        self.params.insert(
            "continuousclosed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuousclosed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuousclosed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrapolateendtangents(mut self, val: bool) -> Self {
        self.params.insert(
            "extrapolateendtangents".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extrapolateendtangents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrapolateendtangents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transformbyattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "transformbyattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_transformbyattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transformbyattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectoratstart(mut self, val: bool) -> Self {
        self.params.insert(
            "upvectoratstart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_upvectoratstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectoratstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useendupvector(mut self, val: bool) -> Self {
        self.params.insert(
            "useendupvector".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useendupvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useendupvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjustupcurvature(mut self, val: bool) -> Self {
        self.params.insert(
            "adjustupcurvature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjustupcurvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adjustupcurvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecurvatureattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecurvatureattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecurvatureattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecurvatureattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applyroll(mut self, val: bool) -> Self {
        self.params.insert(
            "applyroll".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applyroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applyroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applyyaw(mut self, val: bool) -> Self {
        self.params.insert(
            "applyyaw".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applyyaw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applyyaw".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applypitch(mut self, val: bool) -> Self {
        self.params.insert(
            "applypitch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applypitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applypitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalize(mut self, val: bool) -> Self {
        self.params.insert(
            "normalize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretcharoundturns(mut self, val: bool) -> Self {
        self.params.insert(
            "stretcharoundturns".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stretcharoundturns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretcharoundturns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputxaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "outputxaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputxaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputxaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputyaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "outputyaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputyaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputyaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputzaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "outputzaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputzaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputzaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtranslation(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtranslation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtranslation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtranslation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputquaternion(mut self, val: bool) -> Self {
        self.params.insert(
            "outputquaternion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputquaternion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputquaternion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtransform3(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtransform3".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtransform3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtransform3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtransform4(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtransform4".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtransform4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtransform4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOrientalongcurve {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "orientalongcurve"
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
pub enum SopOtisconfiguremuscleandtissueMuscleendsMode {
    ConstrainToInputAnimation = 0,
    ConstrainToNearestBone = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOtisconfiguremuscleandtissueTissuerigidpointsMode {
    ConstrainToInputAnimation = 0,
    ConstrainToNearestBone = 1,
}

#[derive(Debug, Clone)]
pub struct SopOtisconfiguremuscleandtissue {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOtisconfiguremuscleandtissue {
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

    /// Connects to input 0: "Muscle Geometry"
    pub fn set_input_muscle_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Muscle Geometry" and specifies the output index of the target node.
    pub fn set_input_muscle_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Tissue Geometry"
    pub fn set_input_tissue_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Tissue Geometry" and specifies the output index of the target node.
    pub fn set_input_tissue_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Bone Geometry"
    pub fn set_input_bone_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Bone Geometry" and specifies the output index of the target node.
    pub fn set_input_bone_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_restblend(mut self, val: f32) -> Self {
        self.params.insert(
            "restblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleends_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "muscleends_threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscleends_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleends_threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tetquality_minquality(mut self, val: f32) -> Self {
        self.params.insert(
            "tetquality_minquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tetquality_minquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tetquality_minquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tetquality_volfraction(mut self, val: f32) -> Self {
        self.params.insert(
            "tetquality_volfraction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tetquality_volfraction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tetquality_volfraction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_vis_muscleendconstraintscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vis_muscleendconstraintscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vis_muscleendconstraintscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_muscleendconstraintscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_muscleglueconstraintscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vis_muscleglueconstraintscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vis_muscleglueconstraintscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_muscleglueconstraintscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_rigidpointscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vis_rigidpointscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vis_rigidpointscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_rigidpointscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_tissuetoboneconstraintscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vis_tissuetoboneconstraintscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vis_tissuetoboneconstraintscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_tissuetoboneconstraintscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_tissuetomuscleconstraintscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vis_tissuetomuscleconstraintscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vis_tissuetomuscleconstraintscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_tissuetomuscleconstraintscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_refframe(mut self, val: i32) -> Self {
        self.params.insert(
            "refframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_refframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_muscleends_mode(
        mut self,
        val: SopOtisconfiguremuscleandtissueMuscleendsMode,
    ) -> Self {
        self.params.insert(
            "muscleends_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_muscleends_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleends_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuerigidpoints_mode(
        mut self,
        val: SopOtisconfiguremuscleandtissueTissuerigidpointsMode,
    ) -> Self {
        self.params.insert(
            "tissuerigidpoints_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tissuerigidpoints_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissuerigidpoints_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_activation_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("activation{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_activation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("activation{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shape_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("shape{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("shape{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuerigidpoints_group(mut self, val: &str) -> Self {
        self.params.insert(
            "tissuerigidpoints_group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissuerigidpoints_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissuerigidpoints_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobones_bonegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "tissuetobones_bonegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissuetobones_bonegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissuetobones_bonegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscles_tposeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "muscles_tposeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_muscles_tposeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscles_tposeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissue_tposeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "tissue_tposeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissue_tposeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissue_tposeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tposeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "tposeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tposeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tposeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepptattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepptattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepptattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepptattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepvtxattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepvtxattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepvtxattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepvtxattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepprimattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepprimattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepprimattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepdetailattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepdetailattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepdetailattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepdetailattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_deformtorefbones(mut self, val: bool) -> Self {
        self.params.insert(
            "deformtorefbones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deformtorefbones_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deformtorefbones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscles_simulate(mut self, val: bool) -> Self {
        self.params.insert(
            "muscles_simulate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscles_simulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscles_simulate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleends_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "muscleends_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscleends_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleends_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleglue_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "muscleglue_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscleglue_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleglue_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetargetshapes(mut self, val: bool) -> Self {
        self.params.insert(
            "usetargetshapes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetshapes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetargetshapes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableshape_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enableshape{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableshape_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enableshape{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissue_fasciacollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "tissue_fasciacollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tissue_fasciacollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissue_fasciacollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuerigidpoints_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tissuerigidpoints_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tissuerigidpoints_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissuerigidpoints_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobones_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tissuetobones_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tissuetobones_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissuetobones_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscle_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tissuetomuscle_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tissuetomuscle_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tissuetomuscle_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tetquality_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tetquality_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tetquality_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tetquality_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alwaysshowvis(mut self, val: bool) -> Self {
        self.params.insert(
            "alwaysshowvis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alwaysshowvis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alwaysshowvis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_bonegeo(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_bonegeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_bonegeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_bonegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_musclegeo(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_musclegeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_musclegeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_musclegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_tissuegeo(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_tissuegeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_tissuegeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_tissuegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_muscleendconstraints(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_muscleendconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_muscleendconstraints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_muscleendconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_muscleglueconstraints(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_muscleglueconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_muscleglueconstraints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_muscleglueconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_rigidpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_rigidpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_rigidpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_rigidpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_tissuetoboneconstraints(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_tissuetoboneconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_tissuetoboneconstraints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_tissuetoboneconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_tissuetomuscleconstraints(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_tissuetomuscleconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_tissuetomuscleconstraints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_tissuetomuscleconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cleanupattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "cleanupattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cleanupattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cleanupattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cleanupgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "cleanupgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cleanupgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cleanupgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOtisconfiguremuscleandtissue {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "otisconfiguremuscleandtissue"
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
pub enum SopOtissolverSimulationtype {
    Quasistatic = 0,
    Dynamic = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOtissolverInterout {
    None = 0,
    EachSubstep = 1,
    EachTargetPose = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOtissolverTargetmethod {
    FirstInput = 0,
    SopPath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOtissolverTargetconstraintmethod {
    SecondInput = 0,
    SopPath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopOtissolverTargetfreq {
    EachFrame = 0,
    EachSubstep = 1,
}

#[derive(Debug, Clone)]
pub struct SopOtissolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOtissolver {
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

    /// Connects to input 0: "Vellum Geometry"
    pub fn set_input_vellum_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Vellum Geometry" and specifies the output index of the target node.
    pub fn set_input_vellum_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Constraint Geometry"
    pub fn set_input_constraint_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Constraint Geometry" and specifies the output index of the target node.
    pub fn set_input_constraint_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
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
    pub fn with_colfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "colfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kcol(mut self, val: f32) -> Self {
        self.params.insert(
            "kcol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veldamping(mut self, val: f32) -> Self {
        self.params.insert(
            "veldamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veldamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veldamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mus(mut self, val: f32) -> Self {
        self.params.insert(
            "mus".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muk(mut self, val: f32) -> Self {
        self.params.insert(
            "muk".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rho(mut self, val: f32) -> Self {
        self.params.insert(
            "rho".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rho_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rho".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_groundorig(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundorig".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundorig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundorig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grounddir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "grounddir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_grounddir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grounddir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gravity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
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
    pub fn with_iter(mut self, val: i32) -> Self {
        self.params
            .insert("iter".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampquasistaticposes(mut self, val: i32) -> Self {
        self.params.insert(
            "dampquasistaticposes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dampquasistaticposes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampquasistaticposes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_simulationtype(mut self, val: SopOtissolverSimulationtype) -> Self {
        self.params.insert(
            "simulationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_simulationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "simulationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interout(mut self, val: SopOtissolverInterout) -> Self {
        self.params.insert(
            "interout".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetmethod(mut self, val: SopOtissolverTargetmethod) -> Self {
        self.params.insert(
            "targetmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetconstraintmethod(mut self, val: SopOtissolverTargetconstraintmethod) -> Self {
        self.params.insert(
            "targetconstraintmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetconstraintmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetconstraintmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetfreq(mut self, val: SopOtissolverTargetfreq) -> Self {
        self.params.insert(
            "targetfreq".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_trigroup(mut self, val: &str) -> Self {
        self.params.insert(
            "trigroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_trigroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trigroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tetgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "tetgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tetgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tetgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interfilebase(mut self, val: &str) -> Self {
        self.params.insert(
            "interfilebase".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interfilebase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interfilebase".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetconstraintpath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetconstraintpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetconstraintpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetconstraintpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_selfcol(mut self, val: bool) -> Self {
        self.params.insert(
            "selfcol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_selfcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selfcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groundplane(mut self, val: bool) -> Self {
        self.params.insert(
            "groundplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_groundplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotets(mut self, val: bool) -> Self {
        self.params.insert(
            "dotets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dotets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignorepiece(mut self, val: bool) -> Self {
        self.params.insert(
            "ignorepiece".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignorepiece_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignorepiece".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kcoladaptive(mut self, val: bool) -> Self {
        self.params.insert(
            "kcoladaptive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_kcoladaptive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kcoladaptive".to_string(),
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
    pub fn with_autoresim(mut self, val: bool) -> Self {
        self.params.insert(
            "autoresim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoresim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoresim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivestiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "adaptivestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptivestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedoubleprecision(mut self, val: bool) -> Self {
        self.params.insert(
            "usedoubleprecision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedoubleprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedoubleprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepackedprims(mut self, val: bool) -> Self {
        self.params.insert(
            "usepackedprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepackedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepackedprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usequasistaticvel(mut self, val: bool) -> Self {
        self.params.insert(
            "usequasistaticvel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usequasistaticvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usequasistaticvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "dostiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorestshape(mut self, val: bool) -> Self {
        self.params.insert(
            "dorestshape".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorestshape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorestshape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dofiberscale(mut self, val: bool) -> Self {
        self.params.insert(
            "dofiberscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dofiberscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dofiberscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOtissolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "otissolver"
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
pub struct SopOutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopOutput {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_outputidx(mut self, val: i32) -> Self {
        self.params.insert(
            "outputidx".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outputidx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputidx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopOutput {
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
