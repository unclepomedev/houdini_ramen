#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRagdollrunexamplePreXform {
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
pub enum ObjectRagdollrunexampleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRagdollrunexampleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRagdollrunexampleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectRagdollrunexample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectRagdollrunexample {
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
    pub fn with_pre_xform(mut self, val: ObjectRagdollrunexamplePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectRagdollrunexampleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectRagdollrunexampleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectRagdollrunexampleUparmtype) -> Self {
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
}

impl crate::core::types::HoudiniNode for ObjectRagdollrunexample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ragdollrunexample"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub trait ObjectRagdollrunexampleInnerExt {
    fn cam1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn crowd_sim(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn crowdsource(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn envlight1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ground(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hlight1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn mocapbiped2_run(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn mocapbiped2_setup(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectRagdollrunexampleInnerExt
    for crate::core::graph::InnerGraph<'a, ObjectRagdollrunexample>
{
    fn cam1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cam1")
    }
    fn crowd_sim(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("crowd_sim")
    }
    fn crowdsource(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("crowdsource")
    }
    fn envlight1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("envlight1")
    }
    fn ground(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ground")
    }
    fn hlight1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("hlight1")
    }
    fn mocapbiped2_run(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("mocapbiped2_run")
    }
    fn mocapbiped2_setup(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("mocapbiped2_setup")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageOrient {
    Front = 0,
    Top = 1,
    Right = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimagePreXform {
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
pub enum ObjectRefimageUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageVmRaypredice {
    DisablePredicing = 0,
    FullPredicing = 1,
    PrecomputeBounds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageVmRenderpoints {
    NoPointRendering = 0,
    RenderOnlyPoints = 1,
    RenderUnconnectedPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageVmRenderpointsas {
    Spheres = 0,
    Circles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageVmCoving {
    DisableCoving = 0,
    /// Coving for displacement/sub-d
    CovingForDisplacementSubMinusD = 1,
    CovingForAllPrimitives = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRefimageVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectRefimage {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectRefimage {
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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "parent"
    pub fn set_input_parent<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_parent_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
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
    pub fn with_vm_volumefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_volumefilterwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_volumefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_volumefilterwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_shadingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_shadingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_shadingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_shadingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_flatness(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_flatness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_flatness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_flatness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_pointscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_pointscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_pointscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_pointscale".to_string(),
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

    // --- Int parameters ---
    pub fn with_frame(mut self, val: i32) -> Self {
        self.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_vm_raypredice(mut self, val: ObjectRefimageVmRaypredice) -> Self {
        self.params.insert(
            "vm_raypredice".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_raypredice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_raypredice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_renderpoints(mut self, val: ObjectRefimageVmRenderpoints) -> Self {
        self.params.insert(
            "vm_renderpoints".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_renderpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_renderpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_renderpointsas(mut self, val: ObjectRefimageVmRenderpointsas) -> Self {
        self.params.insert(
            "vm_renderpointsas".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_renderpointsas_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_renderpointsas".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_coving(mut self, val: ObjectRefimageVmCoving) -> Self {
        self.params.insert(
            "vm_coving".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_coving_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_coving".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_orient(mut self, val: ObjectRefimageOrient) -> Self {
        self.params.insert(
            "orient".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectRefimageXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectRefimageRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectRefimagePreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectRefimageUparmtype) -> Self {
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
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: ObjectRefimageViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectRefimageVportOnionskin) -> Self {
        self.params.insert(
            "vport_onionskin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_onionskin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_coppath(mut self, val: &str) -> Self {
        self.params.insert(
            "coppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copfile(mut self, val: &str) -> Self {
        self.params.insert(
            "copfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rendervisibility(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_rendervisibility".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_rendervisibility_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rendervisibility".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_categories(mut self, val: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_categories_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectmask(mut self, val: &str) -> Self {
        self.params.insert(
            "reflectmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflectmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractmask(mut self, val: &str) -> Self {
        self.params.insert(
            "refractmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lightmask(mut self, val: &str) -> Self {
        self.params.insert(
            "lightmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lightmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_volumefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_volumefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_volumefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_volumefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_geometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_geometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_geometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_geometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_materialoverride(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_materialoverride".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_materialoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_materialoverride".to_string(),
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
    pub fn with_renderable(mut self, val: bool) -> Self {
        self.params.insert(
            "renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecoppath(mut self, val: bool) -> Self {
        self.params.insert(
            "usecoppath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_vm_rendersubd(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rendersubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rendersubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rendersubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_matte(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_matte".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_matte_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_matte".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rayshade(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rayshade".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rayshade_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rayshade".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_velocityblur(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_velocityblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_velocityblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_velocityblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_curvesurface(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_curvesurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_curvesurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_curvesurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rmbackface(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rmbackface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rmbackface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rmbackface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_forcegeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_forcegeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_forcegeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_forcegeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rendersubdcurves(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rendersubdcurves".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rendersubdcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rendersubdcurves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_usenforpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_usenforpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_usenforpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_usenforpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_pscalediameter(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_pscalediameter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_pscalediameter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_pscalediameter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_metavolume(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_metavolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_metavolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_metavolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_overridedetail(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_overridedetail".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_overridedetail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_overridedetail".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_procuseroottransform(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_procuseroottransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_procuseroottransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_procuseroottransform".to_string(),
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
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectRefimage {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "refimage"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("copnet1")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectRefimageInnerExt {
    fn file1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectRefimageInnerExt for crate::core::graph::InnerGraph<'a, ObjectRefimage> {
    fn file1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copnet1/file1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmusclePreXform {
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
pub enum ObjectRiggedmuscleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleConcentricdriver {
    Off = 0,
    DrivenByLength = 1,
    DrivenByTension = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleBuildtype {
    /// Built-In Spheres
    BuiltMinusInSpheres = 0,
    /// Built-In Tet Mesh
    BuiltMinusInTetMesh = 1,
    /// Built-in Tube
    BuiltMinusInTube = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleTubetype {
    Polygon = 0,
    Nurbs = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleLocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleIntegratortype {
    Be1 = 0,
    Abe2 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleCollisiondetection {
    UseSolverDefault = 0,
    UseVolumeCollisions = 1,
    UseSurfaceCollisions = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRiggedmuscleStrokeasrig {
    TreatStrokeAsMuscle = 0,
    TreatStrokeAsRig = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectRiggedmuscle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectRiggedmuscle {
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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Origin Anchor"
    pub fn set_input_origin_anchor<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Origin Anchor" and specifies the output index of the target node.
    pub fn set_input_origin_anchor_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_origin_anchor_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "End Anchor"
    pub fn set_input_end_anchor<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "End Anchor" and specifies the output index of the target node.
    pub fn set_input_end_anchor_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_end_anchor_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_setcompressedlength(mut self) -> Self {
        self.params.insert(
            "setcompressedlength".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_setstretchedlength(mut self) -> Self {
        self.params.insert(
            "setstretchedlength".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_resethandles(mut self) -> Self {
        self.params.insert(
            "resethandles".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savecapturepose(mut self) -> Self {
        self.params.insert(
            "savecapturepose".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_exportcapturepose(mut self) -> Self {
        self.params.insert(
            "exportcapturepose".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dostroke(mut self) -> Self {
        self.params.insert(
            "dostroke".to_string(),
            crate::core::types::ParamValue::Button,
        );
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
    pub fn with_muscletension(mut self, val: f32) -> Self {
        self.params.insert(
            "muscletension".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscletension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscletension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isometricbias(mut self, val: f32) -> Self {
        self.params.insert(
            "isometricbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_isometricbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isometricbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclebiashigh(mut self, val: f32) -> Self {
        self.params.insert(
            "musclebiashigh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclebiashigh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclebiashigh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isometriccontraction(mut self, val: f32) -> Self {
        self.params.insert(
            "isometriccontraction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_isometriccontraction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isometriccontraction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclecontractionhigh(mut self, val: f32) -> Self {
        self.params.insert(
            "musclecontractionhigh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclecontractionhigh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclecontractionhigh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscletightness(mut self, val: f32) -> Self {
        self.params.insert(
            "muscletightness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscletightness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscletightness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscletightnesshigh(mut self, val: f32) -> Self {
        self.params.insert(
            "muscletightnesshigh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscletightnesshigh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscletightnesshigh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressedscale(mut self, val: f32) -> Self {
        self.params.insert(
            "compressedscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_compressedscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressedscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchedscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchedscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchedscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchedscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclehightensionscale(mut self, val: f32) -> Self {
        self.params.insert(
            "musclehightensionscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclehightensionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclehightensionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclelowtensionscale(mut self, val: f32) -> Self {
        self.params.insert(
            "musclelowtensionscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclelowtensionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclelowtensionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressedlength(mut self, val: f32) -> Self {
        self.params.insert(
            "compressedlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_compressedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressedlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchedlength(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchedlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchedlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_handlescale(mut self, val: f32) -> Self {
        self.params.insert(
            "handlescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_handlescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spherespacing(mut self, val: f32) -> Self {
        self.params.insert(
            "spherespacing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spherespacing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spherespacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_profilemin(mut self, val: f32) -> Self {
        self.params.insert(
            "profilemin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_profilemin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "profilemin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_profilemax(mut self, val: f32) -> Self {
        self.params.insert(
            "profilemax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_profilemax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "profilemax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclebasesize(mut self, val: f32) -> Self {
        self.params.insert(
            "musclebasesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclebasesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshsize(mut self, val: f32) -> Self {
        self.params.insert(
            "remeshsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_remeshsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remeshsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtetsize(mut self, val: f32) -> Self {
        self.params.insert(
            "maxtetsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtetsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxtetsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaleconst(mut self, val: f32) -> Self {
        self.params.insert(
            "scaleconst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scaleconst_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaleconst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalelocalfeature(mut self, val: f32) -> Self {
        self.params.insert(
            "scalelocalfeature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalelocalfeature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalelocalfeature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isodivsize(mut self, val: f32) -> Self {
        self.params.insert(
            "isodivsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_isodivsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isodivsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isooffset(mut self, val: f32) -> Self {
        self.params.insert(
            "isooffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_isooffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isooffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreinnercopyscale(mut self, val: f32) -> Self {
        self.params.insert(
            "coreinnercopyscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coreinnercopyscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coreinnercopyscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureremeshsize(mut self, val: f32) -> Self {
        self.params.insert(
            "captureremeshsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_captureremeshsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureremeshsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturesmoothing(mut self, val: f32) -> Self {
        self.params.insert(
            "capturesmoothing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_capturesmoothing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturesmoothing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturecoreradius(mut self, val: f32) -> Self {
        self.params.insert(
            "capturecoreradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_capturecoreradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturecoreradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureprofilemin(mut self, val: f32) -> Self {
        self.params.insert(
            "captureprofilemin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_captureprofilemin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureprofilemin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureprofilemax(mut self, val: f32) -> Self {
        self.params.insert(
            "captureprofilemax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_captureprofilemax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureprofilemax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggleamount(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggleamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggleamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggleamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stiff(mut self, val: f32) -> Self {
        self.params.insert(
            "stiff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damp(mut self, val: f32) -> Self {
        self.params.insert(
            "damp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limit(mut self, val: f32) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flex(mut self, val: f32) -> Self {
        self.params.insert(
            "flex".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggletimerange(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggletimerange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggletimerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggletimerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitlength(mut self, val: f32) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitmass(mut self, val: f32) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitmass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumestiffnessratio(mut self, val: f32) -> Self {
        self.params.insert(
            "volumestiffnessratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumestiffnessratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumestiffnessratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soliddampingratio".to_string(),
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
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdftol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdftol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdftol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdftol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anchorstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "anchorstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anchorstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anchorstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anchordamping(mut self, val: f32) -> Self {
        self.params.insert(
            "anchordamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anchordamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anchordamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headradius(mut self, val: f32) -> Self {
        self.params.insert(
            "headradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tailradius(mut self, val: f32) -> Self {
        self.params.insert(
            "tailradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tailradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tailradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffmaxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "falloffmaxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloffmaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloffmaxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclebiaslow(mut self, val: f32) -> Self {
        self.params.insert(
            "musclebiaslow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclebiaslow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclebiaslow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclecontractionlow(mut self, val: f32) -> Self {
        self.params.insert(
            "musclecontractionlow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclecontractionlow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclecontractionlow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscletightnesslow(mut self, val: f32) -> Self {
        self.params.insert(
            "muscletightnesslow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscletightnesslow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscletightnesslow".to_string(),
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
    pub fn with_muscleradiusscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscleradiusscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscleradiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleradiusscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos1t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos1t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos1t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos1t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos1r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos1r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos1r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos1r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos1s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos1s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos1s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos1s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos2t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos2t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos2t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos2t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos2r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos2r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos2r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos2r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos2s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos2s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos2s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos2s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos3t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos3t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos3t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos3t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos3r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos3r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos3r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos3r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos3s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos3s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos3s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos3s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos4t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos4t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos4t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos4t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos4r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos4r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos4r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos4r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos4s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos4s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos4s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos4s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos5t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos5t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos5t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos5t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos5r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos5r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos5r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos5r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos5s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos5s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos5s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos5s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sphereradius(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sphereradius".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sphereradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sphereradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos1t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos1t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos1t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos1t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos1r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos1r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos1r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos1r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos1s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos1s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos1s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos1s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos2t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos2t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos2t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos2t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos2r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos2r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos2r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos2r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos2s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos2s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos2s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos2s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos3t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos3t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos3t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos3t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos3r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos3r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos3r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos3r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos3s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos3s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos3s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos3s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos4t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos4t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos4t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos4t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos4r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos4r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos4r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos4r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos4s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos4s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos4s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos4s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos5t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos5t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos5t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos5t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos5r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos5r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos5r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos5r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capture_pos5s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capture_pos5s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capture_pos5s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capture_pos5s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturemuscleradiusscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capturemuscleradiusscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capturemuscleradiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturemuscleradiusscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturesphereradius(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capturesphereradius".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capturesphereradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturesphereradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturedisplaycolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "capturedisplaycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_capturedisplaycolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturedisplaycolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mult".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mult".to_string(),
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
    pub fn with_concentricdriver(mut self, val: ObjectRiggedmuscleConcentricdriver) -> Self {
        self.params.insert(
            "concentricdriver".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_concentricdriver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "concentricdriver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_buildtype(mut self, val: ObjectRiggedmuscleBuildtype) -> Self {
        self.params.insert(
            "buildtype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_buildtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buildtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tuberows(mut self, val: i32) -> Self {
        self.params.insert(
            "tuberows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tuberows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tuberows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tubecols(mut self, val: i32) -> Self {
        self.params.insert(
            "tubecols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tubecols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tubecols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_segs(mut self, val: i32) -> Self {
        self.params
            .insert("segs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_segs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "segs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresections(mut self, val: i32) -> Self {
        self.params.insert(
            "coresections".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_coresections_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coresections".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreinnercopies(mut self, val: i32) -> Self {
        self.params.insert(
            "coreinnercopies".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_coreinnercopies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coreinnercopies".to_string(),
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
    pub fn with_maxcollisionpasses(mut self, val: i32) -> Self {
        self.params.insert(
            "maxcollisionpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strokeasrig(mut self, val: ObjectRiggedmuscleStrokeasrig) -> Self {
        self.params.insert(
            "strokeasrig".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_strokeasrig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strokeasrig".to_string(),
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
    pub fn with_pre_xform(mut self, val: ObjectRiggedmusclePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectRiggedmuscleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectRiggedmuscleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectRiggedmuscleUparmtype) -> Self {
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
    pub fn with_tubetype(mut self, val: ObjectRiggedmuscleTubetype) -> Self {
        self.params.insert(
            "tubetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tubetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tubetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localscaling(mut self, val: ObjectRiggedmuscleLocalscaling) -> Self {
        self.params.insert(
            "localscaling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_localscaling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localscaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_integratortype(mut self, val: ObjectRiggedmuscleIntegratortype) -> Self {
        self.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_integratortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisiondetection(mut self, val: ObjectRiggedmuscleCollisiondetection) -> Self {
        self.params.insert(
            "collisiondetection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisiondetection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisiondetection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_profile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "profile".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_profile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "profile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "colorramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_colorramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstiffnessvramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "targetstiffnessvramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_targetstiffnessvramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstiffnessvramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innercorefalloff(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "innercorefalloff".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_innercorefalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "innercorefalloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_muscleid(mut self, val: &str) -> Self {
        self.params.insert(
            "muscleid".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_muscleid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtransform(mut self, val: &str) -> Self {
        self.params.insert(
            "outputtransform".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputtransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalgeopath(mut self, val: &str) -> Self {
        self.params.insert(
            "externalgeopath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalgeopath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalgeopath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalgeogroup(mut self, val: &str) -> Self {
        self.params.insert(
            "externalgeogroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalgeogroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalgeogroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidersoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "collidersoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collidersoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidersoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidergroup(mut self, val: &str) -> Self {
        self.params.insert(
            "collidergroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collidergroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidergroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "projectionpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_projectionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectionpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_biasdriver(mut self, val: bool) -> Self {
        self.params.insert(
            "biasdriver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_biasdriver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "biasdriver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contractiondriver(mut self, val: bool) -> Self {
        self.params.insert(
            "contractiondriver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_contractiondriver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contractiondriver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tightnessdriver(mut self, val: bool) -> Self {
        self.params.insert(
            "tightnessdriver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tightnessdriver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tightnessdriver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepposhandles(mut self, val: bool) -> Self {
        self.params.insert(
            "keepposhandles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepposhandles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepposhandles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_buildtypeexternal(mut self, val: bool) -> Self {
        self.params.insert(
            "buildtypeexternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_buildtypeexternal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buildtypeexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayexternalgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "displayexternalgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayexternalgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayexternalgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleusebasesize(mut self, val: bool) -> Self {
        self.params.insert(
            "muscleusebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscleusebasesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscleusebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hiquality(mut self, val: bool) -> Self {
        self.params.insert(
            "hiquality".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hiquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hiquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesdf(mut self, val: bool) -> Self {
        self.params.insert(
            "usesdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaycapturepose(mut self, val: bool) -> Self {
        self.params.insert(
            "displaycapturepose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaycapturepose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaycapturepose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaycapturegeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "displaycapturegeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaycapturegeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaycapturegeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usejiggle(mut self, val: bool) -> Self {
        self.params.insert(
            "usejiggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usejiggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usejiggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedynamics(mut self, val: bool) -> Self {
        self.params.insert(
            "usedynamics".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedynamics_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedynamics".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecollider(mut self, val: bool) -> Self {
        self.params.insert(
            "usecollider".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecollider_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecollider".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animategeo(mut self, val: bool) -> Self {
        self.params.insert(
            "animategeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_laserscan(mut self, val: bool) -> Self {
        self.params.insert(
            "laserscan".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_laserscan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "laserscan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fixsigns(mut self, val: bool) -> Self {
        self.params.insert(
            "fixsigns".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixsigns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fixsigns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcebounds(mut self, val: bool) -> Self {
        self.params.insert(
            "forcebounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcebounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invert(mut self, val: bool) -> Self {
        self.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayanchorradius(mut self, val: bool) -> Self {
        self.params.insert(
            "displayanchorradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayanchorradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayanchorradius".to_string(),
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
    pub fn with_displaymuscle(mut self, val: bool) -> Self {
        self.params.insert(
            "displaymuscle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaymuscle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaymuscle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayhandles(mut self, val: bool) -> Self {
        self.params.insert(
            "displayhandles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayhandles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayhandles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayinnercore(mut self, val: bool) -> Self {
        self.params.insert(
            "displayinnercore".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayinnercore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayinnercore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayaswires(mut self, val: bool) -> Self {
        self.params.insert(
            "displayaswires".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayaswires_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayaswires".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaysinglewire(mut self, val: bool) -> Self {
        self.params.insert(
            "displaysinglewire".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaysinglewire_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaysinglewire".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffwithpscale(mut self, val: bool) -> Self {
        self.params.insert(
            "falloffwithpscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_falloffwithpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloffwithpscale".to_string(),
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
}

impl crate::core::types::HoudiniNode for ObjectRiggedmuscle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "riggedmuscle"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub trait ObjectRiggedmuscleInnerExt {
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
    fn guide(&mut self) -> crate::core::graph::ExistingNodeRef;
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
    fn muscle(&mut self) -> crate::core::graph::ExistingNodeRef;
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

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectRiggedmuscleInnerExt for crate::core::graph::InnerGraph<'a, ObjectRiggedmuscle> {
    fn tightnessdriver(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("TightnessDriver")
    }
    fn blend_handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_handle1")
    }
    fn blend_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_handle2")
    }
    fn blend_jiggle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_jiggle")
    }
    fn blend_jiggle_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_jiggle_handle2")
    }
    fn blend_jiggle_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_jiggle_handle4")
    }
    fn blend_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_middle")
    }
    fn blend_tight1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_tight1")
    }
    fn blend_tight2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_tight2")
    }
    fn blend_tight3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend_tight3")
    }
    fn capture_handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("capture_handle1")
    }
    fn capture_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("capture_handle2")
    }
    fn capture_handle3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("capture_handle3")
    }
    fn capture_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("capture_handle4")
    }
    fn capture_handle5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("capture_handle5")
    }
    fn cv1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cv1")
    }
    fn cv2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cv2")
    }
    fn cv3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cv3")
    }
    fn cv4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cv4")
    }
    fn cv5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cv5")
    }
    fn guide(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("guide")
    }
    fn handle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("handle1")
    }
    fn handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("handle2")
    }
    fn handle3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("handle3")
    }
    fn handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("handle4")
    }
    fn handle5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("handle5")
    }
    fn head_radius(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("head_radius")
    }
    fn internal_core(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("internal_core")
    }
    fn jiggle_chop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("jiggle_chop")
    }
    fn jiggled_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("jiggled_handle2")
    }
    fn jiggled_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("jiggled_handle4")
    }
    fn jiggled_null(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("jiggled_null")
    }
    fn muscle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle")
    }
    fn stroke_container(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("stroke_container")
    }
    fn switch_to_capture_pose(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_to_capture_pose")
    }
    fn switch_to_capture_pose1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_to_capture_pose1")
    }
    fn switch_to_capture_pose2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_to_capture_pose2")
    }
    fn switch_to_capture_pose3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_to_capture_pose3")
    }
    fn switch_to_capture_pose4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_to_capture_pose4")
    }
    fn tailradius(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("tailradius")
    }
    fn unjiggled_handle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unjiggled_handle2")
    }
    fn unjiggled_handle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unjiggled_handle4")
    }
    fn unjiggled_null(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unjiggled_null")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRivetDisplayicon {
    Icon = 0,
    Axis = 1,
    IconAndAxis = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRivetControltype {
    Null = 0,
    Circles = 1,
    Box = 2,
    Planes = 3,
    NullAndCircles = 4,
    NullAndBox = 5,
    NullAndPlanes = 6,
    ControlSopInput = 7,
    InstancedSop = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRivetOrientation {
    AllPlanes = 0,
    YzPlane = 1,
    ZxPlane = 2,
    XyPlane = 3,
    /// YZ, ZX planes
    YzZxPlanes = 4,
    /// YZ, XY planes
    YzXyPlanes = 5,
    /// ZX, XY planes
    ZxXyPlanes = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRivetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRivetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRivetPreXform {
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
pub enum ObjectRivetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectRivet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectRivet {
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
    pub fn with_geoscale(mut self, val: f32) -> Self {
        self.params.insert(
            "geoscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_geoscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geoscale".to_string(),
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
    pub fn with_rivetweights(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rivetweights".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rivetweights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rivetweights".to_string(),
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
    pub fn with_geosize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "geosize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_geosize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geocenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "geocenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_geocenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geocenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "georotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_georotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "georotate".to_string(),
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
    pub fn with_displayicon(mut self, val: ObjectRivetDisplayicon) -> Self {
        self.params.insert(
            "displayicon".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_displayicon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayicon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_controltype(mut self, val: ObjectRivetControltype) -> Self {
        self.params.insert(
            "controltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_controltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "controltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientation(mut self, val: ObjectRivetOrientation) -> Self {
        self.params.insert(
            "orientation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orientation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectRivetXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectRivetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectRivetPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectRivetUparmtype) -> Self {
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
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_rivetsop(mut self, val: &str) -> Self {
        self.params.insert(
            "rivetsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rivetsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rivetsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rivetgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "rivetgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rivetgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rivetgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rivetxattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "rivetxattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rivetxattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rivetxattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rivetzattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "rivetzattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rivetzattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rivetzattrib".to_string(),
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
    pub fn with_geocustom(mut self, val: &str) -> Self {
        self.params.insert(
            "geocustom".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geocustom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geocustom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_rivetuseattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "rivetuseattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rivetuseattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rivetuseattribs".to_string(),
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
    pub fn with_shadedmode(mut self, val: bool) -> Self {
        self.params.insert(
            "shadedmode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadedmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadedmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_renderspace(mut self, val: bool) -> Self {
        self.params.insert(
            "renderspace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectRivet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "rivet"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub trait ObjectRivetInnerExt {
    fn control1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn point1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectRivetInnerExt for crate::core::graph::InnerGraph<'a, ObjectRivet> {
    fn control1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("control1")
    }
    fn point1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("point1")
    }
}

#[derive(Debug, Clone)]
pub struct ObjectRopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectRopnet {
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
}

impl crate::core::types::HoudiniNode for ObjectRopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ropnet"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
