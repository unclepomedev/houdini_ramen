#[derive(Debug, Clone)]
pub struct DriverFetch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverFetch {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
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
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverFetch {
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
pub enum DriverFilmboxfbxTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFilmboxfbxVcformat {
    /// Maya Compatible (MC)
    MayaCompatibleMc = 0,
    /// 3DS MAX Compatible (PC2)
    N3dsMaxCompatiblePc2 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFilmboxfbxInvisobj {
    AsHiddenNullNodes = 0,
    AsHiddenFullNodes = 1,
    AsVisibleFullNodes = 2,
    /// Don't Export
    DonTExport = 3,
}

#[derive(Debug, Clone)]
pub struct DriverFilmboxfbx {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverFilmboxfbx {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
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
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_polylod(mut self, val: f32) -> Self {
        self.params.insert(
            "polylod".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_polylod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polylod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_clipframerange_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.params.insert(
            format!("clipframerange{}", index1),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_clipframerange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipframerange{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverFilmboxfbxTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vcformat(mut self, val: DriverFilmboxfbxVcformat) -> Self {
        self.params.insert(
            "vcformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vcformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vcformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invisobj(mut self, val: DriverFilmboxfbxInvisobj) -> Self {
        self.params.insert(
            "invisobj".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_invisobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invisobj".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axissystem(mut self, val: i32) -> Self {
        self.params.insert(
            "axissystem".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_axissystem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axissystem".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startnode(mut self, val: &str) -> Self {
        self.params.insert(
            "startnode".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_startnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startnode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "sopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdkversion(mut self, val: &str) -> Self {
        self.params.insert(
            "sdkversion".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sdkversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdkversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clipname{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createsubnetroot(mut self, val: bool) -> Self {
        self.params.insert(
            "createsubnetroot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createsubnetroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createsubnetroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_buildfrompath(mut self, val: bool) -> Self {
        self.params.insert(
            "buildfrompath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_buildfrompath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buildfrompath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportkind(mut self, val: bool) -> Self {
        self.params.insert(
            "exportkind".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "convertaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertunits(mut self, val: bool) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detectconstpointobjs(mut self, val: bool) -> Self {
        self.params.insert(
            "detectconstpointobjs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_detectconstpointobjs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detectconstpointobjs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertsurfaces(mut self, val: bool) -> Self {
        self.params.insert(
            "convertsurfaces".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertsurfaces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertsurfaces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveshapenames(mut self, val: bool) -> Self {
        self.params.insert(
            "preserveshapenames".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveshapenames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preserveshapenames".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conservemem(mut self, val: bool) -> Self {
        self.params.insert(
            "conservemem".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_conservemem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conservemem".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformsasvcs(mut self, val: bool) -> Self {
        self.params.insert(
            "deformsasvcs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deformsasvcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deformsasvcs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forceblendshape(mut self, val: bool) -> Self {
        self.params.insert(
            "forceblendshape".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forceblendshape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forceblendshape".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forceskindeform(mut self, val: bool) -> Self {
        self.params.insert(
            "forceskindeform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forceskindeform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forceskindeform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportendeffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "exportendeffectors".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportendeffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportendeffectors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embedmedia(mut self, val: bool) -> Self {
        self.params.insert(
            "embedmedia".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_embedmedia_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "embedmedia".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computesmoothinggroups(mut self, val: bool) -> Self {
        self.params.insert(
            "computesmoothinggroups".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computesmoothinggroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computesmoothinggroups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportclips(mut self, val: bool) -> Self {
        self.params.insert(
            "exportclips".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportclips_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportclips".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverFilmboxfbx {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "filmboxfbx"
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
pub enum DriverFlipbookTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookOpsource {
    Objects = 0,
    Lops = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookSopsource {
    DisplayFlag = 0,
    RenderFlag = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookImagetype {
    ColorImage = 0,
    DepthImage = 1,
    /// 360' Cube map
    N360CubeMap = 2,
    TextureSheet = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookColorcorrect {
    None = 0,
    OpencolorioColorspace = 1,
    OpencolorioView = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookVmImageTiffCompression {
    NoCompression = 0,
    LzwCompression = 1,
    AdobeDeflate = 2,
    Packbits = 3,
    Lzma = 4,
    Zstandard = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookVmImageExrCompression {
    None = 0,
    Rle = 1,
    /// ZIP, Single scanline
    ZipSingleScanline = 2,
    /// ZIP, Multi-scanline blocks
    ZipMultiMinusScanlineBlocks = 3,
    PizWavelet = 4,
    /// PXR24 (32 bit float compression, lossy)
    Pxr2432BitFloatCompressionLossy = 5,
    /// B44 (4x4 block compression, lossy)
    B444x4BlockCompressionLossy = 6,
    /// B44A (4x4 block extra compression, lossy)
    B44a4x4BlockExtraCompressionLossy = 7,
    /// DWA-A (32-scanline block compression, lossy)
    DwaMinusA32MinusScanlineBlockCompressionLossy = 8,
    /// DWA-B (256-scanline block compression, lossy)
    DwaMinusB256MinusScanlineBlockCompressionLossy = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookShadingmode {
    Wireframe = 0,
    WireframeGhost = 1,
    HiddenLine = 2,
    HiddenLineGhost = 3,
    MatcapShaded = 4,
    MatcapWireShaded = 5,
    FlatShaded = 6,
    FlatWireShaded = 7,
    SmoothShaded = 8,
    SmoothWireShaded = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookLighting {
    Off = 0,
    WorkLights = 1,
    SceneLights = 2,
    SceneLightsWithShadows = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookWorklighttype {
    Headlight = 0,
    DomeLight = 1,
    PhysicalSky = 2,
    ThreePoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookShadowquality {
    PointShadows = 0,
    AntialiasedPointShadows = 1,
    AreaShadows = 2,
    AntialiasedAreaShadows = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookAamode {
    Off = 0,
    /// 2x AA
    N2xAa = 1,
    /// 4x AA
    N4xAa = 2,
    /// 8x AA
    N8xAa = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookUsehdr {
    /// HDR (16b FP)
    Hdr16bFp = 0,
    /// Full HDR (32b FP)
    FullHdr32bFp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookTransparency {
    None = 0,
    CutoutOnly = 1,
    Low = 2,
    Medium = 3,
    High = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookFogheightmode {
    Off = 0,
    Above = 1,
    Below = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookBokeh {
    None = 0,
    Circular = 1,
    FromFile = 2,
    FromCop = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookVolumequality {
    VeryLow = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookGennormal {
    PointNormals = 0,
    VertexNormals = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookParticle {
    Points = 0,
    Pixels = 1,
    Lines = 2,
    Discs = 3,
    LitSpheres = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookTex2dlimit {
    GpuLimit = 0,
    /// Auto-Detected Limit
    AutoMinusDetectedLimit = 1,
    SpecifyLimit = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookTex2dformat {
    /// 8b Fixed
    N8bFixed = 0,
    /// 16b FP
    N16bFp = 1,
    /// 32b FP
    N32bFp = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookTex3dlimit {
    GpuLimit = 0,
    /// Auto-Detected Limit
    AutoMinusDetectedLimit = 1,
    SpecifyLimit = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverFlipbookTex3dformat {
    /// 8b Fixed
    N8bFixed = 0,
    /// 16b FP
    N16bFp = 1,
    /// 32b FP
    N32bFp = 2,
}

#[derive(Debug, Clone)]
pub struct DriverFlipbook {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverFlipbook {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
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
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
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
    pub fn with_matcapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "matcapintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_matcapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matcapintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headlightint(mut self, val: f32) -> Self {
        self.params.insert(
            "headlightint".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headlightint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headlightint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headlightsoft(mut self, val: f32) -> Self {
        self.params.insert(
            "headlightsoft".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headlightsoft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headlightsoft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domelightintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "domelightintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_domelightintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelightintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domelightexposure(mut self, val: f32) -> Self {
        self.params.insert(
            "domelightexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_domelightexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelightexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyazimuth(mut self, val: f32) -> Self {
        self.params.insert(
            "skyazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyazimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyaltitude(mut self, val: f32) -> Self {
        self.params.insert(
            "skyaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyaltitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "skyintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyexposure(mut self, val: f32) -> Self {
        self.params.insert(
            "skyexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skysunintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "skysunintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skysunintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skysunintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skysunsize(mut self, val: f32) -> Self {
        self.params.insert(
            "skysunsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skysunsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skysunsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyskyintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "skyskyintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyskyintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyskyintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyturbidity(mut self, val: f32) -> Self {
        self.params.insert(
            "skyturbidity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyturbidity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyturbidity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyhorizonblur(mut self, val: f32) -> Self {
        self.params.insert(
            "skyhorizonblur".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skyhorizonblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyhorizonblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpkeyintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tpkeyintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpkeyintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpkeyintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpkeyexposure(mut self, val: f32) -> Self {
        self.params.insert(
            "tpkeyexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpkeyexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpkeyexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpkeyazimuth(mut self, val: f32) -> Self {
        self.params.insert(
            "tpkeyazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpkeyazimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpkeyazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpkeyaltitude(mut self, val: f32) -> Self {
        self.params.insert(
            "tpkeyaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpkeyaltitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpkeyaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpfillintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tpfillintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpfillintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpfillintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpfillexposure(mut self, val: f32) -> Self {
        self.params.insert(
            "tpfillexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpfillexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpfillexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpfillazimuth(mut self, val: f32) -> Self {
        self.params.insert(
            "tpfillazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpfillazimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpfillazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpfillaltitude(mut self, val: f32) -> Self {
        self.params.insert(
            "tpfillaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpfillaltitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpfillaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpbackintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tpbackintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpbackintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpbackintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpbackexposure(mut self, val: f32) -> Self {
        self.params.insert(
            "tpbackexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpbackexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpbackexposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpbackazimuth(mut self, val: f32) -> Self {
        self.params.insert(
            "tpbackazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpbackazimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpbackazimuth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpbackaltitude(mut self, val: f32) -> Self {
        self.params.insert(
            "tpbackaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tpbackaltitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpbackaltitude".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_occlweight(mut self, val: f32) -> Self {
        self.params.insert(
            "occlweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_occlweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_occldist(mut self, val: f32) -> Self {
        self.params.insert(
            "occldist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_occldist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occldist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_occlspread(mut self, val: f32) -> Self {
        self.params.insert(
            "occlspread".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_occlspread_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlspread".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmap(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowmap".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacequality(mut self, val: f32) -> Self {
        self.params.insert(
            "displacequality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacequality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "fogdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogopacity(mut self, val: f32) -> Self {
        self.params.insert(
            "fogopacity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogopacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogopacity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogclipdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "fogclipdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogclipdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogclipdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogheight(mut self, val: f32) -> Self {
        self.params.insert(
            "fogheight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogheight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogheightfalloff(mut self, val: f32) -> Self {
        self.params.insert(
            "fogheightfalloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogheightfalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogheightfalloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogsunbloom(mut self, val: f32) -> Self {
        self.params.insert(
            "fogsunbloom".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogsunbloom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogsunbloom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "fogintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bloomscale(mut self, val: f32) -> Self {
        self.params.insert(
            "bloomscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bloomscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bloomscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bloomintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "bloomintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bloomintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bloomintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bloomthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "bloomthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bloomthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bloomthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bokehaspect(mut self, val: f32) -> Self {
        self.params.insert(
            "bokehaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bokehaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokehaspect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bokehboost(mut self, val: f32) -> Self {
        self.params.insert(
            "bokehboost".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bokehboost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokehboost".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspnormals(mut self, val: f32) -> Self {
        self.params.insert(
            "cuspnormals".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspnormals".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lod(mut self, val: f32) -> Self {
        self.params.insert(
            "lod".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wirewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "wirewidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wirewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wirewidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wireblend(mut self, val: f32) -> Self {
        self.params.insert(
            "wireblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wireblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wireblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointsize(mut self, val: f32) -> Self {
        self.params.insert(
            "pointsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_discsize(mut self, val: f32) -> Self {
        self.params.insert(
            "discsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_discsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "discsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_headlightrotate(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "headlightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_headlightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headlightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fogrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fogrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spritetexmaxres(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "spritetexmaxres".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_spritetexmaxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spritetexmaxres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domelighttint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "domelighttint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_domelighttint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelighttint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domelightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "domelightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_domelightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyground(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "skyground".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_skyground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyground".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpkeycolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tpkeycolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tpkeycolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpkeycolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpfillcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tpfillcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tpfillcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpfillcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpbackcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tpbackcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tpbackcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpbackcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fogcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fogcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_jpeg_quality(mut self, val: i32) -> Self {
        self.params.insert(
            "vm_image_jpeg_quality".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vm_image_jpeg_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_jpeg_quality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lightsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "lightsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lightsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionblurframes(mut self, val: i32) -> Self {
        self.params.insert(
            "motionblurframes".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_motionblurframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionblurframes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tex2dres(mut self, val: i32) -> Self {
        self.params.insert(
            "tex2dres".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tex2dres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex2dres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tex3dres(mut self, val: i32) -> Self {
        self.params.insert(
            "tex3dres".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tex3dres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex3dres".to_string(),
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
    pub fn with_sheetsize(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "sheetsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_sheetsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheetsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverFlipbookTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opsource(mut self, val: DriverFlipbookOpsource) -> Self {
        self.params.insert(
            "opsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_opsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sopsource(mut self, val: DriverFlipbookSopsource) -> Self {
        self.params.insert(
            "sopsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sopsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sopsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagetype(mut self, val: DriverFlipbookImagetype) -> Self {
        self.params.insert(
            "imagetype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_imagetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorcorrect(mut self, val: DriverFlipbookColorcorrect) -> Self {
        self.params.insert(
            "colorcorrect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_colorcorrect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorcorrect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_tiff_compression(
        mut self,
        val: DriverFlipbookVmImageTiffCompression,
    ) -> Self {
        self.params.insert(
            "vm_image_tiff_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vm_image_tiff_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_tiff_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_exr_compression(
        mut self,
        val: DriverFlipbookVmImageExrCompression,
    ) -> Self {
        self.params.insert(
            "vm_image_exr_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vm_image_exr_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_exr_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadingmode(mut self, val: DriverFlipbookShadingmode) -> Self {
        self.params.insert(
            "shadingmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadingmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadingmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lighting(mut self, val: DriverFlipbookLighting) -> Self {
        self.params.insert(
            "lighting".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lighting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lighting".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worklighttype(mut self, val: DriverFlipbookWorklighttype) -> Self {
        self.params.insert(
            "worklighttype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_worklighttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worklighttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowquality(mut self, val: DriverFlipbookShadowquality) -> Self {
        self.params.insert(
            "shadowquality".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadowquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aamode(mut self, val: DriverFlipbookAamode) -> Self {
        self.params.insert(
            "aamode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_aamode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aamode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usehdr(mut self, val: DriverFlipbookUsehdr) -> Self {
        self.params.insert(
            "usehdr".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usehdr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usehdr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transparency(mut self, val: DriverFlipbookTransparency) -> Self {
        self.params.insert(
            "transparency".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_transparency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transparency".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogheightmode(mut self, val: DriverFlipbookFogheightmode) -> Self {
        self.params.insert(
            "fogheightmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fogheightmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogheightmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bokeh(mut self, val: DriverFlipbookBokeh) -> Self {
        self.params.insert(
            "bokeh".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bokeh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokeh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumequality(mut self, val: DriverFlipbookVolumequality) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_volumequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gennormal(mut self, val: DriverFlipbookGennormal) -> Self {
        self.params.insert(
            "gennormal".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_gennormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gennormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particle(mut self, val: DriverFlipbookParticle) -> Self {
        self.params.insert(
            "particle".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_particle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tex2dlimit(mut self, val: DriverFlipbookTex2dlimit) -> Self {
        self.params.insert(
            "tex2dlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tex2dlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex2dlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tex2dformat(mut self, val: DriverFlipbookTex2dformat) -> Self {
        self.params.insert(
            "tex2dformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tex2dformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex2dformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tex3dlimit(mut self, val: DriverFlipbookTex3dlimit) -> Self {
        self.params.insert(
            "tex3dlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tex3dlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex3dlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tex3dformat(mut self, val: DriverFlipbookTex3dformat) -> Self {
        self.params.insert(
            "tex3dformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tex3dformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tex3dformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scenepath(mut self, val: &str) -> Self {
        self.params.insert(
            "scenepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scenepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scenepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "vobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forceobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_forceobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludeobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_excludeobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alights(mut self, val: &str) -> Self {
        self.params.insert(
            "alights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_alights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcelights(mut self, val: &str) -> Self {
        self.params.insert(
            "forcelights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_forcelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcelights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludelights(mut self, val: &str) -> Self {
        self.params.insert(
            "excludelights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_excludelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludelights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgimage(mut self, val: &str) -> Self {
        self.params.insert(
            "bgimage".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bgimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bgimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vpcomment(mut self, val: &str) -> Self {
        self.params.insert(
            "vpcomment".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vpcomment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vpcomment".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ociocolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ociocolorspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ociocolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ociocolorspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ociolooks(mut self, val: &str) -> Self {
        self.params.insert(
            "ociolooks".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ociolooks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ociolooks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ociodisplay(mut self, val: &str) -> Self {
        self.params.insert(
            "ociodisplay".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ociodisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ociodisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocioview(mut self, val: &str) -> Self {
        self.params.insert(
            "ocioview".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocioview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ocioview".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_artist(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_image_artist".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_image_artist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_artist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_image_comment".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_image_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_comment".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_hostname(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_image_hostname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_image_hostname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_hostname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_image_mplay_label(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_image_mplay_label".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_image_mplay_label_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_image_mplay_label".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matcapimage(mut self, val: &str) -> Self {
        self.params.insert(
            "matcapimage".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matcapimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matcapimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domelightmap(mut self, val: &str) -> Self {
        self.params.insert(
            "domelightmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_domelightmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelightmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fognode(mut self, val: &str) -> Self {
        self.params.insert(
            "fognode".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fognode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fognode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bokehfile(mut self, val: &str) -> Self {
        self.params.insert(
            "bokehfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bokehfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokehfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bokehcop(mut self, val: &str) -> Self {
        self.params.insert(
            "bokehcop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bokehcop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokehcop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_purposerender(mut self, val: bool) -> Self {
        self.params.insert(
            "purposerender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_purposerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purposerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purposeproxy(mut self, val: bool) -> Self {
        self.params.insert(
            "purposeproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_purposeproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purposeproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purposeguide(mut self, val: bool) -> Self {
        self.params.insert(
            "purposeguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_purposeguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purposeguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soho_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soho_viewport_menu(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_viewport_menu".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_viewport_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_viewport_menu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tres(mut self, val: bool) -> Self {
        self.params.insert(
            "tres".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useraytrace(mut self, val: bool) -> Self {
        self.params.insert(
            "useraytrace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useraytrace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useraytrace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headlightspec(mut self, val: bool) -> Self {
        self.params.insert(
            "headlightspec".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_headlightspec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headlightspec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyusesun(mut self, val: bool) -> Self {
        self.params.insert(
            "skyusesun".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skyusesun_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyusesun".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyusesky(mut self, val: bool) -> Self {
        self.params.insert(
            "skyusesky".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skyusesky_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyusesky".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worklightshadows(mut self, val: bool) -> Self {
        self.params.insert(
            "worklightshadows".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_worklightshadows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worklightshadows".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "ambocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ambocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usematerials(mut self, val: bool) -> Self {
        self.params.insert(
            "usematerials".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usematerials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usematerials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetextures(mut self, val: bool) -> Self {
        self.params.insert(
            "usetextures".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetextures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetextures".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionblur(mut self, val: bool) -> Self {
        self.params.insert(
            "motionblur".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacement(mut self, val: bool) -> Self {
        self.params.insert(
            "displacement".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displacement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacement".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_backfacecull(mut self, val: bool) -> Self {
        self.params.insert(
            "backfacecull".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_backfacecull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "backfacecull".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformfog(mut self, val: bool) -> Self {
        self.params.insert(
            "uniformfog".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniformfog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformfog".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fogsunenable(mut self, val: bool) -> Self {
        self.params.insert(
            "fogsunenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fogsunenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogsunenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bloom(mut self, val: bool) -> Self {
        self.params.insert(
            "bloom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bloom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bloom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dof(mut self, val: bool) -> Self {
        self.params.insert(
            "dof".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usegeocolor(mut self, val: bool) -> Self {
        self.params.insert(
            "usegeocolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegeocolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegeocolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createtangents(mut self, val: bool) -> Self {
        self.params.insert(
            "createtangents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createtangents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createtangents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientton(mut self, val: bool) -> Self {
        self.params.insert(
            "orientton".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientton_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientton".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesprites(mut self, val: bool) -> Self {
        self.params.insert(
            "usesprites".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesprites_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesprites".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apexdeform(mut self, val: bool) -> Self {
        self.params.insert(
            "apexdeform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apexdeform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apexdeform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverFlipbook {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipbook"
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
pub struct DriverFramecontainer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverFramecontainer {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverFramecontainer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "framecontainer"
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
pub enum DriverFramedepMethod {
    FrameRange = 0,
    CurrentFrameAndPrevious = 1,
    CurrentFrameAndNext = 2,
    ListOfFrames = 3,
}

#[derive(Debug, Clone)]
pub struct DriverFramedep {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverFramedep {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_framedep_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("framedep{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_framedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("framedep{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_frange(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: DriverFramedepMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverFramedep {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "framedep"
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
