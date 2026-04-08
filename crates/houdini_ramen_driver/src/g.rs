#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeo2i3dTrange {
    RenderAnyFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone)]
pub struct DriverGeo2i3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverGeo2i3d {
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
    pub fn with_min(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "min".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "min".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_max(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "max".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_trange(mut self, val: DriverGeo2i3dTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
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
    pub fn with_primnum(mut self, val: i32) -> Self {
        self.params.insert(
            "primnum".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primnum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_res(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_image(mut self, val: &str) -> Self {
        self.params.insert(
            "image".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_image_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
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
}

impl houdini_ramen_core::types::HoudiniNode for DriverGeo2i3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "geo2i3d"
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
pub trait DriverGeo2i3dInnerExt {
    fn image3d1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn shopnet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DriverGeo2i3dInnerExt for houdini_ramen_core::graph::InnerGraph<'a, DriverGeo2i3d> {
    fn image3d1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("image3d1")
    }
    fn shopnet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("shopnet1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryXformtype {
    None = 0,
    IntoWorldSpace = 1,
}

#[derive(Debug, Clone)]
pub struct DriverGeometry {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverGeometry {
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
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
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

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverGeometryTrange) -> Self {
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
    pub fn with_xformtype(mut self, val: DriverGeometryXformtype) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_sopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "sopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sopoutput".to_string(),
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
    pub fn with_postwrite(mut self, val: &str) -> Self {
        self.params.insert(
            "postwrite".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postwrite_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postwrite".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostwrite(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostwrite".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostwrite_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostwrite".to_string(),
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

    // --- Toggle parameters ---
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
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
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
    pub fn with_tpostwrite(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostwrite".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostwrite_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostwrite".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DriverGeometry {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "geometry"
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
pub enum DriverGeometryrawTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryrawXformtype {
    None = 0,
    IntoWorldSpace = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryrawEndian {
    /// Little (Intel)
    LittleIntel = 0,
    /// Big (Network)
    BigNetwork = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryrawTarget {
    Zero = 0,
    Float = 1,
    Integer = 2,
    DetailAttribute = 3,
    PointAttribute = 4,
    Volume = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryrawType {
    Float = 0,
    Integer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryrawPrecision {
    /// 8-bit
    N8MinusBit = 0,
    /// 16-bit
    N16MinusBit = 1,
    /// 32-bit
    N32MinusBit = 2,
    /// 64-bit
    N64MinusBit = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGeometryrawVolorder {
    Zyx = 0,
    Xyz = 1,
}

#[derive(Debug, Clone)]
pub struct DriverGeometryraw {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverGeometryraw {
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
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
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
    pub fn with_fval_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("fval{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("fval{}", index1),
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
    pub fn with_tuplesize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("tuplesize{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tuplesize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("tuplesize{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ival_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("ival{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ival_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ival{}", index1),
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

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverGeometryrawTrange) -> Self {
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
    pub fn with_xformtype(mut self, val: DriverGeometryrawXformtype) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endian(mut self, val: DriverGeometryrawEndian) -> Self {
        self.params.insert(
            "endian".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_endian_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endian".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_target_inst(mut self, index1: usize, val: DriverGeometryrawTarget) -> Self {
        self.params.insert(
            format!("target{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_target_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("target{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: DriverGeometryrawType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: DriverGeometryrawPrecision) -> Self {
        self.params.insert(
            format!("precision{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("precision{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_volorder_inst(mut self, index1: usize, val: DriverGeometryrawVolorder) -> Self {
        self.params.insert(
            format!("volorder{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_volorder_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("volorder{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_sopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "sopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
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
    pub fn with_postwrite(mut self, val: &str) -> Self {
        self.params.insert(
            "postwrite".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postwrite_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postwrite".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostwrite(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostwrite".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostwrite_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostwrite".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_bfloat_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bfloat{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bfloat_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bfloat{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collate_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("collate{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collate_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collate{}", index1),
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
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
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
    pub fn with_tpostwrite(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostwrite".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostwrite_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostwrite".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DriverGeometryraw {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "geometryraw"
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
pub enum DriverGltfTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGltfExporttype {
    DetectFromFilename = 0,
    Gltf = 1,
    Glb = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGltfTexturesource {
    ExportFromMaterial = 0,
    CopyOriginalTextures = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGltfImageformat {
    OriginalFormatOrPng = 0,
    OriginalFormatOrJpeg = 1,
    Png = 2,
    Jpeg = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGltfMaxresolution {
    NoLimit = 0,
    /// 256x256
    N256x256 = 1,
    /// 512x512
    N512x512 = 2,
    /// 1024x1024
    N1024x1024 = 3,
    /// 2048x2048
    N2048x2048 = 4,
    /// 4096x4096
    N4096x4096 = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverGltfMeshnamessource {
    NodeName = 0,
    Attribute = 1,
}

#[derive(Debug, Clone)]
pub struct DriverGltf {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverGltf {
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
    pub fn with_imagequality(mut self, val: i32) -> Self {
        self.params.insert(
            "imagequality".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imagequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagequality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverGltfTrange) -> Self {
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
    pub fn with_exporttype(mut self, val: DriverGltfExporttype) -> Self {
        self.params.insert(
            "exporttype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exporttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exporttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_texturesource(mut self, val: DriverGltfTexturesource) -> Self {
        self.params.insert(
            "texturesource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_texturesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texturesource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_imageformat(mut self, val: DriverGltfImageformat) -> Self {
        self.params.insert(
            "imageformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_imageformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imageformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxresolution(mut self, val: DriverGltfMaxresolution) -> Self {
        self.params.insert(
            "maxresolution".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxresolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxresolution".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_meshnamessource(mut self, val: DriverGltfMeshnamessource) -> Self {
        self.params.insert(
            "meshnamessource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_meshnamessource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "meshnamessource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_objpath(mut self, val: &str) -> Self {
        self.params.insert(
            "objpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objects(mut self, val: &str) -> Self {
        self.params.insert(
            "objects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forceobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forceobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludeobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludeobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyright(mut self, val: &str) -> Self {
        self.params.insert(
            "copyright".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_additionaltexture_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_name", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_additionaltexture_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_name", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_additionaltexture_path_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_path", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_additionaltexture_path_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_path", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animationname(mut self, val: &str) -> Self {
        self.params.insert(
            "animationname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animationname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animationname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_meshnamesattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "meshnamesattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_meshnamesattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "meshnamesattribute".to_string(),
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

    // --- Toggle parameters ---
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
    pub fn with_usesoppath(mut self, val: bool) -> Self {
        self.params.insert(
            "usesoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_poweroftwo(mut self, val: bool) -> Self {
        self.params.insert(
            "poweroftwo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_poweroftwo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "poweroftwo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flipnormalmapy(mut self, val: bool) -> Self {
        self.params.insert(
            "flipnormalmapy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flipnormalmapy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flipnormalmapy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savehidden(mut self, val: bool) -> Self {
        self.params.insert(
            "savehidden".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savehidden_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savehidden".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cullempty(mut self, val: bool) -> Self {
        self.params.insert(
            "cullempty".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cullempty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cullempty".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportnames(mut self, val: bool) -> Self {
        self.params.insert(
            "exportnames".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportnames".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportmeshnames(mut self, val: bool) -> Self {
        self.params.insert(
            "exportmeshnames".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportmeshnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportmeshnames".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportmaterials(mut self, val: bool) -> Self {
        self.params.insert(
            "exportmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportcameras(mut self, val: bool) -> Self {
        self.params.insert(
            "exportcameras".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportcameras_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportcameras".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exportlights(mut self, val: bool) -> Self {
        self.params.insert(
            "exportlights".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportlights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportlights".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DriverGltf {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "gltf"
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
