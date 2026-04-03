#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBakeAnimationRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBakeAnimationUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct DriverBakeAnimation {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverBakeAnimation {
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

    // --- Float parameters ---
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert("rate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert("rate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_start_end(mut self, val: [f32; 2]) -> Self {
        self.params.insert("start_end".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_start_end_expr(mut self, expr: &str) -> Self {
        self.params.insert("start_end".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_range(mut self, val: DriverBakeAnimationRange) -> Self {
        self.params.insert("range".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_units(mut self, val: DriverBakeAnimationUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_user(mut self, val: &str) -> Self {
        self.params.insert("user".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_user_expr(mut self, expr: &str) -> Self {
        self.params.insert("user".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert("source".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert("source".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_parm_filter(mut self, val: &str) -> Self {
        self.params.insert("parm_filter".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_parm_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert("parm_filter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_log(mut self, val: &str) -> Self {
        self.params.insert("log".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_log_expr(mut self, expr: &str) -> Self {
        self.params.insert("log".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_delete_animation(mut self, val: bool) -> Self {
        self.params.insert("delete_animation".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_delete_animation_expr(mut self, expr: &str) -> Self {
        self.params.insert("delete_animation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_revert_parameters(mut self, val: bool) -> Self {
        self.params.insert("revert_parameters".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_revert_parameters_expr(mut self, expr: &str) -> Self {
        self.params.insert("revert_parameters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_copy_parameters(mut self, val: bool) -> Self {
        self.params.insert("copy_parameters".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copy_parameters_expr(mut self, expr: &str) -> Self {
        self.params.insert("copy_parameters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_copy_transforms(mut self, val: bool) -> Self {
        self.params.insert("copy_transforms".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copy_transforms_expr(mut self, expr: &str) -> Self {
        self.params.insert("copy_transforms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_copy_constraints(mut self, val: bool) -> Self {
        self.params.insert("copy_constraints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copy_constraints_expr(mut self, expr: &str) -> Self {
        self.params.insert("copy_constraints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_copy_hierarchy(mut self, val: bool) -> Self {
        self.params.insert("copy_hierarchy".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copy_hierarchy_expr(mut self, expr: &str) -> Self {
        self.params.insert("copy_hierarchy".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_write_to_chop_channel(mut self, val: bool) -> Self {
        self.params.insert("write_to_chop_channel".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_write_to_chop_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("write_to_chop_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverBakeAnimation {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bake_animation"
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
pub trait DriverBakeAnimationInnerExt {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn exports(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn extract_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> DriverBakeAnimationInnerExt for crate::core::graph::InnerGraph<'a> {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("OUT")
    }
    fn exports(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("exports")
    }
    fn extract_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("extract_constraints")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureUvresmenu {
    /// 256 x 256
    N256X256 = 0,
    /// 512 x 512
    N512X512 = 1,
    /// 1024 x 1024
    N1024X1024 = 2,
    /// 2048 x 2048
    N2048X2048 = 3,
    /// 4096 x 4096
    N4096X4096 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureVmUvptexminresmenu {
    /// 4 x 4
    N4X4 = 0,
    /// 8 x 8
    N8X8 = 1,
    /// 16 x 16
    N16X16 = 2,
    /// 32 x 32
    N32X32 = 3,
    /// 64 x 64
    N64X64 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureVmUvptexmaxresmenu {
    /// 32 x 32
    N32X32 = 0,
    /// 64 x 64
    N64X64 = 1,
    /// 128 x 128
    N128X128 = 2,
    /// 256 x 256
    N256X256 = 3,
    /// 512 x 512
    N512X512 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureVmUvptexscalemenu {
    /// 0
    N0 = 0,
    /// 0.5
    N05 = 1,
    /// 1
    N1 = 2,
    /// 2
    N2 = 3,
    /// 5
    N5 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureVmUvUnwrapMethod {
    UvToSurface = 0,
    TraceClosestSurface = 1,
    UvMatch = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBaketextureUvborderpixelsmenu {
    /// 1
    N1 = 0,
    /// 2
    N2 = 1,
    /// 5
    N5 = 2,
    /// 8
    N8 = 3,
    /// 10
    N10 = 4,
}

#[derive(Debug, Clone)]
pub struct DriverBaketexture {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverBaketexture {
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
    pub fn with_vm_ptexmapscale(mut self, val: f32) -> Self {
        self.params.insert("vm_ptexmapscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_vm_ptexmapscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_ptexmapscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvshadingquality(mut self, val: f32) -> Self {
        self.params.insert("vm_uvshadingquality".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_vm_uvshadingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvshadingquality".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uv_ray_bias(mut self, val: f32) -> Self {
        self.params.insert("vm_uv_ray_bias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_vm_uv_ray_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uv_ray_bias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uv_ray_maxdist(mut self, val: f32) -> Self {
        self.params.insert("vm_uv_ray_maxdist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_vm_uv_ray_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uv_ray_maxdist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_vm_uv_unwrap_method(mut self, val: DriverBaketextureVmUvUnwrapMethod) -> Self {
        self.params.insert("vm_uv_unwrap_method".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_vm_uv_unwrap_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uv_unwrap_method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvborderpixels(mut self, val: i32) -> Self {
        self.params.insert("vm_uvborderpixels".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_vm_uvborderpixels_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvborderpixels".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int2 parameters ---
    pub fn with_vm_uvunwrapres(mut self, val: [i32; 2]) -> Self {
        self.params.insert("vm_uvunwrapres".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_vm_uvunwrapres_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvunwrapres".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_ptexmapminres(mut self, val: [i32; 2]) -> Self {
        self.params.insert("vm_ptexmapminres".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_vm_ptexmapminres_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_ptexmapminres".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_ptexmapmaxres(mut self, val: [i32; 2]) -> Self {
        self.params.insert("vm_ptexmapmaxres".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_vm_ptexmapmaxres_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_ptexmapmaxres".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverBaketextureTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvresmenu(mut self, val: DriverBaketextureUvresmenu) -> Self {
        self.params.insert("uvresmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uvresmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvresmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvptexminresmenu(mut self, val: DriverBaketextureVmUvptexminresmenu) -> Self {
        self.params.insert("vm_uvptexminresmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vm_uvptexminresmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvptexminresmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvptexmaxresmenu(mut self, val: DriverBaketextureVmUvptexmaxresmenu) -> Self {
        self.params.insert("vm_uvptexmaxresmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vm_uvptexmaxresmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvptexmaxresmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvptexscalemenu(mut self, val: DriverBaketextureVmUvptexscalemenu) -> Self {
        self.params.insert("vm_uvptexscalemenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vm_uvptexscalemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvptexscalemenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvborderpixelsmenu(mut self, val: DriverBaketextureUvborderpixelsmenu) -> Self {
        self.params.insert("uvborderpixelsmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uvborderpixelsmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvborderpixelsmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_vm_uvtype(mut self, val: &str) -> Self {
        self.params.insert("vm_uvtype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvobject_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("vm_uvobject{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvobject_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("vm_uvobject{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvcageobject_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("vm_uvcageobject{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvcageobject_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("vm_uvcageobject{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvhires_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("vm_uvhires{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvhires_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("vm_uvhires{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvoutputpicture_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("vm_uvoutputpicture{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvoutputpicture_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("vm_uvoutputpicture{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_vm_uvattribute(mut self, val: &str) -> Self {
        self.params.insert("vm_uvattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvpostprocess(mut self, val: &str) -> Self {
        self.params.insert("vm_uvpostprocess".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvpostprocess_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvpostprocess".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_uvlightpaths(mut self, val: &str) -> Self {
        self.params.insert("vm_uvlightpaths".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vm_uvlightpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvlightpaths".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_soho_compression(mut self, val: &str) -> Self {
        self.params.insert("soho_compression".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_compression".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_vm_uvmkpath(mut self, val: bool) -> Self {
        self.params.insert("vm_uvmkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vm_uvmkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uvmkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_vm_uvobjectenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("vm_uvobjectenable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vm_uvobjectenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("vm_uvobjectenable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_isuvrendering(mut self, val: bool) -> Self {
        self.params.insert("vm_isuvrendering".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vm_isuvrendering_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_isuvrendering".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_ipr_support(mut self, val: bool) -> Self {
        self.params.insert("soho_ipr_support".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_ipr_support_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_ipr_support".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_vm_uv_flip_normal(mut self, val: bool) -> Self {
        self.params.insert("vm_uv_flip_normal".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vm_uv_flip_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_uv_flip_normal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vm_ptexwraporient(mut self, val: bool) -> Self {
        self.params.insert("vm_ptexwraporient".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vm_ptexwraporient_expr(mut self, expr: &str) -> Self {
        self.params.insert("vm_ptexwraporient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_soho_outputmode(mut self, val: bool) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
}

impl crate::core::types::HoudiniNode for DriverBaketexture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "baketexture"
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
pub struct DriverBatch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DriverBatch {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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

    // --- Toggle parameters ---
    pub fn with_fullrange(mut self, val: bool) -> Self {
        self.params.insert("fullrange".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fullrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("fullrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverBatch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "batch"
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
pub enum DriverBrickmapTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverBrickmapI3d {
    /// Convert .i3d file
    ConvertI3dFile = 0,
    /// Convert .geo/.bgeo file
    ConvertGeoBgeoFile = 1,
    ConvertSop = 2,
}

#[derive(Debug, Clone)]
pub struct DriverBrickmap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DriverBrickmap {
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

    // --- Float parameters ---
    pub fn with_volumefilterwidth(mut self, val: f32) -> Self {
        self.params.insert("volumefilterwidth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_volumefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert("volumefilterwidth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_trange(mut self, val: DriverBrickmapTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_i3d(mut self, val: DriverBrickmapI3d) -> Self {
        self.params.insert("i3d".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_i3d_expr(mut self, expr: &str) -> Self {
        self.params.insert("i3d".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_output(mut self, val: &str) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_i3dfile(mut self, val: &str) -> Self {
        self.params.insert("i3dfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_i3dfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("i3dfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_geofile(mut self, val: &str) -> Self {
        self.params.insert("geofile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_geofile_expr(mut self, expr: &str) -> Self {
        self.params.insert("geofile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sop(mut self, val: &str) -> Self {
        self.params.insert("sop".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sop_expr(mut self, expr: &str) -> Self {
        self.params.insert("sop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ptcfile(mut self, val: &str) -> Self {
        self.params.insert("ptcfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ptcfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("ptcfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_volumefilter(mut self, val: &str) -> Self {
        self.params.insert("volumefilter".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_volumefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert("volumefilter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_soho_mkpath(mut self, val: bool) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_soho_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createos(mut self, val: bool) -> Self {
        self.params.insert("createOs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createos_expr(mut self, expr: &str) -> Self {
        self.params.insert("createOs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverBrickmap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "brickmap"
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
