#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverUsdTrange {
    RenderCurrentFrame = 0,
    RenderSpecificFrameRange = 1,
    /// Render Specific Frame Range Only (Strict)
    RenderSpecificFrameRangeOnlyStrict = 2,
    RenderFrameRangeFromStage = 3,
}

#[derive(Debug, Clone)]
pub struct DriverUsd {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverUsd {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_filtertimesamplespadding(mut self, val: f32) -> Self {
        self.params.insert(
            "filtertimesamplespadding".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filtertimesamplespadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filtertimesamplespadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_foffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_foffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverUsdTrange) -> Self {
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
    pub fn with_outputprocessors(mut self, val: i32) -> Self {
        self.params.insert(
            "outputprocessors".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_outputprocessors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputprocessors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filtertimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "filtertimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filtertimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filtertimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "savepattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultprim(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strippostlayers(mut self, val: bool) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strippostlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fileperframe(mut self, val: bool) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fileperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trackprimexistence(mut self, val: bool) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trackprimexistence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenetworksafesave(mut self, val: bool) -> Self {
        self.params.insert(
            "usenetworksafesave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenetworksafesave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenetworksafesave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths(mut self, val: bool) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefilesfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "savefilesfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savefilesfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefilesfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenfilelayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flattenfilelayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenfilelayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenfilelayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattensoplayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flattensoplayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattensoplayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattensoplayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputprocessor_removehfs(mut self, val: bool) -> Self {
        self.params.insert(
            "outputprocessor_removehfs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputprocessor_removehfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputprocessor_removehfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requiredefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiredefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetimeinfo(mut self, val: bool) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savetimeinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearhoudinicustomdata(mut self, val: bool) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearhoudinicustomdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ensuremetricsset(mut self, val: bool) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ensuremetricsset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setropcook(mut self, val: bool) -> Self {
        self.params.insert(
            "setropcook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setropcook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setropcook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DriverUsd {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usd"
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
pub enum DriverUsdrenderTrange {
    RenderCurrentFrame = 0,
    RenderSpecificFrameRange = 1,
    /// Render Specific Frame Range Only (Strict)
    RenderSpecificFrameRangeOnlyStrict = 2,
    RenderFrameRangeFromStage = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverUsdrenderResScaleMenu {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverUsdrenderHuskMplayzoommenu {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverUsdrenderVerboseMenu {
    None = 0,
    RenderingStatistics = 1,
    FullVerbosity = 2,
    FullWithVexProfiling = 3,
    /// Full with VEX Profiling and NAN Checks (Slow)
    FullWithVexProfilingAndNanChecksSlow = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverUsdrenderVexprofile {
    NoVexProfiling = 0,
    ExecutionProfiling = 1,
    /// Profiling and NAN Detection (Slow)
    ProfilingAndNanDetectionSlow = 2,
}

#[derive(Debug, Clone)]
pub struct DriverUsdrender {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverUsdrender {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert(
            "renderpreview".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_husk_tex_mempct(mut self, val: f32) -> Self {
        self.params.insert(
            "husk_tex_mempct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_husk_tex_mempct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_mempct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filespct(mut self, val: f32) -> Self {
        self.params.insert(
            "husk_tex_filespct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_husk_tex_filespct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_filespct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filesreserve(mut self, val: f32) -> Self {
        self.params.insert(
            "husk_tex_filesreserve".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_husk_tex_filesreserve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_filesreserve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filtertimesamplespadding(mut self, val: f32) -> Self {
        self.params.insert(
            "filtertimesamplespadding".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filtertimesamplespadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filtertimesamplespadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_foffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_foffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
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
    pub fn with_husk_restartdelegateframes(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_restartdelegateframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_restartdelegateframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_restartdelegateframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snapshotinterval(mut self, val: i32) -> Self {
        self.params.insert(
            "snapshotinterval".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_snapshotinterval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snapshotinterval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_timelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_timelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_timelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_timelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayzoom(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_mplayzoom".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_mplayzoom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayzoom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_mem(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tex_mem".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tex_mem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_mem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_files(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tex_files".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tex_files_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_files".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tileindex(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tileindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tileindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tileindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: i32) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vexprofile(mut self, val: DriverUsdrenderVexprofile) -> Self {
        self.params.insert(
            "vexprofile".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vexprofile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexprofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "maxthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxthreads".to_string(),
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
    pub fn with_husk_tilemaxres(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "husk_tilemaxres".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_husk_tilemaxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilemaxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilecount(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "husk_tilecount".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_husk_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverUsdrenderTrange) -> Self {
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
    pub fn with_res_scale_menu(mut self, val: DriverUsdrenderResScaleMenu) -> Self {
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
    pub fn with_res_usermenu(mut self, val: i32) -> Self {
        self.params.insert(
            "res_userMenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_res_usermenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_userMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayzoommenu(mut self, val: DriverUsdrenderHuskMplayzoommenu) -> Self {
        self.params.insert(
            "husk_mplayzoommenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_husk_mplayzoommenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayzoommenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose_menu(mut self, val: DriverUsdrenderVerboseMenu) -> Self {
        self.params.insert(
            "verbose_menu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_verbose_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose_menu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputprocessors(mut self, val: i32) -> Self {
        self.params.insert(
            "outputprocessors".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_outputprocessors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputprocessors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_renderer(mut self, val: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
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
    pub fn with_outputimage(mut self, val: &str) -> Self {
        self.params.insert(
            "outputimage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputimage".to_string(),
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
    pub fn with_husk_headlight(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_headlight".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_populationmask(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_populationmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_populationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_populationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_complexity(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_complexity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_complexity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_complexity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_key(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_metadata_key".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_key_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_metadata_key".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_value(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_metadata_value".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_metadata_value".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayname(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_mplayname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_mplayname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayaovs(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_mplayaovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_mplayaovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayaovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_res(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tex_res".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tex_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_memmode(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tex_memmode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tex_memmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_memmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filemode(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tex_filemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_filemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_presnapshot(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_presnapshot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_presnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_presnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_postsnapshot(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_postsnapshot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_postsnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_postsnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tiletempdir(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tiletempdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tiletempdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tiletempdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilesuffix(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tilesuffix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tilesuffix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilesuffix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windowsconsole(mut self, val: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_windowsconsole_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_usdtrace(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_usdtrace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_usdtrace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_usdtrace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_chromefile(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_chromefile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_chromefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_chromefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_stdout(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_stdout".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_stdout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_stdout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_stderr(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_stderr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_stderr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_stderr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_source_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_source{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_source_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_source{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_file_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_file{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_file_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_file{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_cop_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_cop{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_cop_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_cop{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_filterlist_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_filterlist{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_filterlist_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_filterlist{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_label_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_label{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_label_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_label{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_aov_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_aov{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_aov_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_aov{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_cop_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_cop{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_cop_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_cop{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_cop_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_cop{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_cop_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_cop{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_aov_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_aov{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_aov_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_aov{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendercommand(mut self, val: &str) -> Self {
        self.params.insert(
            "rendercommand".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendercommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendercommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderexisting(mut self, val: &str) -> Self {
        self.params.insert(
            "renderexisting".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderexisting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderexisting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontext(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontext".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontext_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontext".to_string(),
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
    pub fn with_lopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletefiles(mut self, val: &str) -> Self {
        self.params.insert(
            "deletefiles".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deletefiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deletefiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filtertimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "filtertimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filtertimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filtertimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "savepattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetodirectory_directory(mut self, val: &str) -> Self {
        self.params.insert(
            "savetodirectory_directory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savetodirectory_directory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetodirectory_directory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultprim(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allframesatonce(mut self, val: bool) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allframesatonce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_gpu(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_gpu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_gpu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_gpu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputimageshowwarning(mut self, val: bool) -> Self {
        self.params.insert(
            "outputimageshowwarning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputimageshowwarning_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputimageshowwarning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_instantshutter(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_instantshutter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_instantshutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_instantshutter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_enable_headlight(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_enable_headlight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_enable_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_enable_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_dopopulationmask(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_dopopulationmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_dopopulationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_dopopulationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_rendersettingsandcams(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_rendersettingsandcams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_rendersettingsandcams_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_rendersettingsandcams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_legacyexr(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_legacyexr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_legacyexr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_legacyexr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_delegateprod(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_delegateprod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_delegateprod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_delegateprod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_restartdelegate(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_restartdelegate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_restartdelegate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_restartdelegate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "dosnapshot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_dotimelimit(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_dotimelimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_dotimelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_dotimelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_timelimitperimage(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_timelimitperimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_timelimitperimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_timelimitperimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplay(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_mplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_mplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpresnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpresnapshot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpresnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpresnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpostsnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpostsnapshot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpostsnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpostsnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tile(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_autotile(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_autotile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_autotile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_autotile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_debug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_debug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_log(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_log".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_log_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_log".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_logappend(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_logappend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_logappend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_logappend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("husk_sc_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_sc_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_runcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "runcommand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_runcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "runcommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorenderexisting(mut self, val: bool) -> Self {
        self.params.insert(
            "dorenderexisting".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorenderexisting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorenderexisting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderexistingandsaveusd(mut self, val: bool) -> Self {
        self.params.insert(
            "renderexistingandsaveusd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderexistingandsaveusd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderexistingandsaveusd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_soho_foreground(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_foreground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
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
    pub fn with_domaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxthreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strippostlayers(mut self, val: bool) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strippostlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trackprimexistence(mut self, val: bool) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trackprimexistence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenetworksafesave(mut self, val: bool) -> Self {
        self.params.insert(
            "usenetworksafesave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenetworksafesave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenetworksafesave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths(mut self, val: bool) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefilesfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "savefilesfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savefilesfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefilesfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenfilelayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flattenfilelayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenfilelayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenfilelayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattensoplayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flattensoplayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattensoplayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattensoplayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputprocessor_removehfs(mut self, val: bool) -> Self {
        self.params.insert(
            "outputprocessor_removehfs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputprocessor_removehfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputprocessor_removehfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputprocessor_savetodirectory(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_savetodirectory".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_savetodirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_savetodirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_simplerelativepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_simplerelativepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputprocessor_matchoutputextension(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_matchoutputextension".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_matchoutputextension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_matchoutputextension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requiredefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiredefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetimeinfo(mut self, val: bool) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savetimeinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearhoudinicustomdata(mut self, val: bool) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearhoudinicustomdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ensuremetricsset(mut self, val: bool) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ensuremetricsset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setropcook(mut self, val: bool) -> Self {
        self.params.insert(
            "setropcook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setropcook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setropcook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DriverUsdrender {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdrender"
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
pub struct DriverUsdstitch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverUsdstitch {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- String parameters ---
    pub fn with_infile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("infile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_infile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("infile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outfile{}", index1),
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
    pub fn with_pdg_logoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_logoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_logoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_logoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DriverUsdstitch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdstitch"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait DriverUsdstitchInnerExt {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DriverUsdstitchInnerExt for crate::core::graph::InnerGraph<'a> {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT")
    }
}

#[derive(Debug, Clone)]
pub struct DriverUsdstitchclips {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverUsdstitchclips {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_tcps(mut self, val: f32) -> Self {
        self.params.insert(
            "tcps".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tcps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tcps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- String parameters ---
    pub fn with_settcps(mut self, val: &str) -> Self {
        self.params.insert(
            "settcps".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_settcps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "settcps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_infile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("infile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_infile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("infile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outtemplatefile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outtemplatefile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outtemplatefile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outtemplatefile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clippath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clippath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clippath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clippath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clipset{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipset{}", index1),
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
    pub fn with_pdg_logoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_logoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_logoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_logoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DriverUsdstitchclips {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdstitchclips"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait DriverUsdstitchclipsInnerExt {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DriverUsdstitchclipsInnerExt for crate::core::graph::InnerGraph<'a> {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT")
    }
}

#[derive(Debug, Clone)]
pub struct DriverUsdzip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverUsdzip {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_infile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("infile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_infile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("infile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outfile{}", index1),
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
    pub fn with_arkitasset_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("arkitasset{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_arkitasset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("arkitasset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DriverUsdzip {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdzip"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait DriverUsdzipInnerExt {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DriverUsdzipInnerExt for crate::core::graph::InnerGraph<'a> {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT")
    }
}
