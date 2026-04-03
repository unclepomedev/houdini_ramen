#[derive(Debug, Clone)]
pub struct ObjectMatnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMatnet {
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


}

impl crate::core::types::HoudiniNode for ObjectMatnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "matnet"
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
pub enum ObjectMcacclaimIkmode {
    FowardKinematics = 0,
    InverseKinematics = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMcacclaimBonelen {
    /// Don't Overrride Channels
    DonTOverrrideChannels = 0,
    AlwaysOverrrideChannels = 1,
    AutomaticallyOverrideChannels = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMcacclaimXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMcacclaimRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMcacclaimPreXform {
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
pub enum ObjectMcacclaimUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMcacclaim {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMcacclaim {
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
    pub fn trigger_load(mut self) -> Self {
        self.params.insert("load".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_clear(mut self) -> Self {
        self.params.insert("clear".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_load_amc(mut self) -> Self {
        self.params.insert("load_amc".to_string(), crate::core::types::ParamValue::Button);
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_framerate(mut self, val: i32) -> Self {
        self.params.insert("framerate".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_framerate_expr(mut self, expr: &str) -> Self {
        self.params.insert("framerate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_animation(mut self, val: i32) -> Self {
        self.params.insert("animation".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_animation_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ikmode(mut self, val: ObjectMcacclaimIkmode) -> Self {
        self.params.insert("ikmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_ikmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("ikmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bonelen(mut self, val: ObjectMcacclaimBonelen) -> Self {
        self.params.insert("bonelen".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bonelen_expr(mut self, expr: &str) -> Self {
        self.params.insert("bonelen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMcacclaimXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMcacclaimRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectMcacclaimPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMcacclaimUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_asf(mut self, val: &str) -> Self {
        self.params.insert("asf".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_asf_expr(mut self, expr: &str) -> Self {
        self.params.insert("asf".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ikchainslist(mut self, val: &str) -> Self {
        self.params.insert("ikchainslist".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ikchainslist_expr(mut self, expr: &str) -> Self {
        self.params.insert("ikchainslist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outdir(mut self, val: &str) -> Self {
        self.params.insert("outdir".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outdir_expr(mut self, expr: &str) -> Self {
        self.params.insert("outdir".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_motion_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("motion{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_motion_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("motion{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_cmdout(mut self, val: &str) -> Self {
        self.params.insert("cmdout".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cmdout_expr(mut self, expr: &str) -> Self {
        self.params.insert("cmdout".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_reload(mut self, val: bool) -> Self {
        self.params.insert("reload".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_reload_expr(mut self, expr: &str) -> Self {
        self.params.insert("reload".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_autoloadmotions(mut self, val: bool) -> Self {
        self.params.insert("autoloadmotions".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_autoloadmotions_expr(mut self, expr: &str) -> Self {
        self.params.insert("autoloadmotions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dualchains(mut self, val: bool) -> Self {
        self.params.insert("dualchains".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dualchains_expr(mut self, expr: &str) -> Self {
        self.params.insert("dualchains".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ikchains(mut self, val: bool) -> Self {
        self.params.insert("ikchains".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_ikchains_expr(mut self, expr: &str) -> Self {
        self.params.insert("ikchains".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keeptmp(mut self, val: bool) -> Self {
        self.params.insert("keeptmp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeptmp_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeptmp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMcacclaim {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mcacclaim"
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
pub enum ObjectMicrophoneXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMicrophoneRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMicrophonePreXform {
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
pub enum ObjectMicrophoneUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMicrophoneDropoff {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone)]
pub struct ObjectMicrophone {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMicrophone {
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

    /// Connects to input 0: "parent"
    pub fn set_input_parent<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sensitive(mut self, val: f32) -> Self {
        self.params.insert("sensitive".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sensitive_expr(mut self, expr: &str) -> Self {
        self.params.insert("sensitive".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reccone(mut self, val: f32) -> Self {
        self.params.insert("reccone".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_reccone_expr(mut self, expr: &str) -> Self {
        self.params.insert("reccone".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outcone(mut self, val: f32) -> Self {
        self.params.insert("outcone".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_outcone_expr(mut self, expr: &str) -> Self {
        self.params.insert("outcone".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_droprate(mut self, val: f32) -> Self {
        self.params.insert("droprate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_droprate_expr(mut self, expr: &str) -> Self {
        self.params.insert("droprate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outsensitive(mut self, val: f32) -> Self {
        self.params.insert("outsensitive".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_outsensitive_expr(mut self, expr: &str) -> Self {
        self.params.insert("outsensitive".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_micactive(mut self, val: i32) -> Self {
        self.params.insert("micactive".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_micactive_expr(mut self, expr: &str) -> Self {
        self.params.insert("micactive".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectMicrophoneXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMicrophoneRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectMicrophonePreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMicrophoneUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dropoff(mut self, val: ObjectMicrophoneDropoff) -> Self {
        self.params.insert("dropoff".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_dropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert("dropoff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert("shop_materialopts".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_materialopts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chop_filterpath(mut self, val: &str) -> Self {
        self.params.insert("chop_filterpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_chop_filterpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("chop_filterpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert("shop_materialpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_materialpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_direct(mut self, val: bool) -> Self {
        self.params.insert("direct".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_direct_expr(mut self, expr: &str) -> Self {
        self.params.insert("direct".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert("vport_shadeopen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_shadeopen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert("vport_displayassubdiv".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_displayassubdiv".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMicrophone {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "microphone"
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
pub trait ObjectMicrophoneInnerExt {
    fn add1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn add2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn carve1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn carve2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn grid1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sphere1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sphere2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn tube1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMicrophoneInnerExt for crate::core::graph::InnerGraph<'a> {
    fn add1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("add1")
    }
    fn add2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("add2")
    }
    fn carve1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("carve1")
    }
    fn carve2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("carve2")
    }
    fn grid1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("grid1")
    }
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("merge3")
    }
    fn sphere1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sphere1")
    }
    fn sphere2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sphere2")
    }
    fn tube1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("tube1")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedArmPreXform {
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
pub enum ObjectMocapRigBipedArmXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedArmRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedArmUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedArmSide {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapRigBipedArm {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapRigBipedArm {
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



    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_collarbone_bone_length(mut self, val: f32) -> Self {
        self.params.insert("collarbone_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_collarbone_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("collarbone_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_upperarm_bone_length(mut self, val: f32) -> Self {
        self.params.insert("upperarm_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_upperarm_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("upperarm_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_forearm_bone_length(mut self, val: f32) -> Self {
        self.params.insert("forearm_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_forearm_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("forearm_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_arm_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_arm_root_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_arm_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_arm_root_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_shoulder_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_shoulder_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_wrist_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_wrist_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_arm_twist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_arm_twist_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_arm_twist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_arm_twist_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapRigBipedArmPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapRigBipedArmXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapRigBipedArmRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapRigBipedArmUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_side(mut self, val: ObjectMocapRigBipedArmSide) -> Self {
        self.params.insert("side".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_side_expr(mut self, expr: &str) -> Self {
        self.params.insert("side".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation_rig_path(mut self, val: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_animation_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path(mut self, val: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_targetpath(mut self, val: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_root_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shoulder_targetpath(mut self, val: &str) -> Self {
        self.params.insert("shoulder_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shoulder_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shoulder_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_wrist_targetpath(mut self, val: &str) -> Self {
        self.params.insert("wrist_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_wrist_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("wrist_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_arm_twist_targetpath(mut self, val: &str) -> Self {
        self.params.insert("arm_twist_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_arm_twist_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("arm_twist_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapRigBipedArm {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocap_rig_biped_arm"
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
pub trait ObjectMocapRigBipedArmInnerExt {
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn collarbone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_arm_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_arm_twist(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_shoulder(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_wrist(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn forearm_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_arm_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_arm_twist(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_shoulder(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_wrist(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rotate_null(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn upperarm_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapRigBipedArmInnerExt for crate::core::graph::InnerGraph<'a> {
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chops")
    }
    fn collarbone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("collarbone")
    }
    fn ctrl_arm_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_arm_root")
    }
    fn ctrl_arm_twist(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_arm_twist")
    }
    fn ctrl_shoulder(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_shoulder")
    }
    fn ctrl_wrist(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_wrist")
    }
    fn forearm_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("forearm_bone")
    }
    fn ptr_ctrl_arm_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_arm_root")
    }
    fn ptr_ctrl_arm_twist(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_arm_twist")
    }
    fn ptr_ctrl_shoulder(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_shoulder")
    }
    fn ptr_ctrl_wrist(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_wrist")
    }
    fn rotate_null(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("rotate_null")
    }
    fn upperarm_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("upperarm_bone")
    }
    fn wrist_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_bone")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedHeadAndNeckPreXform {
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
pub enum ObjectMocapRigBipedHeadAndNeckXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedHeadAndNeckRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedHeadAndNeckUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapRigBipedHeadAndNeck {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapRigBipedHeadAndNeck {
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



    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_bone_length(mut self, val: f32) -> Self {
        self.params.insert("root_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_root_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_neck_bone_length(mut self, val: f32) -> Self {
        self.params.insert("neck_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_neck_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("neck_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_head_bone_length(mut self, val: f32) -> Self {
        self.params.insert("head_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_head_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("head_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_neck_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_neck_root_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_neck_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_neck_root_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_neck2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_neck2_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_neck2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_neck2_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_head_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_head_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_head_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_head_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_head_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_head_end_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_head_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_head_end_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapRigBipedHeadAndNeckPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapRigBipedHeadAndNeckXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapRigBipedHeadAndNeckRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapRigBipedHeadAndNeckUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation_rig_path(mut self, val: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_animation_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path(mut self, val: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_targetpath(mut self, val: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_root_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_neck2_targetpath(mut self, val: &str) -> Self {
        self.params.insert("neck2_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_neck2_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("neck2_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_head_targetpath(mut self, val: &str) -> Self {
        self.params.insert("head_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_head_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("head_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_head_end_targetpath(mut self, val: &str) -> Self {
        self.params.insert("head_end_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_head_end_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("head_end_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapRigBipedHeadAndNeck {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocap_rig_biped_head_and_neck"
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
pub trait ObjectMocapRigBipedHeadAndNeckInnerExt {
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_head(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_head_end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_neck2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_neck_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_offset(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn neck_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_head(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_head_end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_neck2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_neck_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn root_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapRigBipedHeadAndNeckInnerExt for crate::core::graph::InnerGraph<'a> {
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chops")
    }
    fn ctrl_head(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_head")
    }
    fn ctrl_head_end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_head_end")
    }
    fn ctrl_neck2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_neck2")
    }
    fn ctrl_neck_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_neck_root")
    }
    fn head_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("head_bone")
    }
    fn head_offset(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("head_offset")
    }
    fn neck_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("neck_bone")
    }
    fn ptr_ctrl_head(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_head")
    }
    fn ptr_ctrl_head_end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_head_end")
    }
    fn ptr_ctrl_neck2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_neck2")
    }
    fn ptr_ctrl_neck_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_neck_root")
    }
    fn root_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("root_bone")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedLegPreXform {
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
pub enum ObjectMocapRigBipedLegXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedLegRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedLegUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedLegSide {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapRigBipedLeg {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapRigBipedLeg {
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



    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pelvis_bone_length(mut self, val: f32) -> Self {
        self.params.insert("pelvis_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pelvis_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("pelvis_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thigh_bone_length(mut self, val: f32) -> Self {
        self.params.insert("thigh_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_thigh_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("thigh_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shin_bone_length(mut self, val: f32) -> Self {
        self.params.insert("shin_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shin_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("shin_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ankle_bone_length(mut self, val: f32) -> Self {
        self.params.insert("ankle_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ankle_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("ankle_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_toe_bone_length(mut self, val: f32) -> Self {
        self.params.insert("toe_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_toe_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("toe_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_leg_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_leg_root_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_leg_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_leg_root_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_hip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_hip_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_hip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_hip_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_ankle_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_ankle_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_ball_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_ball_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_toe_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_toe_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_leg_twist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_leg_twist_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_leg_twist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_leg_twist_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ankle_reference_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ankle_reference_r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ankle_reference_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("ankle_reference_r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapRigBipedLegPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapRigBipedLegXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapRigBipedLegRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapRigBipedLegUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_side(mut self, val: ObjectMocapRigBipedLegSide) -> Self {
        self.params.insert("side".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_side_expr(mut self, expr: &str) -> Self {
        self.params.insert("side".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation_rig_path(mut self, val: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_animation_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path(mut self, val: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_targetpath(mut self, val: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_root_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hip_targetpath(mut self, val: &str) -> Self {
        self.params.insert("hip_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_hip_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("hip_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ankle_targetpath(mut self, val: &str) -> Self {
        self.params.insert("ankle_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ankle_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("ankle_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ball_targetpath(mut self, val: &str) -> Self {
        self.params.insert("ball_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ball_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("ball_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_toe_targetpath(mut self, val: &str) -> Self {
        self.params.insert("toe_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_toe_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("toe_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_leg_twist_targetpath(mut self, val: &str) -> Self {
        self.params.insert("leg_twist_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_leg_twist_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("leg_twist_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapRigBipedLeg {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocap_rig_biped_leg"
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
pub trait ObjectMocapRigBipedLegInnerExt {
    fn ankle_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ankle_reference(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_ankle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_ball(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_hip(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_leg_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_leg_twist(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_toe(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pelvis_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_ankle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_ball(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_hip(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_leg_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_leg_twist(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_toe(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn shin_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thigh_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn toe_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapRigBipedLegInnerExt for crate::core::graph::InnerGraph<'a> {
    fn ankle_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ankle_bone")
    }
    fn ankle_reference(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ankle_reference")
    }
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chops")
    }
    fn ctrl_ankle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_ankle")
    }
    fn ctrl_ball(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_ball")
    }
    fn ctrl_hip(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_hip")
    }
    fn ctrl_leg_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_leg_root")
    }
    fn ctrl_leg_twist(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_leg_twist")
    }
    fn ctrl_toe(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_toe")
    }
    fn pelvis_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pelvis_bone")
    }
    fn ptr_ctrl_ankle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_ankle")
    }
    fn ptr_ctrl_ball(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_ball")
    }
    fn ptr_ctrl_hip(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_hip")
    }
    fn ptr_ctrl_leg_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_leg_root")
    }
    fn ptr_ctrl_leg_twist(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_leg_twist")
    }
    fn ptr_ctrl_toe(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_toe")
    }
    fn shin_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("shin_bone")
    }
    fn thigh_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thigh_bone")
    }
    fn toe_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("toe_bone")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedSpine3pcPreXform {
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
pub enum ObjectMocapRigBipedSpine3pcXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedSpine3pcRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedSpine3pcUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapRigBipedSpine3pc {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapRigBipedSpine3pc {
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



    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_bone_length(mut self, val: f32) -> Self {
        self.params.insert("root_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_root_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back1_bone_length(mut self, val: f32) -> Self {
        self.params.insert("lower_back1_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lower_back1_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back1_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back2_bone_length(mut self, val: f32) -> Self {
        self.params.insert("lower_back2_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lower_back2_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back2_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chest_bone_length(mut self, val: f32) -> Self {
        self.params.insert("chest_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_chest_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("chest_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_spine_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_spine_root_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_spine_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_spine_root_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_lower_back2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_lower_back2_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_lower_back2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_lower_back2_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_chest_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_chest_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_chest_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_chest_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_neck_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_neck_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapRigBipedSpine3pcPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapRigBipedSpine3pcXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapRigBipedSpine3pcRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapRigBipedSpine3pcUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation_rig_path(mut self, val: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_animation_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path(mut self, val: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_targetpath(mut self, val: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_root_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back1_targetpath(mut self, val: &str) -> Self {
        self.params.insert("lower_back1_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lower_back1_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back1_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back2_targetpath(mut self, val: &str) -> Self {
        self.params.insert("lower_back2_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lower_back2_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back2_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chest_targetpath(mut self, val: &str) -> Self {
        self.params.insert("chest_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_chest_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("chest_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_neck_targetpath(mut self, val: &str) -> Self {
        self.params.insert("neck_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_neck_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("neck_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapRigBipedSpine3pc {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocap_rig_biped_spine_3pc"
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
pub trait ObjectMocapRigBipedSpine3pcInnerExt {
    fn chest_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_chest(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_lower_back1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lower_back1_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lower_back2_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_chest(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn root_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapRigBipedSpine3pcInnerExt for crate::core::graph::InnerGraph<'a> {
    fn chest_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chest_bone")
    }
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chops")
    }
    fn ctrl_chest(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_chest")
    }
    fn ctrl_lower_back1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_lower_back1")
    }
    fn ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_lower_back2")
    }
    fn ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_neck")
    }
    fn ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_spine_root")
    }
    fn lower_back1_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("lower_back1_bone")
    }
    fn lower_back2_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("lower_back2_bone")
    }
    fn ptr_ctrl_chest(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_chest")
    }
    fn ptr_ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_lower_back2")
    }
    fn ptr_ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_neck")
    }
    fn ptr_ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_spine_root")
    }
    fn root_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("root_bone")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedSpine5pcPreXform {
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
pub enum ObjectMocapRigBipedSpine5pcXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedSpine5pcRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapRigBipedSpine5pcUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapRigBipedSpine5pc {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapRigBipedSpine5pc {
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



    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_bone_length(mut self, val: f32) -> Self {
        self.params.insert("root_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_root_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back1_bone_length(mut self, val: f32) -> Self {
        self.params.insert("lower_back1_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lower_back1_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back1_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back2_bone_length(mut self, val: f32) -> Self {
        self.params.insert("lower_back2_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lower_back2_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back2_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back3_bone_length(mut self, val: f32) -> Self {
        self.params.insert("lower_back3_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lower_back3_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back3_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chest1_bone_length(mut self, val: f32) -> Self {
        self.params.insert("chest1_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_chest1_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("chest1_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chest2_bone_length(mut self, val: f32) -> Self {
        self.params.insert("chest2_bone_length".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_chest2_bone_length_expr(mut self, expr: &str) -> Self {
        self.params.insert("chest2_bone_length".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_spine_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_spine_root_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_spine_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_spine_root_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_lower_back2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_lower_back2_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_lower_back2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_lower_back2_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_lower_back3_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_lower_back3_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_lower_back3_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_lower_back3_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_chest1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_chest1_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_chest1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_chest1_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_chest2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_chest2_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_chest2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_chest2_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptr_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ptr_ctrl_neck_t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ptr_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptr_ctrl_neck_t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapRigBipedSpine5pcPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapRigBipedSpine5pcXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapRigBipedSpine5pcRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapRigBipedSpine5pcUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation_rig_path(mut self, val: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_animation_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation_rig_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path(mut self, val: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mocap_skeleton_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("mocap_skeleton_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_root_targetpath(mut self, val: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_root_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("root_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back1_targetpath(mut self, val: &str) -> Self {
        self.params.insert("lower_back1_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lower_back1_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back1_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back2_targetpath(mut self, val: &str) -> Self {
        self.params.insert("lower_back2_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lower_back2_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back2_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lower_back3_targetpath(mut self, val: &str) -> Self {
        self.params.insert("lower_back3_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lower_back3_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lower_back3_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chest1_targetpath(mut self, val: &str) -> Self {
        self.params.insert("chest1_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_chest1_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("chest1_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chest2_targetpath(mut self, val: &str) -> Self {
        self.params.insert("chest2_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_chest2_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("chest2_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_neck_targetpath(mut self, val: &str) -> Self {
        self.params.insert("neck_targetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_neck_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("neck_targetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapRigBipedSpine5pc {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocap_rig_biped_spine_5pc"
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
pub trait ObjectMocapRigBipedSpine5pcInnerExt {
    fn chest1_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chest2_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_chest1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_chest2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_lower_back1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_lower_back3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lower_back1_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lower_back2_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lower_back3_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_chest1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_chest2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_lower_back3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ptr_ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn root_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapRigBipedSpine5pcInnerExt for crate::core::graph::InnerGraph<'a> {
    fn chest1_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chest1_bone")
    }
    fn chest2_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chest2_bone")
    }
    fn chops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("chops")
    }
    fn ctrl_chest1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_chest1")
    }
    fn ctrl_chest2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_chest2")
    }
    fn ctrl_lower_back1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_lower_back1")
    }
    fn ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_lower_back2")
    }
    fn ctrl_lower_back3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_lower_back3")
    }
    fn ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_neck")
    }
    fn ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ctrl_spine_root")
    }
    fn lower_back1_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("lower_back1_bone")
    }
    fn lower_back2_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("lower_back2_bone")
    }
    fn lower_back3_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("lower_back3_bone")
    }
    fn ptr_ctrl_chest1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_chest1")
    }
    fn ptr_ctrl_chest2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_chest2")
    }
    fn ptr_ctrl_lower_back2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_lower_back2")
    }
    fn ptr_ctrl_lower_back3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_lower_back3")
    }
    fn ptr_ctrl_neck(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_neck")
    }
    fn ptr_ctrl_spine_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ptr_ctrl_spine_root")
    }
    fn root_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("root_bone")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped1PreXform {
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
pub enum ObjectMocapbiped1Xord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped1Rord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped1Uparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped1Animation {
    Walk = 0,
    Run = 1,
    Wait = 2,
    Standing = 3,
    Zombie = 4,
    Rest = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped1Geotype {
    Polygon = 0,
    Polysoup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped1GeoVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapbiped1 {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapbiped1 {
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
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_speed(mut self, val: f32) -> Self {
        self.params.insert("speed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_speed_expr(mut self, expr: &str) -> Self {
        self.params.insert("speed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_frameoffset(mut self, val: f32) -> Self {
        self.params.insert("frameoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_frameoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("frameoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_t2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t2_expr(mut self, expr: &str) -> Self {
        self.params.insert("t2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r2_expr(mut self, expr: &str) -> Self {
        self.params.insert("r2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert("s2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_texture(mut self, val: i32) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_geotype(mut self, val: ObjectMocapbiped1Geotype) -> Self {
        self.params.insert("geoType".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_geotype_expr(mut self, expr: &str) -> Self {
        self.params.insert("geoType".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapbiped1PreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapbiped1Xord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapbiped1Rord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapbiped1Uparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation(mut self, val: ObjectMocapbiped1Animation) -> Self {
        self.params.insert("animation".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_animation_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_geo_vport_onionskin(mut self, val: ObjectMocapbiped1GeoVportOnionskin) -> Self {
        self.params.insert("geo_vport_onionskin".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_geo_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.params.insert("geo_vport_onionskin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inplaceanim(mut self, val: bool) -> Self {
        self.params.insert("inplaceanim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_inplaceanim_expr(mut self, expr: &str) -> Self {
        self.params.insert("inplaceanim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapbiped1 {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocapbiped1"
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
pub trait ObjectMocapbiped1InnerExt {
    fn chop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_to_headend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hips(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hips_to_lhipjoint(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hips_to_lowerback(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hips_to_rhipjoint(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lhipjoint_to_leftupleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lthumb_to_lthumbend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftarm_to_leftforearm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftfingerbase_to_lefthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftfoot_to_lefttoebase(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftforearm_to_lefthand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lefthandindex1_to_lefthandindex1end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lefthand_to_lthumb(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lefthand_to_leftfingerbase(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftleg_to_leftfoot(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftshoulder_to_leftarm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lefttoebase_to_lefttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftupleg_to_leftleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lowerback_to_spine(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn master(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn neck1_to_head(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn neck_to_neck1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rhipjoint_to_rightupleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rthumb_to_rthumbend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightarm_to_rightforearm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightfingerbase_to_righthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightfoot_to_righttoebase(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightforearm_to_righthand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn righthandindex1_to_righthandindex1end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn righthand_to_rthumb(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn righthand_to_rightfingerbase(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightleg_to_rightfoot(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightshoulder_to_rightarm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn righttoebase_to_righttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightupleg_to_rightleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine1_to_leftshoulder(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine1_to_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine1_to_rightshoulder(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine_to_spine1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rig_offset(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rig_scale(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapbiped1InnerExt for crate::core::graph::InnerGraph<'a> {
    fn chop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("CHOP")
    }
    fn head_to_headend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Head_To_HeadEnd")
    }
    fn hips(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Hips")
    }
    fn hips_to_lhipjoint(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Hips_To_LHipJoint")
    }
    fn hips_to_lowerback(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Hips_To_LowerBack")
    }
    fn hips_to_rhipjoint(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Hips_To_RHipJoint")
    }
    fn lhipjoint_to_leftupleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LHipJoint_To_LeftUpLeg")
    }
    fn lthumb_to_lthumbend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LThumb_To_LThumbEnd")
    }
    fn leftarm_to_leftforearm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftArm_To_LeftForeArm")
    }
    fn leftfingerbase_to_lefthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftFingerBase_To_LeftHandIndex1")
    }
    fn leftfoot_to_lefttoebase(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftFoot_To_LeftToeBase")
    }
    fn leftforearm_to_lefthand(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftForeArm_To_LeftHand")
    }
    fn lefthandindex1_to_lefthandindex1end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftHandIndex1_To_LeftHandIndex1End")
    }
    fn lefthand_to_lthumb(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftHand_To_LThumb")
    }
    fn lefthand_to_leftfingerbase(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftHand_To_LeftFingerBase")
    }
    fn leftleg_to_leftfoot(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftLeg_To_LeftFoot")
    }
    fn leftshoulder_to_leftarm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftShoulder_To_LeftArm")
    }
    fn lefttoebase_to_lefttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftToeBase_To_LeftToeBaseEnd")
    }
    fn leftupleg_to_leftleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftUpLeg_To_LeftLeg")
    }
    fn lowerback_to_spine(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LowerBack_To_Spine")
    }
    fn master(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MASTER")
    }
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MATERIALS")
    }
    fn neck1_to_head(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Neck1_To_Head")
    }
    fn neck_to_neck1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Neck_To_Neck1")
    }
    fn rhipjoint_to_rightupleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RHipJoint_To_RightUpLeg")
    }
    fn root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ROOT")
    }
    fn rthumb_to_rthumbend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RThumb_To_RThumbEnd")
    }
    fn rightarm_to_rightforearm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightArm_To_RightForeArm")
    }
    fn rightfingerbase_to_righthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightFingerBase_To_RightHandIndex1")
    }
    fn rightfoot_to_righttoebase(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightFoot_To_RightToeBase")
    }
    fn rightforearm_to_righthand(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightForeArm_To_RightHand")
    }
    fn righthandindex1_to_righthandindex1end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightHandIndex1_To_RightHandIndex1End")
    }
    fn righthand_to_rthumb(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightHand_To_RThumb")
    }
    fn righthand_to_rightfingerbase(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightHand_To_RightFingerBase")
    }
    fn rightleg_to_rightfoot(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightLeg_To_RightFoot")
    }
    fn rightshoulder_to_rightarm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightShoulder_To_RightArm")
    }
    fn righttoebase_to_righttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightToeBase_To_RightToeBaseEnd")
    }
    fn rightupleg_to_rightleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightUpLeg_To_RightLeg")
    }
    fn spine1_to_leftshoulder(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Spine1_To_LeftShoulder")
    }
    fn spine1_to_neck(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Spine1_To_Neck")
    }
    fn spine1_to_rightshoulder(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Spine1_To_RightShoulder")
    }
    fn spine_to_spine1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Spine_To_Spine1")
    }
    fn geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("geo")
    }
    fn rig_offset(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("rig_offset")
    }
    fn rig_scale(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("rig_scale")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped2PreXform {
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
pub enum ObjectMocapbiped2Xord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped2Rord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped2Uparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped2Animation {
    Jogging = 0,
    Running = 1,
    SitCheer = 2,
    SitInteract = 3,
    SitToStandCheer = 4,
    SitToStand = 5,
    StandCheer = 6,
    StandInteract = 7,
    StandLook = 8,
    StandTalk = 9,
    StandToSitClap = 10,
    StandToWalk = 11,
    WalkFast = 12,
    WalkNormal = 13,
    Rest = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped2Geotype {
    Polygon = 0,
    Polysoup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped2VportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapbiped2 {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapbiped2 {
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
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_speed(mut self, val: f32) -> Self {
        self.params.insert("speed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_speed_expr(mut self, expr: &str) -> Self {
        self.params.insert("speed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_frameoffset(mut self, val: f32) -> Self {
        self.params.insert("frameoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_frameoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("frameoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_t2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t2_expr(mut self, expr: &str) -> Self {
        self.params.insert("t2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r2_expr(mut self, expr: &str) -> Self {
        self.params.insert("r2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert("s2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_texture(mut self, val: i32) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_geotype(mut self, val: ObjectMocapbiped2Geotype) -> Self {
        self.params.insert("geoType".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_geotype_expr(mut self, expr: &str) -> Self {
        self.params.insert("geoType".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapbiped2PreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapbiped2Xord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapbiped2Rord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapbiped2Uparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_animation(mut self, val: ObjectMocapbiped2Animation) -> Self {
        self.params.insert("animation".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_animation_expr(mut self, expr: &str) -> Self {
        self.params.insert("animation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_onionskin(mut self, val: ObjectMocapbiped2VportOnionskin) -> Self {
        self.params.insert("vport_onionskin".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_onionskin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inplaceanim(mut self, val: bool) -> Self {
        self.params.insert("inplaceanim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_inplaceanim_expr(mut self, expr: &str) -> Self {
        self.params.insert("inplaceanim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapbiped2 {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocapbiped2"
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
pub trait ObjectMocapbiped2InnerExt {
    fn chop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn master(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ank_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ank_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ank_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ank_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ball_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ball_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bicep_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bicep_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bicep_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bicep_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn clav_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn clav_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn clav_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn clav_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn elbow_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn elbow_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn elbow_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn elbow_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hip_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hip_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hip_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hip_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn index2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn indexbase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn indexbase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn indexbase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn indexbase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn jaw_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn knee_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn knee_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn knee_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn knee_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middle2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middlebase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middlebase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middlebase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn middlebase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pelvis_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pelvis_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pelvis_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pelvis_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinky2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinkybase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinkybase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinkybase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pinkybase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rescale(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ring2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ringbase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ringbase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ringbase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ringbase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sho_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sho_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sho_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sho_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfnecktip_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfnecktip_0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt1_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt1_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt2_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt2_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt3_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn surfneck_0_jnt3_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thigh_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thigh_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thigh_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thigh_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumb2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumbbase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumbbase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumbbase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn thumbbase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspinetip_c_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspinetip_c_0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspinetip_c_0_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspinetip_c_0_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_1_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_1_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_2_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_2_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_3_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_3_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_4_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn twisterspine_c_0_vrb_4_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ulna_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ulna_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ulna_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ulna_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_l_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_l_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_l_bnd_bone3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_l_bnd_bone4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_r_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_r_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_r_bnd_bone3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wrist_0_r_bnd_bone4(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapbiped2InnerExt for crate::core::graph::InnerGraph<'a> {
    fn chop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("CHOP")
    }
    fn master(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MASTER")
    }
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MATERIALS")
    }
    fn ank_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ank_0_L_Bnd")
    }
    fn ank_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ank_0_L_Bnd_bone")
    }
    fn ank_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ank_0_R_Bnd")
    }
    fn ank_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ank_0_R_Bnd_bone")
    }
    fn ball_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ball_0_L_Bnd")
    }
    fn ball_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ball_0_R_Bnd")
    }
    fn bicep_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("bicep_0_L_Bnd")
    }
    fn bicep_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("bicep_0_L_Bnd_bone")
    }
    fn bicep_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("bicep_0_R_Bnd")
    }
    fn bicep_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("bicep_0_R_Bnd_bone")
    }
    fn clav_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("clav_0_L_Bnd")
    }
    fn clav_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("clav_0_L_Bnd_bone")
    }
    fn clav_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("clav_0_R_Bnd")
    }
    fn clav_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("clav_0_R_Bnd_bone")
    }
    fn elbow_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("elbow_0_L_Bnd")
    }
    fn elbow_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("elbow_0_L_Bnd_bone")
    }
    fn elbow_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("elbow_0_R_Bnd")
    }
    fn elbow_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("elbow_0_R_Bnd_bone")
    }
    fn geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("geo")
    }
    fn hip_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("hip_0_L_Bnd")
    }
    fn hip_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("hip_0_L_Bnd_bone")
    }
    fn hip_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("hip_0_R_Bnd")
    }
    fn hip_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("hip_0_R_Bnd_bone")
    }
    fn index0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index0_0_L_Bnd")
    }
    fn index0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index0_0_L_Bnd_bone")
    }
    fn index0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index0_0_R_Bnd")
    }
    fn index0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index0_0_R_Bnd_bone")
    }
    fn index1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index1_0_L_Bnd")
    }
    fn index1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index1_0_L_Bnd_bone")
    }
    fn index1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index1_0_R_Bnd")
    }
    fn index1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index1_0_R_Bnd_bone")
    }
    fn index2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index2_0_L_Bnd")
    }
    fn index2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("index2_0_R_Bnd")
    }
    fn indexbase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("indexBase_0_L_Bnd")
    }
    fn indexbase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("indexBase_0_L_Bnd_bone")
    }
    fn indexbase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("indexBase_0_R_Bnd")
    }
    fn indexbase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("indexBase_0_R_Bnd_bone")
    }
    fn jaw_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("jaw_0_Bnd")
    }
    fn knee_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("knee_0_L_Bnd")
    }
    fn knee_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("knee_0_L_Bnd_bone")
    }
    fn knee_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("knee_0_R_Bnd")
    }
    fn knee_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("knee_0_R_Bnd_bone")
    }
    fn middle0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle0_0_L_Bnd")
    }
    fn middle0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle0_0_L_Bnd_bone")
    }
    fn middle0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle0_0_R_Bnd")
    }
    fn middle0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle0_0_R_Bnd_bone")
    }
    fn middle1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle1_0_L_Bnd")
    }
    fn middle1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle1_0_L_Bnd_bone")
    }
    fn middle1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle1_0_R_Bnd")
    }
    fn middle1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle1_0_R_Bnd_bone")
    }
    fn middle2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle2_0_L_Bnd")
    }
    fn middle2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middle2_0_R_Bnd")
    }
    fn middlebase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middleBase_0_L_Bnd")
    }
    fn middlebase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middleBase_0_L_Bnd_bone")
    }
    fn middlebase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middleBase_0_R_Bnd")
    }
    fn middlebase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("middleBase_0_R_Bnd_bone")
    }
    fn pelvis_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pelvis_Bnd")
    }
    fn pelvis_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pelvis_Bnd_bone")
    }
    fn pelvis_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pelvis_Bnd_bone1")
    }
    fn pelvis_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pelvis_Bnd_bone2")
    }
    fn pinky0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky0_0_L_Bnd")
    }
    fn pinky0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky0_0_L_Bnd_bone")
    }
    fn pinky0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky0_0_R_Bnd")
    }
    fn pinky0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky0_0_R_Bnd_bone")
    }
    fn pinky1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky1_0_L_Bnd")
    }
    fn pinky1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky1_0_L_Bnd_bone")
    }
    fn pinky1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky1_0_R_Bnd")
    }
    fn pinky1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky1_0_R_Bnd_bone")
    }
    fn pinky2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky2_0_L_Bnd")
    }
    fn pinky2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinky2_0_R_Bnd")
    }
    fn pinkybase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinkyBase_0_L_Bnd")
    }
    fn pinkybase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinkyBase_0_L_Bnd_bone")
    }
    fn pinkybase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinkyBase_0_R_Bnd")
    }
    fn pinkybase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("pinkyBase_0_R_Bnd_bone")
    }
    fn rescale(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("rescale")
    }
    fn ring0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring0_0_L_Bnd")
    }
    fn ring0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring0_0_L_Bnd_bone")
    }
    fn ring0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring0_0_R_Bnd")
    }
    fn ring0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring0_0_R_Bnd_bone")
    }
    fn ring1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring1_0_L_Bnd")
    }
    fn ring1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring1_0_L_Bnd_bone")
    }
    fn ring1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring1_0_R_Bnd")
    }
    fn ring1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring1_0_R_Bnd_bone")
    }
    fn ring2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring2_0_L_Bnd")
    }
    fn ring2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ring2_0_R_Bnd")
    }
    fn ringbase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ringBase_0_L_Bnd")
    }
    fn ringbase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ringBase_0_L_Bnd_bone")
    }
    fn ringbase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ringBase_0_R_Bnd")
    }
    fn ringbase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ringBase_0_R_Bnd_bone")
    }
    fn sho_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sho_0_L_Bnd")
    }
    fn sho_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sho_0_L_Bnd_bone")
    }
    fn sho_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sho_0_R_Bnd")
    }
    fn sho_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("sho_0_R_Bnd_bone")
    }
    fn surfnecktip_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneckTip_0_Bnd")
    }
    fn surfnecktip_0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneckTip_0_Bnd_bone")
    }
    fn surfneck_0_jnt0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt0_Bnd")
    }
    fn surfneck_0_jnt0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt0_Bnd_bone")
    }
    fn surfneck_0_jnt1_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt1_Bnd")
    }
    fn surfneck_0_jnt1_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt1_Bnd_bone")
    }
    fn surfneck_0_jnt2_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt2_Bnd")
    }
    fn surfneck_0_jnt2_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt2_Bnd_bone")
    }
    fn surfneck_0_jnt3_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt3_Bnd")
    }
    fn surfneck_0_jnt3_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("surfneck_0_jnt3_Bnd_bone")
    }
    fn thigh_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thigh_0_L_Bnd")
    }
    fn thigh_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thigh_0_L_Bnd_bone")
    }
    fn thigh_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thigh_0_R_Bnd")
    }
    fn thigh_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thigh_0_R_Bnd_bone")
    }
    fn thumb0_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb0_0_L_Bnd")
    }
    fn thumb0_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb0_0_L_Bnd_bone")
    }
    fn thumb0_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb0_0_R_Bnd")
    }
    fn thumb0_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb0_0_R_Bnd_bone")
    }
    fn thumb1_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb1_0_L_Bnd")
    }
    fn thumb1_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb1_0_L_Bnd_bone")
    }
    fn thumb1_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb1_0_R_Bnd")
    }
    fn thumb1_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb1_0_R_Bnd_bone")
    }
    fn thumb2_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb2_0_L_Bnd")
    }
    fn thumb2_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumb2_0_R_Bnd")
    }
    fn thumbbase_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumbBase_0_L_Bnd")
    }
    fn thumbbase_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumbBase_0_L_Bnd_bone")
    }
    fn thumbbase_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumbBase_0_R_Bnd")
    }
    fn thumbbase_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("thumbBase_0_R_Bnd_bone")
    }
    fn twisterspinetip_c_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspineTip_C_0_Bnd")
    }
    fn twisterspinetip_c_0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspineTip_C_0_Bnd_bone")
    }
    fn twisterspinetip_c_0_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspineTip_C_0_Bnd_bone1")
    }
    fn twisterspinetip_c_0_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspineTip_C_0_Bnd_bone2")
    }
    fn twisterspine_c_0_vrb_0_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_0_Bnd")
    }
    fn twisterspine_c_0_vrb_0_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_0_Bnd_bone")
    }
    fn twisterspine_c_0_vrb_1_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_1_Bnd")
    }
    fn twisterspine_c_0_vrb_1_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_1_Bnd_bone")
    }
    fn twisterspine_c_0_vrb_2_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_2_Bnd")
    }
    fn twisterspine_c_0_vrb_2_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_2_Bnd_bone")
    }
    fn twisterspine_c_0_vrb_3_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_3_Bnd")
    }
    fn twisterspine_c_0_vrb_3_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_3_Bnd_bone")
    }
    fn twisterspine_c_0_vrb_4_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_4_Bnd")
    }
    fn twisterspine_c_0_vrb_4_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("twisterspine_C_0_vrb_4_Bnd_bone")
    }
    fn ulna_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ulna_0_L_Bnd")
    }
    fn ulna_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ulna_0_L_Bnd_bone")
    }
    fn ulna_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ulna_0_R_Bnd")
    }
    fn ulna_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("ulna_0_R_Bnd_bone")
    }
    fn wrist_0_l_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_L_Bnd")
    }
    fn wrist_0_l_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_L_Bnd_bone")
    }
    fn wrist_0_l_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_L_Bnd_bone1")
    }
    fn wrist_0_l_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_L_Bnd_bone2")
    }
    fn wrist_0_l_bnd_bone3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_L_Bnd_bone3")
    }
    fn wrist_0_l_bnd_bone4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_L_Bnd_bone4")
    }
    fn wrist_0_r_bnd(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_R_Bnd")
    }
    fn wrist_0_r_bnd_bone(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_R_Bnd_bone")
    }
    fn wrist_0_r_bnd_bone1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_R_Bnd_bone1")
    }
    fn wrist_0_r_bnd_bone2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_R_Bnd_bone2")
    }
    fn wrist_0_r_bnd_bone3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_R_Bnd_bone3")
    }
    fn wrist_0_r_bnd_bone4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("wrist_0_R_Bnd_bone4")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3PreXform {
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
pub enum ObjectMocapbiped3Xord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Rord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Uparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3AnimTypes {
    WalksAndTurns = 0,
    Runs = 1,
    Stadium = 2,
    Zombie = 3,
    Inclines = 4,
    Injured = 5,
    Steps = 6,
    Poses = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Geotype {
    Polygon = 0,
    Polysoup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Deformmethod {
    LinearSkinning = 0,
    DualQuaternionSkinning = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3VportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Textures {
    Casual = 0,
    Military = 1,
    Zombie = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Casual {
    CasualDefault = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Military {
    MilitaryDefault = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMocapbiped3Zombie2 {
    ZombieDefault = 0,
}

#[derive(Debug, Clone)]
pub struct ObjectMocapbiped3 {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMocapbiped3 {
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
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniform_scale(mut self, val: f32) -> Self {
        self.params.insert("uniform_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_uniform_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniform_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_speed(mut self, val: f32) -> Self {
        self.params.insert("speed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_speed_expr(mut self, expr: &str) -> Self {
        self.params.insert("speed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_frameoffset(mut self, val: f32) -> Self {
        self.params.insert("frameoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_frameoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("frameoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_master_trans(mut self, val: [f32; 3]) -> Self {
        self.params.insert("master_trans".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_master_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("master_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_master_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert("master_rotate".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_master_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert("master_rotate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nframes(mut self, val: i32) -> Self {
        self.params.insert("nFrames".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_nframes_expr(mut self, expr: &str) -> Self {
        self.params.insert("nFrames".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_geotype(mut self, val: ObjectMocapbiped3Geotype) -> Self {
        self.params.insert("geoType".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_geotype_expr(mut self, expr: &str) -> Self {
        self.params.insert("geoType".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_texture(mut self, val: i32) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert("texture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMocapbiped3PreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMocapbiped3Xord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMocapbiped3Rord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMocapbiped3Uparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_anim_types(mut self, val: ObjectMocapbiped3AnimTypes) -> Self {
        self.params.insert("anim_types".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_anim_types_expr(mut self, expr: &str) -> Self {
        self.params.insert("anim_types".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_deformmethod(mut self, val: ObjectMocapbiped3Deformmethod) -> Self {
        self.params.insert("deformmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_deformmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("deformmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_onionskin(mut self, val: ObjectMocapbiped3VportOnionskin) -> Self {
        self.params.insert("vport_onionskin".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_onionskin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_textures(mut self, val: ObjectMocapbiped3Textures) -> Self {
        self.params.insert("textures".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_textures_expr(mut self, expr: &str) -> Self {
        self.params.insert("textures".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_casual(mut self, val: ObjectMocapbiped3Casual) -> Self {
        self.params.insert("casual".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_casual_expr(mut self, expr: &str) -> Self {
        self.params.insert("casual".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_military(mut self, val: ObjectMocapbiped3Military) -> Self {
        self.params.insert("military".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_military_expr(mut self, expr: &str) -> Self {
        self.params.insert("military".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_zombie2(mut self, val: ObjectMocapbiped3Zombie2) -> Self {
        self.params.insert("zombie2".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_zombie2_expr(mut self, expr: &str) -> Self {
        self.params.insert("zombie2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_walks_and_turns(mut self, val: &str) -> Self {
        self.params.insert("walks_and_turns".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_walks_and_turns_expr(mut self, expr: &str) -> Self {
        self.params.insert("walks_and_turns".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_runs(mut self, val: &str) -> Self {
        self.params.insert("runs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_runs_expr(mut self, expr: &str) -> Self {
        self.params.insert("runs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stadium(mut self, val: &str) -> Self {
        self.params.insert("stadium".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stadium_expr(mut self, expr: &str) -> Self {
        self.params.insert("stadium".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_zombie(mut self, val: &str) -> Self {
        self.params.insert("zombie".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_zombie_expr(mut self, expr: &str) -> Self {
        self.params.insert("zombie".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inclines(mut self, val: &str) -> Self {
        self.params.insert("inclines".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_inclines_expr(mut self, expr: &str) -> Self {
        self.params.insert("inclines".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_injured(mut self, val: &str) -> Self {
        self.params.insert("injured".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_injured_expr(mut self, expr: &str) -> Self {
        self.params.insert("injured".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_steps(mut self, val: &str) -> Self {
        self.params.insert("steps".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_steps_expr(mut self, expr: &str) -> Self {
        self.params.insert("steps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_poses(mut self, val: &str) -> Self {
        self.params.insert("poses".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_poses_expr(mut self, expr: &str) -> Self {
        self.params.insert("poses".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inplaceanim(mut self, val: bool) -> Self {
        self.params.insert("inplaceanim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_inplaceanim_expr(mut self, expr: &str) -> Self {
        self.params.insert("inplaceanim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extendmatch(mut self, val: bool) -> Self {
        self.params.insert("extendMatch".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_extendmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("extendMatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMocapbiped3 {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mocapbiped3"
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
pub trait ObjectMocapbiped3InnerExt {
    fn chop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_to_headend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hips(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn incline_proxy(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lhipjoint_to_leftupleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftarm_to_leftforearm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftfingerbase_to_lefthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftfoot_to_lefttoebase(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftforearm_to_lefthand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftleg_to_leftfoot(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftshoulder_to_leftarm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lefttoebase_to_lefttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn leftupleg_to_leftleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lowerback_to_spine(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn master(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn neck1_to_head(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn neck_to_neck1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rhipjoint_to_rightupleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightarm_to_rightforearm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightfingerbase_to_righthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightfoot_to_righttoebase(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightforearm_to_righthand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightleg_to_rightfoot(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightshoulder_to_rightarm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn righttoebase_to_righttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rightupleg_to_rightleg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn step_proxy(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine_to_spine1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn geo_skin(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMocapbiped3InnerExt for crate::core::graph::InnerGraph<'a> {
    fn chop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("CHOP")
    }
    fn head_to_headend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Head_To_HeadEnd")
    }
    fn hips(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Hips")
    }
    fn incline_proxy(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("INCLINE_PROXY")
    }
    fn lhipjoint_to_leftupleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LHipJoint_To_LeftUpLeg")
    }
    fn leftarm_to_leftforearm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftArm_To_LeftForeArm")
    }
    fn leftfingerbase_to_lefthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftFingerBase_To_LeftHandIndex1")
    }
    fn leftfoot_to_lefttoebase(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftFoot_To_LeftToeBase")
    }
    fn leftforearm_to_lefthand(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftForeArm_To_LeftHand")
    }
    fn leftleg_to_leftfoot(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftLeg_To_LeftFoot")
    }
    fn leftshoulder_to_leftarm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftShoulder_To_LeftArm")
    }
    fn lefttoebase_to_lefttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftToeBase_To_LeftToeBaseEnd")
    }
    fn leftupleg_to_leftleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LeftUpLeg_To_LeftLeg")
    }
    fn lowerback_to_spine(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("LowerBack_To_Spine")
    }
    fn master(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MASTER")
    }
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("MATERIALS")
    }
    fn neck1_to_head(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Neck1_To_Head")
    }
    fn neck_to_neck1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Neck_To_Neck1")
    }
    fn rhipjoint_to_rightupleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RHipJoint_To_RightUpLeg")
    }
    fn rightarm_to_rightforearm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightArm_To_RightForeArm")
    }
    fn rightfingerbase_to_righthandindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightFingerBase_To_RightHandIndex1")
    }
    fn rightfoot_to_righttoebase(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightFoot_To_RightToeBase")
    }
    fn rightforearm_to_righthand(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightForeArm_To_RightHand")
    }
    fn rightleg_to_rightfoot(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightLeg_To_RightFoot")
    }
    fn rightshoulder_to_rightarm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightShoulder_To_RightArm")
    }
    fn righttoebase_to_righttoebaseend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightToeBase_To_RightToeBaseEnd")
    }
    fn rightupleg_to_rightleg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("RightUpLeg_To_RightLeg")
    }
    fn step_proxy(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("STEP_PROXY")
    }
    fn spine_to_spine1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Spine_To_Spine1")
    }
    fn geo_skin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("geo_skin")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMuscleMuscletype {
    MetaballCapture = 0,
    SpheresOnly = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMuscleBasis {
    Bezier = 0,
    SpecialBezier = 1,
    SpecialSmoothBezier = 2,
    Degree2Bezier = 3,
    Bspline = 4,
    Cardinal = 5,
    Linear = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMuscleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMuscleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclePreXform {
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
pub enum ObjectMuscleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMuscle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl ObjectMuscle {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }


    // --- Float parameters ---
    pub fn with_positionbias(mut self, val: f32) -> Self {
        self.params.insert("positionbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_positionbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("positionbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control2position(mut self, val: f32) -> Self {
        self.params.insert("control2position".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert("control2position".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control3position(mut self, val: f32) -> Self {
        self.params.insert("control3position".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert("control3position".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control4position(mut self, val: f32) -> Self {
        self.params.insert("control4position".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert("control4position".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gradient(mut self, val: f32) -> Self {
        self.params.insert("gradient".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gradient_expr(mut self, expr: &str) -> Self {
        self.params.insert("gradient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tension(mut self, val: f32) -> Self {
        self.params.insert("tension".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tension_expr(mut self, expr: &str) -> Self {
        self.params.insert("tension".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_musclescale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("musclescale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_musclescale_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclescale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("control1scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control1scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("control2scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control2scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("control3scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control3scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("control4scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control4scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("control5scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("control5scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_musclecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("musclecolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_musclecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclecolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_primspersegment(mut self, val: i32) -> Self {
        self.params.insert("primspersegment".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_primspersegment_expr(mut self, expr: &str) -> Self {
        self.params.insert("primspersegment".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sampledivs(mut self, val: i32) -> Self {
        self.params.insert("sampledivs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_sampledivs_expr(mut self, expr: &str) -> Self {
        self.params.insert("sampledivs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_first(mut self, val: i32) -> Self {
        self.params.insert("first".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_first_expr(mut self, expr: &str) -> Self {
        self.params.insert("first".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_last(mut self, val: i32) -> Self {
        self.params.insert("last".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_last_expr(mut self, expr: &str) -> Self {
        self.params.insert("last".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_muscletype(mut self, val: ObjectMuscleMuscletype) -> Self {
        self.params.insert("muscletype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_muscletype_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscletype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_basis(mut self, val: ObjectMuscleBasis) -> Self {
        self.params.insert("basis".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert("basis".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMuscleXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMuscleRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectMusclePreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMuscleUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert("shop_materialopts".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_materialopts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_musclename(mut self, val: &str) -> Self {
        self.params.insert("musclename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_musclename_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_restanchor(mut self, val: &str) -> Self {
        self.params.insert("restanchor".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_restanchor_expr(mut self, expr: &str) -> Self {
        self.params.insert("restanchor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert("shop_materialpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_materialpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_togglecolor(mut self, val: bool) -> Self {
        self.params.insert("togglecolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_togglecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("togglecolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enablegradient(mut self, val: bool) -> Self {
        self.params.insert("enablegradient".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enablegradient_expr(mut self, expr: &str) -> Self {
        self.params.insert("enablegradient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert("vport_shadeopen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_shadeopen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert("vport_displayassubdiv".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_displayassubdiv".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMuscle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "muscle"
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
pub trait ObjectMuscleInnerExt {
    fn muscle(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMuscleInnerExt for crate::core::graph::InnerGraph<'a> {
    fn muscle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("muscle")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclepinPreXform {
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
pub enum ObjectMusclepinXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclepinRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclepinUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclepinUseexternalgeo {
    /// Built-In Geometry
    BuiltMinusInGeometry = 0,
    ExternalGeometry = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMusclepin {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMusclepin {
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

    /// Connects to input 0: "Origin Anchor"
    pub fn set_input_origin_anchor<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Origin Anchor" and specifies the output index of the target node.
    pub fn set_input_origin_anchor_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_resethandles(mut self) -> Self {
        self.params.insert("resethandles".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_savecapturepose(mut self) -> Self {
        self.params.insert("savecapturepose".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_exportcapturepose(mut self) -> Self {
        self.params.insert("exportcapturepose".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_resetcapturehandles(mut self) -> Self {
        self.params.insert("resetcapturehandles".to_string(), crate::core::types::ParamValue::Button);
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_muscletension(mut self, val: f32) -> Self {
        self.params.insert("muscletension".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_muscletension_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscletension".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_handlescale(mut self, val: f32) -> Self {
        self.params.insert("handlescale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_handlescale_expr(mut self, expr: &str) -> Self {
        self.params.insert("handlescale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_headradius(mut self, val: f32) -> Self {
        self.params.insert("headradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_headradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("headradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_captureprofilemin(mut self, val: f32) -> Self {
        self.params.insert("captureprofilemin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_captureprofilemin_expr(mut self, expr: &str) -> Self {
        self.params.insert("captureprofilemin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos1t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos1t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos1t_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos1t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos1r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos1r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos1r_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos1r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos1s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos1s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos1s_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos1s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos1t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos1t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos1t_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos1t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos1r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos1r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos1r_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos1r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos1s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos1s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos1s_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos1s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert("color".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert("color".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useexternalgeo(mut self, val: ObjectMusclepinUseexternalgeo) -> Self {
        self.params.insert("useexternalgeo".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_useexternalgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert("useexternalgeo".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_divs(mut self, val: i32) -> Self {
        self.params.insert("divs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_divs_expr(mut self, expr: &str) -> Self {
        self.params.insert("divs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMusclepinPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMusclepinXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMusclepinRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMusclepinUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputtransform(mut self, val: &str) -> Self {
        self.params.insert("outputtransform".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputtransform_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputtransform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_externalgeopath(mut self, val: &str) -> Self {
        self.params.insert("externalgeopath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_externalgeopath_expr(mut self, expr: &str) -> Self {
        self.params.insert("externalgeopath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_externalgeogroup(mut self, val: &str) -> Self {
        self.params.insert("externalgeogroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_externalgeogroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("externalgeogroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keepposhandles(mut self, val: bool) -> Self {
        self.params.insert("keepposhandles".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepposhandles_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepposhandles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displaycapturepose(mut self, val: bool) -> Self {
        self.params.insert("displaycapturepose".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displaycapturepose_expr(mut self, expr: &str) -> Self {
        self.params.insert("displaycapturepose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayconstraintradius(mut self, val: bool) -> Self {
        self.params.insert("displayconstraintradius".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayconstraintradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayconstraintradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayhandles(mut self, val: bool) -> Self {
        self.params.insert("displayhandles".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayhandles_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayhandles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMusclepin {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "musclepin"
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
pub trait ObjectMusclepinInnerExt {
    fn capture_handle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn constraint_region(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn default_geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn handle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_radius(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_radius_capture(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_radius_handle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn internal_core(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMusclepinInnerExt for crate::core::graph::InnerGraph<'a> {
    fn capture_handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("capture_handle1")
    }
    fn constraint_region(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("constraint_region")
    }
    fn default_geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("default_geo")
    }
    fn handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("handle1")
    }
    fn head_radius(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("head_radius")
    }
    fn head_radius_capture(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("head_radius_capture")
    }
    fn head_radius_handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("head_radius_handle1")
    }
    fn internal_core(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("internal_core")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclerigPreXform {
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
pub enum ObjectMusclerigXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclerigRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclerigUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclerigConcentricdriver {
    Off = 0,
    DrivenByLength = 1,
    DrivenByTension = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclerigBuildtype {
    /// Built-In Spheres
    BuiltMinusInSpheres = 0,
    /// Built-In Tet Mesh
    BuiltMinusInTetMesh = 1,
    /// Built-in Tube
    BuiltMinusInTube = 2,
    ExternalGeometry = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMusclerigStrokeasrig {
    TreatStrokeAsMuscle = 0,
    TreatStrokeAsRig = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectMusclerig {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectMusclerig {
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

    /// Connects to input 0: "Origin Anchor"
    pub fn set_input_origin_anchor<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Origin Anchor" and specifies the output index of the target node.
    pub fn set_input_origin_anchor_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "End Anchor"
    pub fn set_input_end_anchor<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "End Anchor" and specifies the output index of the target node.
    pub fn set_input_end_anchor_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_setcompressedlength(mut self) -> Self {
        self.params.insert("setcompressedlength".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_setstretchedlength(mut self) -> Self {
        self.params.insert("setstretchedlength".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_resethandles(mut self) -> Self {
        self.params.insert("resethandles".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_savecapturepose(mut self) -> Self {
        self.params.insert("savecapturepose".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_exportcapturepose(mut self) -> Self {
        self.params.insert("exportcapturepose".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_dostroke(mut self) -> Self {
        self.params.insert("dostroke".to_string(), crate::core::types::ParamValue::Button);
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_muscletension(mut self, val: f32) -> Self {
        self.params.insert("muscletension".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_muscletension_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscletension".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_isometricbias(mut self, val: f32) -> Self {
        self.params.insert("isometricbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_isometricbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("isometricbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_musclebiashigh(mut self, val: f32) -> Self {
        self.params.insert("musclebiashigh".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_musclebiashigh_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclebiashigh".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_isometriccontraction(mut self, val: f32) -> Self {
        self.params.insert("isometriccontraction".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_isometriccontraction_expr(mut self, expr: &str) -> Self {
        self.params.insert("isometriccontraction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_musclecontractionhigh(mut self, val: f32) -> Self {
        self.params.insert("musclecontractionhigh".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_musclecontractionhigh_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclecontractionhigh".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_muscletightness(mut self, val: f32) -> Self {
        self.params.insert("muscletightness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_muscletightness_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscletightness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_muscletightnesshigh(mut self, val: f32) -> Self {
        self.params.insert("muscletightnesshigh".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_muscletightnesshigh_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscletightnesshigh".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_compressedscale(mut self, val: f32) -> Self {
        self.params.insert("compressedscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_compressedscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("compressedscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stretchedscale(mut self, val: f32) -> Self {
        self.params.insert("stretchedscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stretchedscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("stretchedscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_musclehightensionscale(mut self, val: f32) -> Self {
        self.params.insert("musclehightensionscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_musclehightensionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclehightensionscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_musclelowtensionscale(mut self, val: f32) -> Self {
        self.params.insert("musclelowtensionscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_musclelowtensionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("musclelowtensionscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_compressedlength(mut self, val: f32) -> Self {
        self.params.insert("compressedlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_compressedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("compressedlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stretchedlength(mut self, val: f32) -> Self {
        self.params.insert("stretchedlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stretchedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("stretchedlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_handlescale(mut self, val: f32) -> Self {
        self.params.insert("handlescale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_handlescale_expr(mut self, expr: &str) -> Self {
        self.params.insert("handlescale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_profilemin(mut self, val: f32) -> Self {
        self.params.insert("profilemin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_profilemin_expr(mut self, expr: &str) -> Self {
        self.params.insert("profilemin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_profilemax(mut self, val: f32) -> Self {
        self.params.insert("profilemax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_profilemax_expr(mut self, expr: &str) -> Self {
        self.params.insert("profilemax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_coreinnercopyscale(mut self, val: f32) -> Self {
        self.params.insert("coreinnercopyscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_coreinnercopyscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("coreinnercopyscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capturecoreradius(mut self, val: f32) -> Self {
        self.params.insert("capturecoreradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_capturecoreradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("capturecoreradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_captureprofilemin(mut self, val: f32) -> Self {
        self.params.insert("captureprofilemin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_captureprofilemin_expr(mut self, expr: &str) -> Self {
        self.params.insert("captureprofilemin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_captureprofilemax(mut self, val: f32) -> Self {
        self.params.insert("captureprofilemax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_captureprofilemax_expr(mut self, expr: &str) -> Self {
        self.params.insert("captureprofilemax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jiggleamount(mut self, val: f32) -> Self {
        self.params.insert("jiggleamount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_jiggleamount_expr(mut self, expr: &str) -> Self {
        self.params.insert("jiggleamount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stiff(mut self, val: f32) -> Self {
        self.params.insert("stiff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stiff_expr(mut self, expr: &str) -> Self {
        self.params.insert("stiff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_damp(mut self, val: f32) -> Self {
        self.params.insert("damp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_damp_expr(mut self, expr: &str) -> Self {
        self.params.insert("damp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limit(mut self, val: f32) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flex(mut self, val: f32) -> Self {
        self.params.insert("flex".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_flex_expr(mut self, expr: &str) -> Self {
        self.params.insert("flex".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jiggletimerange(mut self, val: f32) -> Self {
        self.params.insert("jiggletimerange".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_jiggletimerange_expr(mut self, expr: &str) -> Self {
        self.params.insert("jiggletimerange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_headradius(mut self, val: f32) -> Self {
        self.params.insert("headradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_headradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("headradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tailradius(mut self, val: f32) -> Self {
        self.params.insert("tailradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tailradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("tailradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_muscleradiusscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("muscleradiusscale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_muscleradiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscleradiusscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos1t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos1t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos1t_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos1t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos1r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos1r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos1r_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos1r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos1s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos1s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos1s_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos1s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos2t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos2t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos2t_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos2t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos2r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos2r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos2r_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos2r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos2s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos2s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos2s_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos2s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos3t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos3t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos3t_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos3t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos3r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos3r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos3r_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos3r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos3s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos3s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos3s_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos3s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos4t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos4t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos4t_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos4t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos4r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos4r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos4r_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos4r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos4s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos4s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos4s_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos4s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos5t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos5t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos5t_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos5t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos5r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos5r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos5r_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos5r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos5s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pos5s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pos5s_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos5s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sphereradius(mut self, val: [f32; 3]) -> Self {
        self.params.insert("sphereradius".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_sphereradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("sphereradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos1t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos1t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos1t_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos1t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos1r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos1r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos1r_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos1r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos1s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos1s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos1s_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos1s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos2t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos2t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos2t_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos2t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos2r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos2r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos2r_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos2r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos2s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos2s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos2s_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos2s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos3t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos3t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos3t_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos3t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos3r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos3r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos3r_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos3r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos3s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos3s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos3s_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos3s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos4t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos4t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos4t_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos4t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos4r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos4r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos4r_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos4r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos4s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos4s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos4s_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos4s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos5t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos5t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos5t_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos5t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos5r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos5r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos5r_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos5r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capture_pos5s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capture_pos5s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capture_pos5s_expr(mut self, expr: &str) -> Self {
        self.params.insert("capture_pos5s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capturemuscleradiusscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capturemuscleradiusscale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capturemuscleradiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("capturemuscleradiusscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_capturesphereradius(mut self, val: [f32; 3]) -> Self {
        self.params.insert("capturesphereradius".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_capturesphereradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("capturesphereradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert("mult".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert("mult".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_concentricdriver(mut self, val: ObjectMusclerigConcentricdriver) -> Self {
        self.params.insert("concentricdriver".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_concentricdriver_expr(mut self, expr: &str) -> Self {
        self.params.insert("concentricdriver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_buildtype(mut self, val: ObjectMusclerigBuildtype) -> Self {
        self.params.insert("buildtype".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_buildtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("buildtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_segs(mut self, val: i32) -> Self {
        self.params.insert("segs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_segs_expr(mut self, expr: &str) -> Self {
        self.params.insert("segs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_coresections(mut self, val: i32) -> Self {
        self.params.insert("coresections".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_coresections_expr(mut self, expr: &str) -> Self {
        self.params.insert("coresections".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_coreinnercopies(mut self, val: i32) -> Self {
        self.params.insert("coreinnercopies".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_coreinnercopies_expr(mut self, expr: &str) -> Self {
        self.params.insert("coreinnercopies".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strokeasrig(mut self, val: ObjectMusclerigStrokeasrig) -> Self {
        self.params.insert("strokeasrig".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_strokeasrig_expr(mut self, expr: &str) -> Self {
        self.params.insert("strokeasrig".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectMusclerigPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: ObjectMusclerigXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: ObjectMusclerigRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectMusclerigUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_profile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("profile".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_profile_expr(mut self, expr: &str) -> Self {
        self.params.insert("profile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetstiffnessvramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("targetstiffnessvramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_targetstiffnessvramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetstiffnessvramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_colorramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("colorramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_colorramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("colorramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_muscleid(mut self, val: &str) -> Self {
        self.params.insert("muscleid".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_muscleid_expr(mut self, expr: &str) -> Self {
        self.params.insert("muscleid".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputtransform(mut self, val: &str) -> Self {
        self.params.insert("outputtransform".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputtransform_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputtransform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_projectionpath(mut self, val: &str) -> Self {
        self.params.insert("projectionpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_projectionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("projectionpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_biasdriver(mut self, val: bool) -> Self {
        self.params.insert("biasdriver".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_biasdriver_expr(mut self, expr: &str) -> Self {
        self.params.insert("biasdriver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_contractiondriver(mut self, val: bool) -> Self {
        self.params.insert("contractiondriver".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_contractiondriver_expr(mut self, expr: &str) -> Self {
        self.params.insert("contractiondriver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessdriver(mut self, val: bool) -> Self {
        self.params.insert("tightnessdriver".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tightnessdriver_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessdriver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p2display(mut self, val: bool) -> Self {
        self.params.insert("p2display".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_p2display_expr(mut self, expr: &str) -> Self {
        self.params.insert("p2display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keepposhandles(mut self, val: bool) -> Self {
        self.params.insert("keepposhandles".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepposhandles_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepposhandles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displaycapturepose(mut self, val: bool) -> Self {
        self.params.insert("displaycapturepose".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displaycapturepose_expr(mut self, expr: &str) -> Self {
        self.params.insert("displaycapturepose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usejiggle(mut self, val: bool) -> Self {
        self.params.insert("usejiggle".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usejiggle_expr(mut self, expr: &str) -> Self {
        self.params.insert("usejiggle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usedynamics(mut self, val: bool) -> Self {
        self.params.insert("usedynamics".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedynamics_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedynamics".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayanchorradius(mut self, val: bool) -> Self {
        self.params.insert("displayanchorradius".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayanchorradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayanchorradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayhandles(mut self, val: bool) -> Self {
        self.params.insert("displayhandles".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayhandles_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayhandles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayinnercore(mut self, val: bool) -> Self {
        self.params.insert("displayinnercore".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayinnercore_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayinnercore".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayaswires(mut self, val: bool) -> Self {
        self.params.insert("displayaswires".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayaswires_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayaswires".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displaysinglewire(mut self, val: bool) -> Self {
        self.params.insert("displaysinglewire".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displaysinglewire_expr(mut self, expr: &str) -> Self {
        self.params.insert("displaysinglewire".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectMusclerig {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "musclerig"
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
pub trait ObjectMusclerigInnerExt {
    fn tightnessdriver(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_handle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_handle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_jiggle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_jiggle_handle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_jiggle_handle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_tight1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_tight2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_tight3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn capture_handle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn capture_handle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn capture_handle3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn capture_handle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn capture_handle5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cv1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cv2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cv3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cv4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cv5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn handle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn handle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn handle3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn handle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn handle5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_radius(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn internal_core(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn jiggle_chop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn jiggled_handle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn jiggled_handle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn jiggled_null(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn stroke_container(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_to_capture_pose(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_to_capture_pose1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_to_capture_pose2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_to_capture_pose3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_to_capture_pose4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn tailradius(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unjiggled_handle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unjiggled_handle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unjiggled_null(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ObjectMusclerigInnerExt for crate::core::graph::InnerGraph<'a> {
    fn tightnessdriver(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("TightnessDriver")
    }
    fn blend_handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_handle1")
    }
    fn blend_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_handle2")
    }
    fn blend_jiggle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_jiggle")
    }
    fn blend_jiggle_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_jiggle_handle2")
    }
    fn blend_jiggle_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_jiggle_handle4")
    }
    fn blend_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_middle")
    }
    fn blend_tight1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_tight1")
    }
    fn blend_tight2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_tight2")
    }
    fn blend_tight3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend_tight3")
    }
    fn capture_handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("capture_handle1")
    }
    fn capture_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("capture_handle2")
    }
    fn capture_handle3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("capture_handle3")
    }
    fn capture_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("capture_handle4")
    }
    fn capture_handle5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("capture_handle5")
    }
    fn cv1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cv1")
    }
    fn cv2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cv2")
    }
    fn cv3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cv3")
    }
    fn cv4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cv4")
    }
    fn cv5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cv5")
    }
    fn handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("handle1")
    }
    fn handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("handle2")
    }
    fn handle3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("handle3")
    }
    fn handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("handle4")
    }
    fn handle5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("handle5")
    }
    fn head_radius(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("head_radius")
    }
    fn internal_core(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("internal_core")
    }
    fn jiggle_chop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("jiggle_chop")
    }
    fn jiggled_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("jiggled_handle2")
    }
    fn jiggled_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("jiggled_handle4")
    }
    fn jiggled_null(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("jiggled_null")
    }
    fn stroke_container(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("stroke_container")
    }
    fn switch_to_capture_pose(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_to_capture_pose")
    }
    fn switch_to_capture_pose1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_to_capture_pose1")
    }
    fn switch_to_capture_pose2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_to_capture_pose2")
    }
    fn switch_to_capture_pose3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_to_capture_pose3")
    }
    fn switch_to_capture_pose4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_to_capture_pose4")
    }
    fn tailradius(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("tailradius")
    }
    fn unjiggled_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("unjiggled_handle2")
    }
    fn unjiggled_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("unjiggled_handle4")
    }
    fn unjiggled_null(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("unjiggled_null")
    }
}
