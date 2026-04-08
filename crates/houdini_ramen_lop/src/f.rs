#[derive(Debug, Clone)]
pub struct LopFetch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopFetch {
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

    // --- String parameters ---
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopFetch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fetch"
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
pub enum LopFilecacheFilemethod {
    Constructed = 0,
    Explicit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFilecacheFiletype {
    Usd = 0,
    Usda = 1,
    Usdc = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFilecacheTrange {
    SingleFrame = 0,
    FrameRange = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFilecacheFmenu {
    AnimationRange = 0,
    PlaybarRange = 1,
    NoSubsteps = 2,
    /// 2 Substeps
    N2Substeps = 3,
    /// 4 Substeps
    N4Substeps = 4,
    /// 5 Substeps
    N5Substeps = 5,
    Every2ndFrame = 6,
    Every5thFrame = 7,
    Every10thFrame = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFilecacheMissingfile {
    ReportError = 0,
    ReportWarning = 1,
    IgnoreFile = 2,
}

#[derive(Debug, Clone)]
pub struct LopFilecache {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopFilecache {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params.insert(
            "reload".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_openfiledir(mut self) -> Self {
        self.params.insert(
            "openfiledir".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_openbasedir(mut self) -> Self {
        self.params.insert(
            "openbasedir".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookoutputnode(mut self) -> Self {
        self.params.insert(
            "cookoutputnode".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtyall(mut self) -> Self {
        self.params.insert(
            "dirtyall".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_frameoverride(mut self, val: f32) -> Self {
        self.params.insert(
            "frameoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frameoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frameoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_version(mut self, val: i32) -> Self {
        self.params.insert(
            "version".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_version_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "version".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_saveretry(mut self, val: i32) -> Self {
        self.params.insert(
            "saveretry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_saveretry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "saveretry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadretry(mut self, val: i32) -> Self {
        self.params.insert(
            "loadretry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_loadretry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadretry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_filemethod(mut self, val: LopFilecacheFilemethod) -> Self {
        self.params.insert(
            "filemethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filemethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filetype(mut self, val: LopFilecacheFiletype) -> Self {
        self.params.insert(
            "filetype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trange(mut self, val: LopFilecacheTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fmenu(mut self, val: LopFilecacheFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_missingfile(mut self, val: LopFilecacheMissingfile) -> Self {
        self.params.insert(
            "missingfile".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missingfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_basename(mut self, val: &str) -> Self {
        self.params.insert(
            "basename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basedir(mut self, val: &str) -> Self {
        self.params.insert(
            "basedir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basedir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specificrange(mut self, val: &str) -> Self {
        self.params.insert(
            "specificrange".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specificrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specificrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettopnetwork(mut self, val: &str) -> Self {
        self.params.insert(
            "targettopnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettopnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettopnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_versionstr(mut self, val: &str) -> Self {
        self.params.insert(
            "versionstr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_versionstr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "versionstr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachedir(mut self, val: &str) -> Self {
        self.params.insert(
            "cachedir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachedir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachename(mut self, val: &str) -> Self {
        self.params.insert(
            "cachename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_descriptivelabel(mut self, val: &str) -> Self {
        self.params.insert(
            "descriptivelabel".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_descriptivelabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "descriptivelabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "loadfromdisk".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadfromdisk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableversion(mut self, val: bool) -> Self {
        self.params.insert(
            "enableversion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useframeoverride(mut self, val: bool) -> Self {
        self.params.insert(
            "useframeoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useframeoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useframeoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fileperframe(mut self, val: bool) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fileperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths(mut self, val: bool) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_simplerelativepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_simplerelativepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableoutputprocessor_savetodirectory(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_savetodirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_savetodirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_savetodirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableoutputprocessor_savepathsrelativetooutput(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_savepathsrelativetooutput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_savepathsrelativetooutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_savepathsrelativetooutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableoutputprocessor_usesearchpaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_usesearchpaths".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_usesearchpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_usesearchpaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableoutputprocessor_matchoutputextension(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_matchoutputextension".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_matchoutputextension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_matchoutputextension".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadfromdiskonsave(mut self, val: bool) -> Self {
        self.params.insert(
            "loadfromdiskonsave".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdiskonsave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadfromdiskonsave".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hardenbasename(mut self, val: bool) -> Self {
        self.params.insert(
            "hardenbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hardenbasename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hardenbasename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savebackground(mut self, val: bool) -> Self {
        self.params.insert(
            "savebackground".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebackground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savebackground".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savebackuphip(mut self, val: bool) -> Self {
        self.params.insert(
            "savebackuphip".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebackuphip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savebackuphip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savethumbnails(mut self, val: bool) -> Self {
        self.params.insert(
            "savethumbnails".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savethumbnails_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savethumbnails".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefetch(mut self, val: bool) -> Self {
        self.params.insert(
            "prefetch".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefetch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefetch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopFilecache {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "filecache"
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
pub enum LopFollowpathconstraintSourcetype {
    Primitives = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintTargetsource {
    FirstInput = 0,
    SecondInput = 1,
    SopPath = 2,
    DrawCurve = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintTargettype {
    Primitive = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintUparmtype {
    NormalizedDistance = 0,
    NormalizedSpline = 1,
    DistanceFromStart = 2,
    DistanceFromEnd = 3,
    DistancePointAttribute = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintLookatmode {
    None = 0,
    AlongPath = 1,
    DirectionAttributeFromPath = 2,
    FromPrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintLookatsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintLookupmode {
    UpVectorAttributeFromPath = 0,
    AlongPath = 1,
    XAxis = 2,
    YAxis = 3,
    ZAxis = 4,
    FromPrimitive = 5,
    Custom = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintUpvectorsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintLookataxis {
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
pub enum LopFollowpathconstraintLookupaxisx {
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
pub enum LopFollowpathconstraintLookupaxisy {
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
pub enum LopFollowpathconstraintLookupaxisz {
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
pub enum LopFollowpathconstraintOutputtype {
    Polygon = 0,
    NurbsCurve = 1,
    BezierCurve = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintMode {
    /// Select/Edit Mode
    SelectEditMode = 0,
    DrawMode = 1,
    /// Auto-Bezier Draw Mode
    AutoMinusBezierDrawMode = 2,
    OrientMode = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintInterpmethod {
    SmoothCurvature = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintTangenttype {
    AverageOfEdgeDirections = 0,
    CentralDifference = 1,
    PreviousEdge = 2,
    NextEdge = 3,
    /// Z Axis (Ignore Curve)
    ZAxisIgnoreCurve = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopFollowpathconstraintOptype {
    AppendPoint = 0,
    Balance = 1,
    Close = 2,
    Contract = 3,
    Corner = 4,
    Cut = 5,
    Delete = 6,
    Expand = 7,
    Fuse = 8,
    Insert = 9,
    Join = 10,
    PrependPoint = 11,
    Pull = 12,
    SegmentContract = 13,
    SegmentDelete = 14,
    Smooth = 15,
    PointTransform = 16,
    Reverse = 17,
    BezierHandleMove = 18,
    BakeRoundCorners = 19,
    RoundCornersRadius = 20,
    RoundCornersSplit = 21,
    RoundCornersFuse = 22,
    EvenlySpace = 23,
    StraightenSelection = 24,
    SpaceOnCircle = 25,
    Flatten = 26,
    RelaxSelection = 27,
    NoOp = 28,
    Auto = 29,
    Manual = 30,
    Convert = 31,
    Paste = 32,
    CutBranch = 33,
    FuseBranch = 34,
    ConvertThroughHulls = 35,
}

#[derive(Debug, Clone)]
pub struct LopFollowpathconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopFollowpathconstraint {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Stage Containing Target Primitives (optional)"
    pub fn set_input_stage_containing_target_primitives_optio<
        N: houdini_ramen_core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Stage Containing Target Primitives (optional)" and specifies the output index of the target node.
    pub fn set_input_stage_containing_target_primitives_optio_from<
        N: houdini_ramen_core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_delete(mut self) -> Self {
        self.params.insert(
            "delete".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_fuse(mut self) -> Self {
        self.params.insert(
            "fuse".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cut(mut self) -> Self {
        self.params.insert(
            "cut".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_join(mut self) -> Self {
        self.params.insert(
            "join".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_deleteseg(mut self) -> Self {
        self.params.insert(
            "deleteseg".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_fusebranch(mut self) -> Self {
        self.params.insert(
            "fusebranch".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_splitbranch(mut self) -> Self {
        self.params.insert(
            "splitbranch".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_close(mut self) -> Self {
        self.params.insert(
            "close".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_createroundedcorners(mut self) -> Self {
        self.params.insert(
            "createroundedcorners".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_removeroundedcorners(mut self) -> Self {
        self.params.insert(
            "removeroundedcorners".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reverse(mut self) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_makecorner(mut self) -> Self {
        self.params.insert(
            "makecorner".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_makesmooth(mut self) -> Self {
        self.params.insert(
            "makesmooth".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_makebalanced(mut self) -> Self {
        self.params.insert(
            "makebalanced".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_contract(mut self) -> Self {
        self.params.insert(
            "contract".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_expand(mut self) -> Self {
        self.params.insert(
            "expand".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_straightenseg(mut self) -> Self {
        self.params.insert(
            "straightenseg".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_makeauto(mut self) -> Self {
        self.params.insert(
            "makeauto".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_makemanual(mut self) -> Self {
        self.params.insert(
            "makemanual".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reset(mut self) -> Self {
        self.params.insert(
            "reset".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_bakeroundcorners(mut self) -> Self {
        self.params.insert(
            "bakeroundcorners".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_stashgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "stashgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Data(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stashgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stashgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savedstashgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "savedstashgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Data(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savedstashgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savedstashgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "parmpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Data(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_importtime(mut self, val: f32) -> Self {
        self.params.insert(
            "importtime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_importtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_softtransformrad(mut self, val: f32) -> Self {
        self.params.insert(
            "softtransformrad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softtransformrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softtransformrad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundcornerradius(mut self, val: f32) -> Self {
        self.params.insert(
            "roundcornerradius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roundcornerradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundcornerradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upvector".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvector".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetup_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("targetup{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_targetup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("targetup{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettangent_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("targettangent{}", index1),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_targettangent_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("targettangent{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_translate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "translate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_translate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pivottranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pivottranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pivottranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivottranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pivotrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pivotrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pivotrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivotrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_planeorig(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "planeorig".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_planeorig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planeorig".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_planenml(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "planenml".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_planenml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planenml".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlep0pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handlep0pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handlep0pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlep0pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlep1pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handlep1pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handlep1pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlep1pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlep2pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handlep2pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handlep2pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlep2pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcetype(mut self, val: LopFollowpathconstraintSourcetype) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourcetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetsource(mut self, val: LopFollowpathconstraintTargetsource) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targetsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettype(mut self, val: LopFollowpathconstraintTargettype) -> Self {
        self.params.insert(
            "targettype".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatmode(mut self, val: LopFollowpathconstraintLookatmode) -> Self {
        self.params.insert(
            "lookatmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_lookatmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatsource(mut self, val: LopFollowpathconstraintLookatsource) -> Self {
        self.params.insert(
            "lookatsource".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_lookatsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupmode(mut self, val: LopFollowpathconstraintLookupmode) -> Self {
        self.params.insert(
            "lookupmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_lookupmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvectorsource(mut self, val: LopFollowpathconstraintUpvectorsource) -> Self {
        self.params.insert(
            "upvectorsource".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_upvectorsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_order(mut self, val: i32) -> Self {
        self.params.insert(
            "order".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_order_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "order".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ninputprimscached(mut self, val: i32) -> Self {
        self.params.insert(
            "ninputprimscached".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ninputprimscached_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ninputprimscached".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetptnum_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("targetptnum{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_targetptnum_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("targetptnum{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activeprim(mut self, val: i32) -> Self {
        self.params.insert(
            "activeprim".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activeprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_handlepts(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "handlepts".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_handlepts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlepts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_uparmtype(mut self, val: LopFollowpathconstraintUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookataxis(mut self, val: LopFollowpathconstraintLookataxis) -> Self {
        self.params.insert(
            "lookataxis".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookataxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: LopFollowpathconstraintLookupaxisx) -> Self {
        self.params.insert(
            "lookupaxisx".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: LopFollowpathconstraintLookupaxisy) -> Self {
        self.params.insert(
            "lookupaxisy".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: LopFollowpathconstraintLookupaxisz) -> Self {
        self.params.insert(
            "lookupaxisz".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtype(mut self, val: LopFollowpathconstraintOutputtype) -> Self {
        self.params.insert(
            "outputtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: LopFollowpathconstraintMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interpmethod(mut self, val: LopFollowpathconstraintInterpmethod) -> Self {
        self.params.insert(
            "interpmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interpmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tangenttype(mut self, val: LopFollowpathconstraintTangenttype) -> Self {
        self.params.insert(
            "tangenttype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tangenttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangenttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optype(mut self, val: LopFollowpathconstraintOptype) -> Self {
        self.params.insert(
            "optype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_optype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "optype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceinstances(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceinstances".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceinstances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceinstances".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetinstance(mut self, val: &str) -> Self {
        self.params.insert(
            "targetinstance".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetinstance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetinstance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "distattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positionsnippet(mut self, val: &str) -> Self {
        self.params.insert(
            "positionsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positionsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatxform(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatxform".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatxform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvectorxform(mut self, val: &str) -> Self {
        self.params.insert(
            "upvectorxform".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvectorxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorxform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dirattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "dirattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dirattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "upattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rollsnippet(mut self, val: &str) -> Self {
        self.params.insert(
            "rollsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rollsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stashid(mut self, val: &str) -> Self {
        self.params.insert(
            "stashid".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stashid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stashid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xaxisname(mut self, val: &str) -> Self {
        self.params.insert(
            "xaxisname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xaxisname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xaxisname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yaxisname(mut self, val: &str) -> Self {
        self.params.insert(
            "yaxisname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yaxisname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yaxisname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zaxisname(mut self, val: &str) -> Self {
        self.params.insert(
            "zaxisname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zaxisname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zaxisname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientname(mut self, val: &str) -> Self {
        self.params.insert(
            "orientname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activepoints(mut self, val: &str) -> Self {
        self.params.insert(
            "activepoints".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activepoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addpts(mut self, val: &str) -> Self {
        self.params.insert(
            "addpts".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cornerpts(mut self, val: &str) -> Self {
        self.params.insert(
            "cornerpts".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cornerpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cornerpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_smoothpts(mut self, val: &str) -> Self {
        self.params.insert(
            "smoothpts".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_smoothpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smoothpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autopts(mut self, val: &str) -> Self {
        self.params.insert(
            "autopts".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autopts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundcornerpts(mut self, val: &str) -> Self {
        self.params.insert(
            "roundcornerpts".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundcornerpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundcornerpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hidetarget(mut self, val: bool) -> Self {
        self.params.insert(
            "hidetarget".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hidetarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hidetarget".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepoffset(mut self, val: bool) -> Self {
        self.params.insert(
            "keepoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doposition(mut self, val: bool) -> Self {
        self.params.insert(
            "doposition".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doposition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dorotation(mut self, val: bool) -> Self {
        self.params.insert(
            "dorotation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doscale(mut self, val: bool) -> Self {
        self.params.insert(
            "doscale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doshear(mut self, val: bool) -> Self {
        self.params.insert(
            "doshear".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doshear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doshear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clamp(mut self, val: bool) -> Self {
        self.params.insert(
            "clamp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepositionsnippet(mut self, val: bool) -> Self {
        self.params.insert(
            "usepositionsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepositionsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepositionsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_userollsnippet(mut self, val: bool) -> Self {
        self.params.insert(
            "userollsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userollsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userollsnippet".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_convertcurves(mut self, val: bool) -> Self {
        self.params.insert(
            "convertcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablebezierops(mut self, val: bool) -> Self {
        self.params.insert(
            "enablebezierops".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablebezierops_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablebezierops".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_snaponclose(mut self, val: bool) -> Self {
        self.params.insert(
            "snaponclose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_snaponclose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snaponclose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maintainpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "maintainpoint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maintainpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maintainpoint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_approxendtangents(mut self, val: bool) -> Self {
        self.params.insert(
            "approxendtangents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_approxendtangents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "approxendtangents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_softtransform(mut self, val: bool) -> Self {
        self.params.insert(
            "softtransform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_softtransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softtransform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputcornerpts(mut self, val: bool) -> Self {
        self.params.insert(
            "outputcornerpts".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcornerpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcornerpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputsmoothpts(mut self, val: bool) -> Self {
        self.params.insert(
            "outputsmoothpts".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputsmoothpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputsmoothpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputautopts(mut self, val: bool) -> Self {
        self.params.insert(
            "outputautopts".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputautopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputautopts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputname(mut self, val: bool) -> Self {
        self.params.insert(
            "outputname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputxaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "outputxaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputxaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputxaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputyaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "outputyaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputyaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputyaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputzaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "outputzaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputzaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputzaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputorient(mut self, val: bool) -> Self {
        self.params.insert(
            "outputorient".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aligntangent(mut self, val: bool) -> Self {
        self.params.insert(
            "aligntangent".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aligntangent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aligntangent".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabletargetpt_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enabletargetpt{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpt_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enabletargetpt{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pivotfixed(mut self, val: bool) -> Self {
        self.params.insert(
            "pivotfixed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pivotfixed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivotfixed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewroundcorners(mut self, val: bool) -> Self {
        self.params.insert(
            "viewroundcorners".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viewroundcorners_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewroundcorners".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showroundcornerwidget(mut self, val: bool) -> Self {
        self.params.insert(
            "showroundcornerwidget".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showroundcornerwidget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showroundcornerwidget".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showroundcornerlabels(mut self, val: bool) -> Self {
        self.params.insert(
            "showroundcornerlabels".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showroundcornerlabels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showroundcornerlabels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrainstraighten(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainstraighten".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainstraighten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainstraighten".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopFollowpathconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "followpathconstraint"
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
pub struct LopForeach {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopForeach {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Possible Iteration Data"
    pub fn set_input_possible_iteration_data<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Possible Iteration Data" and specifies the output index of the target node.
    pub fn set_input_possible_iteration_data_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Iterate On This Input"
    pub fn set_input_iterate_on_this_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Iterate On This Input" and specifies the output index of the target node.
    pub fn set_input_iterate_on_this_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_firstiteration(mut self, val: i32) -> Self {
        self.params.insert(
            "firstiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_firstiteration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "firstiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lastiteration(mut self, val: i32) -> Self {
        self.params.insert(
            "lastiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lastiteration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lastiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_itermethod(mut self, val: &str) -> Self {
        self.params.insert(
            "itermethod".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_itermethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "itermethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset(mut self, val: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strings(mut self, val: &str) -> Self {
        self.params.insert(
            "strings".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strings".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_combineiterations(mut self, val: &str) -> Self {
        self.params.insert(
            "combineiterations".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_combineiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterationname(mut self, val: &str) -> Self {
        self.params.insert(
            "iterationname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterationname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterationname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterationcountname(mut self, val: &str) -> Self {
        self.params.insert(
            "iterationcountname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterationcountname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterationcountname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_valuename(mut self, val: &str) -> Self {
        self.params.insert(
            "valuename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_valuename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valuename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterrange(mut self, val: &str) -> Self {
        self.params.insert(
            "iterrange".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allowinstanceproxies(mut self, val: bool) -> Self {
        self.params.insert(
            "allowinstanceproxies".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowinstanceproxies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowinstanceproxies".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lastmodifiedfromiteration(mut self, val: bool) -> Self {
        self.params.insert(
            "lastmodifiedfromiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lastmodifiedfromiteration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lastmodifiedfromiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeloadmasks(mut self, val: bool) -> Self {
        self.params.insert(
            "mergeloadmasks".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mergeloadmasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeloadmasks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopForeach {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "foreach"
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
