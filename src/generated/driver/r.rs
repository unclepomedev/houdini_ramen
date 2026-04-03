#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverRibTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverRibSohoOutputmode {
    Command = 0,
    DiskFile = 1,
    NoOutput = 2,
}

#[derive(Debug, Clone)]
pub struct DriverRib {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverRib {
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


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert("renderpreview".to_string(), crate::core::types::ParamValue::Button);
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
    pub fn with_aspect_override(mut self, val: f32) -> Self {
        self.params.insert("aspect_override".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_aspect_override_expr(mut self, expr: &str) -> Self {
        self.params.insert("aspect_override".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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

    // --- Int2 parameters ---
    pub fn with_res_override(mut self, val: [i32; 2]) -> Self {
        self.params.insert("res_override".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_res_override_expr(mut self, expr: &str) -> Self {
        self.params.insert("res_override".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverRibTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_res_overridemenu(mut self, val: i32) -> Self {
        self.params.insert("res_overrideMenu".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_res_overridemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("res_overrideMenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_outputmode(mut self, val: DriverRibSohoOutputmode) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert("camera".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert("camera".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_res_fraction(mut self, val: &str) -> Self {
        self.params.insert("res_fraction".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_res_fraction_expr(mut self, expr: &str) -> Self {
        self.params.insert("res_fraction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_program(mut self, val: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_program_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_shopstyle(mut self, val: &str) -> Self {
        self.params.insert("soho_shopstyle".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_shopstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_shopstyle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vobject(mut self, val: &str) -> Self {
        self.params.insert("vobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("vobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_forceobject(mut self, val: &str) -> Self {
        self.params.insert("forceobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_forceobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("forceobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_matte_objects(mut self, val: &str) -> Self {
        self.params.insert("matte_objects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_matte_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert("matte_objects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_phantom_objects(mut self, val: &str) -> Self {
        self.params.insert("phantom_objects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_phantom_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert("phantom_objects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_excludeobject(mut self, val: &str) -> Self {
        self.params.insert("excludeobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_excludeobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("excludeobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sololight(mut self, val: &str) -> Self {
        self.params.insert("sololight".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sololight_expr(mut self, expr: &str) -> Self {
        self.params.insert("sololight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alights(mut self, val: &str) -> Self {
        self.params.insert("alights".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_alights_expr(mut self, expr: &str) -> Self {
        self.params.insert("alights".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_forcelights(mut self, val: &str) -> Self {
        self.params.insert("forcelights".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_forcelights_expr(mut self, expr: &str) -> Self {
        self.params.insert("forcelights".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_excludelights(mut self, val: &str) -> Self {
        self.params.insert("excludelights".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_excludelights_expr(mut self, expr: &str) -> Self {
        self.params.insert("excludelights".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vfog(mut self, val: &str) -> Self {
        self.params.insert("vfog".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vfog_expr(mut self, expr: &str) -> Self {
        self.params.insert("vfog".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert("target".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert("target".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ri_backend(mut self, val: &str) -> Self {
        self.params.insert("ri_backend".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ri_backend_expr(mut self, expr: &str) -> Self {
        self.params.insert("ri_backend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ri_begin(mut self, val: &str) -> Self {
        self.params.insert("ri_begin".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ri_begin_expr(mut self, expr: &str) -> Self {
        self.params.insert("ri_begin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_pipecmd(mut self, val: &str) -> Self {
        self.params.insert("soho_pipecmd".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_pipecmd_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_pipecmd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_diskfile(mut self, val: &str) -> Self {
        self.params.insert("soho_diskfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_diskfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_diskfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_propertiespath(mut self, val: &str) -> Self {
        self.params.insert("shop_propertiespath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shop_propertiespath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_propertiespath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_override_camerares(mut self, val: bool) -> Self {
        self.params.insert("override_camerares".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_override_camerares_expr(mut self, expr: &str) -> Self {
        self.params.insert("override_camerares".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_autoheadlight(mut self, val: bool) -> Self {
        self.params.insert("soho_autoheadlight".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_autoheadlight_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_autoheadlight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_rib_outputmode(mut self, val: bool) -> Self {
        self.params.insert("rib_outputmode".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_rib_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("rib_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_foreground(mut self, val: bool) -> Self {
        self.params.insert("soho_foreground".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_foreground_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_foreground".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_mkpath(mut self, val: bool) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_initsim(mut self, val: bool) -> Self {
        self.params.insert("soho_initsim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_initsim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_viewport_menu(mut self, val: bool) -> Self {
        self.params.insert("soho_viewport_menu".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_viewport_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_viewport_menu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_previewsupport(mut self, val: bool) -> Self {
        self.params.insert("soho_previewsupport".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_previewsupport_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_previewsupport".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverRib {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "rib"
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
pub enum DriverRibarchiveTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone)]
pub struct DriverRibarchive {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverRibarchive {
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


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert("renderpreview".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert("executebackground".to_string(), crate::core::types::ParamValue::Button);
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

    // --- Int parameters ---
    pub fn with_soho_outputmode(mut self, val: i32) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverRibarchiveTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_program(mut self, val: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_program_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_shopstyle(mut self, val: &str) -> Self {
        self.params.insert("soho_shopstyle".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_shopstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_shopstyle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert("target".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert("target".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_arch_objpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("arch_objpath{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_arch_objpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("arch_objpath{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_arch_archive_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("arch_archive{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_arch_archive_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("arch_archive{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_propertiespath(mut self, val: &str) -> Self {
        self.params.insert("shop_propertiespath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shop_propertiespath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_propertiespath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_soho_rismode(mut self, val: bool) -> Self {
        self.params.insert("soho_rismode".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_rismode_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_rismode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_mkpath(mut self, val: bool) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_initsim(mut self, val: bool) -> Self {
        self.params.insert("soho_initsim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_initsim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_arch_disable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("arch_disable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_arch_disable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("arch_disable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_arch_instance_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("arch_instance{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_arch_instance_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("arch_instance{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
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
}

impl crate::core::types::HoudiniNode for DriverRibarchive {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ribarchive"
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
pub enum DriverRisTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverRisSohoOutputmode {
    Command = 0,
    DiskFile = 1,
    NoOutput = 2,
}

#[derive(Debug, Clone)]
pub struct DriverRis {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverRis {
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


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert("renderpreview".to_string(), crate::core::types::ParamValue::Button);
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
    pub fn with_aspect_override(mut self, val: f32) -> Self {
        self.params.insert("aspect_override".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_aspect_override_expr(mut self, expr: &str) -> Self {
        self.params.insert("aspect_override".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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

    // --- Int2 parameters ---
    pub fn with_res_override(mut self, val: [i32; 2]) -> Self {
        self.params.insert("res_override".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_res_override_expr(mut self, expr: &str) -> Self {
        self.params.insert("res_override".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverRisTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_res_overridemenu(mut self, val: i32) -> Self {
        self.params.insert("res_overrideMenu".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_res_overridemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("res_overrideMenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_outputmode(mut self, val: DriverRisSohoOutputmode) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert("camera".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert("camera".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_res_fraction(mut self, val: &str) -> Self {
        self.params.insert("res_fraction".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_res_fraction_expr(mut self, expr: &str) -> Self {
        self.params.insert("res_fraction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_program(mut self, val: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_program_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_shopstyle(mut self, val: &str) -> Self {
        self.params.insert("soho_shopstyle".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_shopstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_shopstyle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vobject(mut self, val: &str) -> Self {
        self.params.insert("vobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("vobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_forceobject(mut self, val: &str) -> Self {
        self.params.insert("forceobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_forceobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("forceobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_matte_objects(mut self, val: &str) -> Self {
        self.params.insert("matte_objects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_matte_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert("matte_objects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_phantom_objects(mut self, val: &str) -> Self {
        self.params.insert("phantom_objects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_phantom_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert("phantom_objects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_excludeobject(mut self, val: &str) -> Self {
        self.params.insert("excludeobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_excludeobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("excludeobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sololight(mut self, val: &str) -> Self {
        self.params.insert("sololight".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sololight_expr(mut self, expr: &str) -> Self {
        self.params.insert("sololight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alights(mut self, val: &str) -> Self {
        self.params.insert("alights".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_alights_expr(mut self, expr: &str) -> Self {
        self.params.insert("alights".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_forcelights(mut self, val: &str) -> Self {
        self.params.insert("forcelights".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_forcelights_expr(mut self, expr: &str) -> Self {
        self.params.insert("forcelights".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_excludelights(mut self, val: &str) -> Self {
        self.params.insert("excludelights".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_excludelights_expr(mut self, expr: &str) -> Self {
        self.params.insert("excludelights".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vfog(mut self, val: &str) -> Self {
        self.params.insert("vfog".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vfog_expr(mut self, expr: &str) -> Self {
        self.params.insert("vfog".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert("target".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert("target".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ri_backend(mut self, val: &str) -> Self {
        self.params.insert("ri_backend".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ri_backend_expr(mut self, expr: &str) -> Self {
        self.params.insert("ri_backend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ri_begin(mut self, val: &str) -> Self {
        self.params.insert("ri_begin".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ri_begin_expr(mut self, expr: &str) -> Self {
        self.params.insert("ri_begin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_pipecmd(mut self, val: &str) -> Self {
        self.params.insert("soho_pipecmd".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_pipecmd_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_pipecmd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_diskfile(mut self, val: &str) -> Self {
        self.params.insert("soho_diskfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_diskfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_diskfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_propertiespath(mut self, val: &str) -> Self {
        self.params.insert("shop_propertiespath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shop_propertiespath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_propertiespath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_override_camerares(mut self, val: bool) -> Self {
        self.params.insert("override_camerares".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_override_camerares_expr(mut self, expr: &str) -> Self {
        self.params.insert("override_camerares".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_autoheadlight(mut self, val: bool) -> Self {
        self.params.insert("soho_autoheadlight".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_autoheadlight_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_autoheadlight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_rib_outputmode(mut self, val: bool) -> Self {
        self.params.insert("rib_outputmode".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_rib_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("rib_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_foreground(mut self, val: bool) -> Self {
        self.params.insert("soho_foreground".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_foreground_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_foreground".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_mkpath(mut self, val: bool) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_initsim(mut self, val: bool) -> Self {
        self.params.insert("soho_initsim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_initsim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_viewport_menu(mut self, val: bool) -> Self {
        self.params.insert("soho_viewport_menu".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_viewport_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_viewport_menu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_previewsupport(mut self, val: bool) -> Self {
        self.params.insert("soho_previewsupport".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_previewsupport_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_previewsupport".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverRis {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ris"
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
