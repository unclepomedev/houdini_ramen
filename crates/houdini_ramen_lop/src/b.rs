#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBackgroundplateQuicksetup {
    /// Quick Setups ↓
    QuickSetups = 0,
    /// Sample Compositing Network (Old COPs)
    SampleCompositingNetworkOldCops = 1,
    /// Sample Compositing Network (New COPs)
    SampleCompositingNetworkNewCops = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBackgroundplateShadows {
    Combined = 0,
    PerLpeTag = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBackgroundplateReflectionocclusion {
    Combined = 0,
    PerLpeTag = 1,
}

#[derive(Debug, Clone)]
pub struct LopBackgroundplate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopBackgroundplate {
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

    // --- Float parameters ---
    pub fn with_metallic(mut self, val: f32) -> Self {
        self.params.insert(
            "metallic".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_metallic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflect(mut self, val: f32) -> Self {
        self.params.insert(
            "reflect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ior(mut self, val: f32) -> Self {
        self.params.insert(
            "ior".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_offscreentex_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offscreentex_rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offscreentex_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreentex_rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offscreen_col(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offscreen_col".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offscreen_col_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreen_col".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_quicksetup(mut self, val: LopBackgroundplateQuicksetup) -> Self {
        self.params.insert(
            "quicksetup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quicksetup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quicksetup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadows(mut self, val: LopBackgroundplateShadows) -> Self {
        self.params.insert(
            "shadows".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadows".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflectionocclusion(mut self, val: LopBackgroundplateReflectionocclusion) -> Self {
        self.params.insert(
            "reflectionocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_reflectionocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectionocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matpathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "matpathprefix".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matpathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matpathprefix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_plate(mut self, val: &str) -> Self {
        self.params.insert(
            "plate".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_plate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offscreentex(mut self, val: &str) -> Self {
        self.params.insert(
            "offscreentex".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offscreentex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreentex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allowparmanim(mut self, val: bool) -> Self {
        self.params.insert(
            "allowparmanim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowparmanim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowparmanim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useoffscreentex(mut self, val: bool) -> Self {
        self.params.insert(
            "useoffscreentex".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useoffscreentex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useoffscreentex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emission(mut self, val: bool) -> Self {
        self.params.insert(
            "emission".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflection(mut self, val: bool) -> Self {
        self.params.insert(
            "reflection".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopBackgroundplate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "backgroundplate"
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

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("materiallibrary/background")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopBackgroundplateInnerExt {
    fn material_outputs_and_aovs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rx(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ry(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn siny(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cos_x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cos_y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cos_z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxcosy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxsiny(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxsinycosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxsinycosz_sinxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxsinysinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxsinysinz_sinxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosycosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cosysinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn kma_rayimport1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn kma_rayimport2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn material_properties(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxadd9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxasin1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxatan2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxcombine2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxcombine3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxcombine4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxcombine5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxcombine6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxcombine7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdisplacement(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdivide1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdotproduct1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdotproduct2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdotproduct3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdotproduct4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdotproduct5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxdotproduct6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxif_vx_greatereq_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxif_vx_greatereq_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxif_vy_greatereq_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxif_vy_greatereq_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxif_vz_greater_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxifequal1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxifequal2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlximage1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlximage2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxmultiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxmultiply10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxmultiply11(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxmultiply12(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxmultiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxnormalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxposition1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxseparate3v7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxstandard_surface(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxsubtract1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxswitch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mtlxtransformvector1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row1_const(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row1_transpose(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row2_const(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row2_transpose(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row3_const(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn row3_transpose(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sin_x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sin_y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sin_z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxcosy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxsiny(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxsinycosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxsinycosz_cosxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxsinysinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxsinysinz_cosxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sinxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subnetinput6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopBackgroundplateInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, LopBackgroundplate>
{
    fn material_outputs_and_aovs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/Material_Outputs_and_AOVs")
    }
    fn rx(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/_rx")
    }
    fn ry(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/_ry")
    }
    fn siny(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/_siny")
    }
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/const1")
    }
    fn const4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/const4")
    }
    fn const5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/const5")
    }
    fn cos_x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cos_x")
    }
    fn cos_y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cos_y")
    }
    fn cos_z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cos_z")
    }
    fn cosxcosy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxcosy")
    }
    fn cosxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxcosz")
    }
    fn cosxsiny(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxsiny")
    }
    fn cosxsinycosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxsinycosz")
    }
    fn cosxsinycosz_sinxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxsinycosz_sinxsinz")
    }
    fn cosxsinysinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxsinysinz")
    }
    fn cosxsinysinz_sinxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxsinysinz_sinxcosz")
    }
    fn cosxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosxsinz")
    }
    fn cosycosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosycosz")
    }
    fn cosysinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/cosysinz")
    }
    fn inputs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/inputs")
    }
    fn kma_rayimport1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/kma_rayimport1")
    }
    fn kma_rayimport2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/kma_rayimport2")
    }
    fn material_properties(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/material_properties")
    }
    fn mtlxadd1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd1")
    }
    fn mtlxadd2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd2")
    }
    fn mtlxadd3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd3")
    }
    fn mtlxadd4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd4")
    }
    fn mtlxadd5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd5")
    }
    fn mtlxadd6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd6")
    }
    fn mtlxadd8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd8")
    }
    fn mtlxadd9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxadd9")
    }
    fn mtlxasin1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxasin1")
    }
    fn mtlxatan2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxatan2")
    }
    fn mtlxcombine2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxcombine2")
    }
    fn mtlxcombine3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxcombine3")
    }
    fn mtlxcombine4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxcombine4")
    }
    fn mtlxcombine5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxcombine5")
    }
    fn mtlxcombine6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxcombine6")
    }
    fn mtlxcombine7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxcombine7")
    }
    fn mtlxdisplacement(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdisplacement")
    }
    fn mtlxdivide1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdivide1")
    }
    fn mtlxdotproduct1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdotproduct1")
    }
    fn mtlxdotproduct2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdotproduct2")
    }
    fn mtlxdotproduct3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdotproduct3")
    }
    fn mtlxdotproduct4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdotproduct4")
    }
    fn mtlxdotproduct5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdotproduct5")
    }
    fn mtlxdotproduct6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxdotproduct6")
    }
    fn mtlxif_vx_greatereq_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxif_vx_greatereq_0")
    }
    fn mtlxif_vx_greatereq_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxif_vx_greatereq_1")
    }
    fn mtlxif_vy_greatereq_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxif_vy_greatereq_0")
    }
    fn mtlxif_vy_greatereq_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxif_vy_greatereq_1")
    }
    fn mtlxif_vz_greater_0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxif_vz_greater_0")
    }
    fn mtlxifequal1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxifequal1")
    }
    fn mtlxifequal2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxifequal2")
    }
    fn mtlximage1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlximage1")
    }
    fn mtlximage2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlximage2")
    }
    fn mtlxmultiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxmultiply1")
    }
    fn mtlxmultiply10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxmultiply10")
    }
    fn mtlxmultiply11(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxmultiply11")
    }
    fn mtlxmultiply12(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxmultiply12")
    }
    fn mtlxmultiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxmultiply2")
    }
    fn mtlxnormalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxnormalize1")
    }
    fn mtlxposition1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxposition1")
    }
    fn mtlxseparate2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate2")
    }
    fn mtlxseparate3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3")
    }
    fn mtlxseparate3v1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v1")
    }
    fn mtlxseparate3v2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v2")
    }
    fn mtlxseparate3v3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v3")
    }
    fn mtlxseparate3v4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v4")
    }
    fn mtlxseparate3v5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v5")
    }
    fn mtlxseparate3v6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v6")
    }
    fn mtlxseparate3v7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxseparate3v7")
    }
    fn mtlxstandard_surface(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxstandard_surface")
    }
    fn mtlxsubtract1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxsubtract1")
    }
    fn mtlxswitch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxswitch2")
    }
    fn mtlxtransformvector1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/mtlxtransformvector1")
    }
    fn row1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row1")
    }
    fn row1_const(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row1_const")
    }
    fn row1_transpose(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row1_transpose")
    }
    fn row2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row2")
    }
    fn row2_const(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row2_const")
    }
    fn row2_transpose(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row2_transpose")
    }
    fn row3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row3")
    }
    fn row3_const(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row3_const")
    }
    fn row3_transpose(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/row3_transpose")
    }
    fn sin_x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sin_x")
    }
    fn sin_y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sin_y")
    }
    fn sin_z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sin_z")
    }
    fn sinxcosy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxcosy")
    }
    fn sinxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxcosz")
    }
    fn sinxsiny(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxsiny")
    }
    fn sinxsinycosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxsinycosz")
    }
    fn sinxsinycosz_cosxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxsinycosz_cosxsinz")
    }
    fn sinxsinysinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxsinysinz")
    }
    fn sinxsinysinz_cosxcosz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxsinysinz_cosxcosz")
    }
    fn sinxsinz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/sinxsinz")
    }
    fn subnetinput(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput")
    }
    fn subnetinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput1")
    }
    fn subnetinput2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput2")
    }
    fn subnetinput3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput3")
    }
    fn subnetinput4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput4")
    }
    fn subnetinput5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput5")
    }
    fn subnetinput6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materiallibrary/background/subnetinput6")
    }
}

#[derive(Debug, Clone)]
pub struct LopBakeskinning {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopBakeskinning {
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
}

impl houdini_ramen_core::types::HoudiniNode for LopBakeskinning {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bakeskinning"
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
pub enum LopBasiscurvesCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBasiscurvesXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBasiscurvesRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopBasiscurves {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopBasiscurves {
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

    // --- Float parameters ---
    pub fn with_xn_primvarsdisplayopacity_ycb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_xn_primvarsdisplaycolor_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopBasiscurvesCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopBasiscurvesXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopBasiscurvesRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
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
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type_control(mut self, val: &str) -> Self {
        self.params.insert(
            "type_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type(mut self, val: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis_control(mut self, val: &str) -> Self {
        self.params.insert(
            "basis_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basis_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis(mut self, val: &str) -> Self {
        self.params.insert(
            "basis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap_control(mut self, val: &str) -> Self {
        self.params.insert(
            "wrap_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrap_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap(mut self, val: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control(mut self, val: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
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
    pub fn with_doublesided(mut self, val: bool) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doublesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopBasiscurves {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "basiscurves"
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
pub struct LopBegincontextoptionsblock {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopBegincontextoptionsblock {
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

    // --- Toggle parameters ---
    pub fn with_layerbreak(mut self, val: bool) -> Self {
        self.params.insert(
            "layerbreak".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_layerbreak_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layerbreak".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopBegincontextoptionsblock {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "begincontextoptionsblock"
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
pub struct LopBlend {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopBlend {
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

    /// Connects to input 1: "Blend Towards This Stage"
    pub fn set_input_blend_towards_this_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Blend Towards This Stage" and specifies the output index of the target node.
    pub fn set_input_blend_towards_this_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "blend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopBlend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "blend"
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
pub enum LopBlendconstraintSourcetype {
    Primitives = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBlendconstraintTargettype {
    SameAsSource = 0,
    Primitives = 1,
    PointInstancer = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBlendconstraintMethod {
    Proportional = 0,
    Difference = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBlendconstraintRotblend {
    Euler = 0,
    /// Shortest Path (Linear)
    ShortestPathLinear = 1,
    /// Shortest Path (Fast Linear)
    ShortestPathFastLinear = 2,
    /// Euler (Raw Angles)
    EulerRawAngles = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopBlendconstraintMask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone)]
pub struct LopBlendconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopBlendconstraint {
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

    /// Connects to input 1: "Input Targets"
    pub fn set_input_input_targets<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input Targets" and specifies the output index of the target node.
    pub fn set_input_input_targets_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
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
    pub fn with_blend_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("blend{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("blend{}", index1),
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
    pub fn with_sourcetype(mut self, val: LopBlendconstraintSourcetype) -> Self {
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
    pub fn with_targettype(mut self, val: LopBlendconstraintTargettype) -> Self {
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
    pub fn with_method(mut self, val: LopBlendconstraintMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rotblend(mut self, val: LopBlendconstraintRotblend) -> Self {
        self.params.insert(
            "rotblend".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rotblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mask_inst(mut self, index1: usize, val: LopBlendconstraintMask) -> Self {
        self.params.insert(
            format!("mask{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mask{}", index1),
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
    pub fn with_targetinstances(mut self, val: &str) -> Self {
        self.params.insert(
            "targetinstances".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetinstances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetinstances".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for LopBlendconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "blendconstraint"
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
