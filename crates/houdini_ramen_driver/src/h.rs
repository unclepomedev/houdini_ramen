#[derive(Debug, Clone)]
pub struct DriverHaircardtex {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverHaircardtex {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_samples(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "vm_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_vm_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hairobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "hairobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hairobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hairobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_picture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_picture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "nameprefix".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameprefix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_namesep(mut self, val: &str) -> Self {
        self.params.insert(
            "namesep".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_namesep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "namesep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameext(mut self, val: &str) -> Self {
        self.params.insert(
            "nameext".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameext_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameext".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffusename(mut self, val: &str) -> Self {
        self.params.insert(
            "diffusename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffusename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alphaname(mut self, val: &str) -> Self {
        self.params.insert(
            "alphaname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alphaname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alphaname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idname(mut self, val: &str) -> Self {
        self.params.insert(
            "idname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipname(mut self, val: &str) -> Self {
        self.params.insert(
            "tipname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthname(mut self, val: &str) -> Self {
        self.params.insert(
            "depthname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvboundsname(mut self, val: &str) -> Self {
        self.params.insert(
            "uvboundsname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvboundsname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvboundsname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forceobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
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
    pub fn with_outputdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "outputdiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputdiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputalpha(mut self, val: bool) -> Self {
        self.params.insert(
            "outputalpha".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputalpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputid(mut self, val: bool) -> Self {
        self.params.insert(
            "outputid".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtip(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtip".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputdepth(mut self, val: bool) -> Self {
        self.params.insert(
            "outputdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputuvbounds(mut self, val: bool) -> Self {
        self.params.insert(
            "outputuvbounds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputuvbounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputuvbounds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverHaircardtex {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "haircardtex"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DriverHaircardtexOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DriverHaircardtexOutputs for DriverHaircardtex {}
impl DriverHaircardtexOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DriverHaircardtex>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait DriverHaircardtexInnerExt {
    fn paths(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_depth(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn objects(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn render_depth(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn render_hairtex(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn render_uvbounds(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uv_bounds(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DriverHaircardtexInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, DriverHaircardtex>
{
    fn paths(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("PATHS")
    }
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn normalize_depth(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_depth")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn objects(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("objects")
    }
    fn render_depth(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("render_depth")
    }
    fn render_hairtex(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("render_hairtex")
    }
    fn render_uvbounds(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("render_uvbounds")
    }
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn uv_bounds(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uv_bounds")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqRenderTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqRenderHqAssignIfdgenTo {
    SameClientsAssignedToRenderJobs = 0,
    ListedClients = 1,
    ClientsFromListedGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqRenderHqAssignUsdgenTo {
    SameClientsAssignedToRenderJobs = 0,
    ListedClients = 1,
    ClientsFromListedGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqRenderHqAssignTo {
    AnyClient = 0,
    ListedClients = 1,
    ClientsFromListedGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqRenderHqRenderFrameOrder {
    RenderFramesInLinearOrder = 0,
    RenderFramesInBinarySubdivisionOrder = 1,
}

#[derive(Debug, Clone)]
pub struct DriverHqRender {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverHqRender {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_rop_to_submit_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input1".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_hq_cloud_check_progress(mut self) -> Self {
        self.params.insert(
            "hq_cloud_check_progress".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_hq_cloud_amazon_web_console(mut self) -> Self {
        self.params.insert(
            "hq_cloud_amazon_web_console".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_hq_cloud_help(mut self) -> Self {
        self.params.insert(
            "hq_cloud_help".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_hq_cloud_redetect_region(mut self) -> Self {
        self.params.insert(
            "hq_cloud_redetect_region".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_ifdgen_clients(mut self) -> Self {
        self.params.insert(
            "select_ifdgen_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_ifdgen_client_groups(mut self) -> Self {
        self.params.insert(
            "select_ifdgen_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_usdgen_clients(mut self) -> Self {
        self.params.insert(
            "select_usdgen_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_usdgen_client_groups(mut self) -> Self {
        self.params.insert(
            "select_usdgen_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_clients(mut self) -> Self {
        self.params.insert(
            "select_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_client_groups(mut self) -> Self {
        self.params.insert(
            "select_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert(
            "renderpreview".to_string(),
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
    pub fn with_hq_frame_range(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hq_frame_range_".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hq_frame_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_frame_range_".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soho_outputmode(mut self, val: i32) -> Self {
        self.params.insert(
            "soho_outputmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_outputmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_num_cloud_machines(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_num_cloud_machines".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_num_cloud_machines_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_num_cloud_machines".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_min_hosts(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_min_hosts".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_min_hosts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_min_hosts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_max_hosts(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_max_hosts".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_max_hosts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_max_hosts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_framesperjob(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_framesperjob".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_framesperjob_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_framesperjob".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cpus_to_use(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_CPUs_to_use".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_cpus_to_use_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_CPUs_to_use".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_tries_left(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_tries_left".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_tries_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_tries_left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res_count_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("res_count{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_res_count_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("res_count{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trange(mut self, val: DriverHqRenderTrange) -> Self {
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
    pub fn with_hq_priority(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_priority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hq_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_priority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cloud_region(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_cloud_region".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hq_cloud_region_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_cloud_region".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_assign_ifdgen_to(mut self, val: DriverHqRenderHqAssignIfdgenTo) -> Self {
        self.params.insert(
            "hq_assign_ifdgen_to".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hq_assign_ifdgen_to_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_assign_ifdgen_to".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_assign_usdgen_to(mut self, val: DriverHqRenderHqAssignUsdgenTo) -> Self {
        self.params.insert(
            "hq_assign_usdgen_to".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hq_assign_usdgen_to_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_assign_usdgen_to".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_assign_to(mut self, val: DriverHqRenderHqAssignTo) -> Self {
        self.params.insert(
            "hq_assign_to".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hq_assign_to_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_assign_to".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_render_frame_order(mut self, val: DriverHqRenderHqRenderFrameOrder) -> Self {
        self.params.insert(
            "hq_render_frame_order".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hq_render_frame_order_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_render_frame_order".to_string(),
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
    pub fn with_soho_program(mut self, val: &str) -> Self {
        self.params.insert(
            "soho_program".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soho_program_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_program".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_server(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_server".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_server_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_server".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_job_name(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_job_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_job_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_job_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_driver(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_driver".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_driver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_driver".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_linux(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs_linux".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_linux_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs_linux".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_macos(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs_macos".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_macos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs_macos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_windows(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs_windows".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_windows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs_windows".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip_action(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hip_action".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip_action_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hip_action".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hip".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_project_path(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_project_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_project_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_project_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_input_ifd(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_input_ifd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_input_ifd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_input_ifd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_input_usd(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_input_usd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_input_usd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_input_usd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_renderer(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_renderer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_renderer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_renderer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_outputimage(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_outputimage".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_outputimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_outputimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_rendercommand(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_rendercommand".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_rendercommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_rendercommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cloud_cached_file_info(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_cloud_cached_file_info".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cloud_cached_file_info_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_cloud_cached_file_info".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cloud_machine_type(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_cloud_machine_type".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cloud_machine_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_cloud_machine_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_emailto(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_emailTo".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_emailto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_emailTo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_outputifd(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_outputifd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_outputifd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_outputifd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_ifdgen_clients(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_ifdgen_clients".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_ifdgen_clients_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_ifdgen_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_ifdgen_client_groups(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_ifdgen_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_ifdgen_client_groups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_ifdgen_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_output_usd(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_output_usd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_output_usd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_output_usd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_usdgen_clients(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_usdgen_clients".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_usdgen_clients_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_usdgen_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_usdgen_client_groups(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_usdgen_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_usdgen_client_groups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_usdgen_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_clients(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_clients".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_clients_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_client_groups(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_client_groups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("res_name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("res_name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("var_name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("var_name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_value_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("var_value{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("var_value{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_directory_path_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("directory_path{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_directory_path_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("directory_path{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soho_multiframe(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_multiframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_multiframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_multiframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_use_render_tracker(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_use_render_tracker".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_use_render_tracker_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_use_render_tracker".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_openbrowser(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_openbrowser".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_openbrowser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_openbrowser".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_report_submitted_job_id(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_report_submitted_job_id".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_report_submitted_job_id_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_report_submitted_job_id".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_useuniversalhfs(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_useuniversalhfs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_useuniversalhfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_useuniversalhfs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_autosave(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_autosave".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_autosave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_autosave".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_warn_unsaved_changes(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_warn_unsaved_changes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_warn_unsaved_changes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_warn_unsaved_changes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_skip_file_dependency_dialog(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_skip_file_dependency_dialog".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_skip_file_dependency_dialog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_skip_file_dependency_dialog".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_will_email(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_will_email".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_will_email_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_will_email".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_start(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_start".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_success(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_success".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_success_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_success".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_failure(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_failure".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_failure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_failure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_pause(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_pause".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_pause_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_pause".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_resume(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_resume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_resume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_resume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_reschedule(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_reschedule".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_reschedule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_reschedule".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_priority_change(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_priority_change".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_priority_change_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_priority_change".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_makeifds(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_makeifds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_makeifds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_makeifds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_delete_ifds(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_delete_ifds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_delete_ifds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_delete_ifds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cross_platform_ifd(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_cross_platform_ifd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_cross_platform_ifd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_cross_platform_ifd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_distribute_ifd_gen(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_distribute_ifd_gen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_distribute_ifd_gen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_distribute_ifd_gen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_render_single_tile(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_render_single_tile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_render_single_tile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_render_single_tile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_enable_checkpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_enable_checkpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_enable_checkpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_enable_checkpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_make_usds(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_make_usds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_make_usds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_make_usds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_delete_usds(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_delete_usds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_delete_usds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_delete_usds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_distribute_usd_gen(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_distribute_usd_gen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_distribute_usd_gen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_distribute_usd_gen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_batch_all_frames(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_batch_all_frames".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_batch_all_frames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_batch_all_frames".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_is_cpu_number_set(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_is_CPU_number_set".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_is_cpu_number_set_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_is_CPU_number_set".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_tries_different_client(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_tries_different_client".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_tries_different_client_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_tries_different_client".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverHqRender {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hq_render"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DriverHqRenderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DriverHqRenderOutputs for DriverHqRender {}
impl DriverHqRenderOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DriverHqRender> {}

pub trait DriverHqRenderWiringExt {
    fn set_rop_to_submit_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> DriverHqRenderWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, DriverHqRender>
{
    fn set_rop_to_submit_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input1", output)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqSimTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqSimSliceType {
    VolumeSlices = 0,
    ParticleSlices = 1,
    Clusters = 2,
    Wedges = 3,
    None = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqSimHqAssignTo {
    AnyClient = 0,
    ListedClients = 1,
    ClientsFromListedGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHqSimHqDependencyOrder {
    FrameByFrame = 0,
    NodeByNode = 1,
}

#[derive(Debug, Clone)]
pub struct DriverHqSim {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverHqSim {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_rop_to_submit_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input1".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_clients(mut self) -> Self {
        self.params.insert(
            "select_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_client_groups(mut self) -> Self {
        self.params.insert(
            "select_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert(
            "renderpreview".to_string(),
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
    pub fn with_soho_outputmode(mut self, val: i32) -> Self {
        self.params.insert(
            "soho_outputmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_outputmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_num_slices(mut self, val: i32) -> Self {
        self.params.insert(
            "num_slices".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_num_slices_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "num_slices".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cpus_to_use(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_CPUs_to_use".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hq_cpus_to_use_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_CPUs_to_use".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res_count_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("res_count{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_res_count_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("res_count{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slicediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trange(mut self, val: DriverHqSimTrange) -> Self {
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
    pub fn with_hq_priority(mut self, val: i32) -> Self {
        self.params.insert(
            "hq_priority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hq_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_priority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slice_type(mut self, val: DriverHqSimSliceType) -> Self {
        self.params.insert(
            "slice_type".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slice_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_assign_to(mut self, val: DriverHqSimHqAssignTo) -> Self {
        self.params.insert(
            "hq_assign_to".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hq_assign_to_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_assign_to".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_dependency_order(mut self, val: DriverHqSimHqDependencyOrder) -> Self {
        self.params.insert(
            "hq_dependency_order".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hq_dependency_order_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_dependency_order".to_string(),
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
    pub fn with_soho_program(mut self, val: &str) -> Self {
        self.params.insert(
            "soho_program".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soho_program_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_program".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_server(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_server".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_server_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_server".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_job_name(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_job_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_job_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_job_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_driver(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_driver".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_driver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_driver".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_linux(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs_linux".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_linux_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs_linux".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_macos(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs_macos".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_macos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs_macos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_windows(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hfs_windows".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hfs_windows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hfs_windows".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip_action(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hip_action".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip_action_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hip_action".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_hip".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_hip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_hip".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_project_path(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_project_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_project_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_project_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_sim_controls(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_sim_controls".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_sim_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_sim_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cluster_node(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_cluster_node".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_cluster_node_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_cluster_node".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_emailto(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_emailTo".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_emailto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_emailTo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_clients(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_clients".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_clients_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_client_groups(mut self, val: &str) -> Self {
        self.params.insert(
            "hq_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_client_groups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("res_name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("res_name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("var_name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("var_name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_value_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("var_value{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_var_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("var_value{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_directory_path_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("directory_path{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_directory_path_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("directory_path{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soho_multiframe(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_multiframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_multiframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_multiframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_report_submitted_job_id(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_report_submitted_job_id".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_report_submitted_job_id_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_report_submitted_job_id".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_useuniversalhfs(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_useuniversalhfs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_useuniversalhfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_useuniversalhfs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_autosave(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_autosave".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_autosave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_autosave".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_warn_unsaved_changes(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_warn_unsaved_changes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_warn_unsaved_changes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_warn_unsaved_changes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_skip_file_dependency_dialog(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_skip_file_dependency_dialog".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_skip_file_dependency_dialog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_skip_file_dependency_dialog".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_use_dedicated_tracker(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_use_dedicated_tracker".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_use_dedicated_tracker_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_use_dedicated_tracker".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_openbrowser(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_openbrowser".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_openbrowser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_openbrowser".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_will_email(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_will_email".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_will_email_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_will_email".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_start(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_start".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_success(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_success".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_success_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_success".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_failure(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_failure".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_failure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_failure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_pause(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_pause".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_pause_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_pause".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_resume(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_resume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_resume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_resume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_reschedule(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_reschedule".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_reschedule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_reschedule".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_email_on_priority_change(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_email_on_priority_change".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_email_on_priority_change_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_email_on_priority_change".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hq_is_cpu_number_set(mut self, val: bool) -> Self {
        self.params.insert(
            "hq_is_CPU_number_set".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hq_is_cpu_number_set_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hq_is_CPU_number_set".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enable_perfmon(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_perfmon".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_perfmon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_perfmon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverHqSim {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hq_sim"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DriverHqSimOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DriverHqSimOutputs for DriverHqSim {}
impl DriverHqSimOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DriverHqSim> {}

pub trait DriverHqSimWiringExt {
    fn set_rop_to_submit_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> DriverHqSimWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, DriverHqSim>
{
    fn set_rop_to_submit_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input1", output)
    }
}
