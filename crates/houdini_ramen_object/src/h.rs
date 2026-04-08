#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenSourcemode {
    GroomObject = 0,
    GroomFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenPreXform {
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
pub enum ObjectHaircardgenUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenWidthmethod {
    SpecifyWidth = 0,
    ComputeUsingClusterDistribution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenLengthdivmethod {
    UniformSegmentCount = 0,
    SegmentLength = 1,
    Refine = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenUvmethod {
    Regular = 0,
    UvReferenceFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardgenUvregpackingmethod {
    SingleTile = 0,
    FixedScale = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectHaircardgen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectHaircardgen {
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

    /// Connects to input 0: "parent"
    pub fn set_input_parent<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mincurvelength(mut self, val: f32) -> Self {
        self.params.insert(
            "mincurvelength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mincurvelength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mincurvelength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clustercurveu(mut self, val: f32) -> Self {
        self.params.insert(
            "clustercurveu".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clustercurveu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clustercurveu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_snaprootsblenddist(mut self, val: f32) -> Self {
        self.params.insert(
            "snaprootsblenddist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_snaprootsblenddist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snaprootsblenddist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_snaprootsblendbias(mut self, val: f32) -> Self {
        self.params.insert(
            "snaprootsblendbias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_snaprootsblendbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snaprootsblendbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_widthscale(mut self, val: f32) -> Self {
        self.params.insert(
            "widthscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_widthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "widthscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_widthmax(mut self, val: f32) -> Self {
        self.params.insert(
            "widthmax".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_widthmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "widthmax".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extralength(mut self, val: f32) -> Self {
        self.params.insert(
            "extralength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extralength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extralength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lengthseg(mut self, val: f32) -> Self {
        self.params.insert(
            "lengthseg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthseg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lengthseg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lengthrefinedivs(mut self, val: f32) -> Self {
        self.params.insert(
            "lengthrefinedivs".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthrefinedivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lengthrefinedivs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_bendrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bendrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bendrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
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
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numcards(mut self, val: i32) -> Self {
        self.params.insert(
            "numcards".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numcards_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numcards".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clusterseed(mut self, val: i32) -> Self {
        self.params.insert(
            "clusterseed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_clusterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusterseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientsmoothiters(mut self, val: i32) -> Self {
        self.params.insert(
            "orientsmoothiters".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_orientsmoothiters_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientsmoothiters".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_widthdivs(mut self, val: i32) -> Self {
        self.params.insert(
            "widthdivs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_widthdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "widthdivs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lengthuniformdivs(mut self, val: i32) -> Self {
        self.params.insert(
            "lengthuniformdivs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lengthuniformdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lengthuniformdivs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvregpadding(mut self, val: i32) -> Self {
        self.params.insert(
            "uvregpadding".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uvregpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvregpadding".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sourcemode(mut self, val: ObjectHaircardgenSourcemode) -> Self {
        self.params.insert(
            "sourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectHaircardgenXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectHaircardgenRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectHaircardgenPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHaircardgenUparmtype) -> Self {
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
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_onionskin(mut self, val: ObjectHaircardgenVportOnionskin) -> Self {
        self.params.insert(
            "vport_onionskin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_onionskin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_widthmethod(mut self, val: ObjectHaircardgenWidthmethod) -> Self {
        self.params.insert(
            "widthmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_widthmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "widthmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lengthdivmethod(mut self, val: ObjectHaircardgenLengthdivmethod) -> Self {
        self.params.insert(
            "lengthdivmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lengthdivmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lengthdivmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvmethod(mut self, val: ObjectHaircardgenUvmethod) -> Self {
        self.params.insert(
            "uvmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uvmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvregpackingmethod(mut self, val: ObjectHaircardgenUvregpackingmethod) -> Self {
        self.params.insert(
            "uvregpackingmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uvregpackingmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvregpackingmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_widthramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "widthramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_widthramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "widthramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcegroomobject(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcegroomobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcegroomobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomfile(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcegroomfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcegroomfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
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
    pub fn with_clusterpointsop(mut self, val: &str) -> Self {
        self.params.insert(
            "clusterpointsop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clusterpointsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusterpointsop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvreffile(mut self, val: &str) -> Self {
        self.params.insert(
            "uvreffile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvreffile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvreffile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvrefchannel(mut self, val: &str) -> Self {
        self.params.insert(
            "uvrefchannel".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvrefchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvrefchannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hasinput(mut self, val: bool) -> Self {
        self.params.insert(
            "hasinput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_renderable(mut self, val: bool) -> Self {
        self.params.insert(
            "renderable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ignoreshortcurves(mut self, val: bool) -> Self {
        self.params.insert(
            "ignoreshortcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignoreshortcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignoreshortcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usecustomclusterpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usecustomclusterpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomclusterpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecustomclusterpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientsmooth(mut self, val: bool) -> Self {
        self.params.insert(
            "orientsmooth".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientsmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientsmooth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_snaproots(mut self, val: bool) -> Self {
        self.params.insert(
            "snaproots".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_snaproots_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snaproots".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvenable(mut self, val: bool) -> Self {
        self.params.insert(
            "uvenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uvenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectHaircardgen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "haircardgen"
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
        Some("modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHaircardgenInnerExt {
    fn hair(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHaircardgenInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectHaircardgen>
{
    fn hair(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("modify/HAIR")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardtexexampleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardtexexampleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHaircardtexexamplePreXform {
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
pub enum ObjectHaircardtexexampleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectHaircardtexexample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectHaircardtexexample {
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

    // --- Float parameters ---
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
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectHaircardtexexampleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectHaircardtexexampleRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectHaircardtexexamplePreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHaircardtexexampleUparmtype) -> Self {
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

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectHaircardtexexample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "haircardtexexample"
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
pub trait ObjectHaircardtexexampleInnerExt {
    fn emitters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn groom(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn haircardcam(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn hairgen(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn materials(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ropnet(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHaircardtexexampleInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectHaircardtexexample>
{
    fn emitters(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("emitters")
    }
    fn groom(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("groom")
    }
    fn haircardcam(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("haircardcam")
    }
    fn hairgen(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("hairgen")
    }
    fn materials(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materials")
    }
    fn ropnet(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ropnet")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenSourcemode {
    GroomObject = 0,
    GroomFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenVdbsource {
    FromSkinGeometry = 0,
    SopGeometry = 1,
    File = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenDensityoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenGuideblendmethod {
    LinearBlend = 0,
    ExtrudeAndBlend = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenInfluenceradiusoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenInfluencedecayoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenMaxguidecountoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenMaxguideangleoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenClumpcrossoveroverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenUnguidedlengthoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenDeformmethod {
    /// Per Point Capture (Legacy)
    PerPointCaptureLegacy = 0,
    UseGenerationWeights = 1,
    BarycentricWeights = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenAprunescreensizeunit {
    FractionOfScreenWidth = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenThicknessoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenSkinsubdmode {
    MatchSkinObject = 0,
    AlwaysOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenDisplacemode {
    /// Match Skin Shader (Only Supports Displace Along Normal)
    MatchSkinShaderOnlySupportsDisplaceAlongNormal = 0,
    /// Match Specified Shader (Only Supports Displace Along Normal)
    MatchSpecifiedShaderOnlySupportsDisplaceAlongNormal = 1,
    DisplaceAlongNormal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenVmRaypredice {
    DisablePredicing = 0,
    FullPredicing = 1,
    PrecomputeBounds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenVmRenderpoints {
    NoPointRendering = 0,
    RenderOnlyPoints = 1,
    RenderUnconnectedPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenVmRenderpointsas {
    Spheres = 0,
    Circles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenVmCoving {
    DisableCoving = 0,
    /// Coving for displacement/sub-d
    CovingForDisplacementSubMinusD = 1,
    CovingForAllPrimitives = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenPreXform {
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
pub enum ObjectHairgenXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHairgenViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone)]
pub struct ObjectHairgen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectHairgen {
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

    /// Connects to input 0: "Parent"
    pub fn set_input_parent<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Parent" and specifies the output index of the target node.
    pub fn set_input_parent_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Groom Data"
    pub fn set_input_groom_data<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Groom Data" and specifies the output index of the target node.
    pub fn set_input_groom_data_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_saveresthairfile(mut self) -> Self {
        self.params.insert(
            "saveresthairfile".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "seed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influenceradius(mut self, val: f32) -> Self {
        self.params.insert(
            "influenceradius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_influenceradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influenceradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influencedecay(mut self, val: f32) -> Self {
        self.params.insert(
            "influencedecay".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_influencedecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influencedecay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguideangle(mut self, val: f32) -> Self {
        self.params.insert(
            "maxguideangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxguideangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguideangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clumpcrossover(mut self, val: f32) -> Self {
        self.params.insert(
            "clumpcrossover".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clumpcrossover_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clumpcrossover".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedlength(mut self, val: f32) -> Self {
        self.params.insert(
            "unguidedlength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unguidedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedminlength(mut self, val: f32) -> Self {
        self.params.insert(
            "unguidedminlength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unguidedminlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedminlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expansionfactor(mut self, val: f32) -> Self {
        self.params.insert(
            "expansionfactor".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expansionfactor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expansionfactor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pruneratio(mut self, val: f32) -> Self {
        self.params.insert(
            "pruneratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pruneratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pruneratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunecamfocal(mut self, val: f32) -> Self {
        self.params.insert(
            "aprunecamfocal".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunecamfocal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunecamfocal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunecamaperture(mut self, val: f32) -> Self {
        self.params.insert(
            "aprunecamaperture".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunecamaperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunecamaperture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunenormscreensize(mut self, val: f32) -> Self {
        self.params.insert(
            "aprunenormscreensize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunenormscreensize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunenormscreensize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunepixelscreensize(mut self, val: f32) -> Self {
        self.params.insert(
            "aprunepixelscreensize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunepixelscreensize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunepixelscreensize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunescreenagg(mut self, val: f32) -> Self {
        self.params.insert(
            "aprunescreenagg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunescreenagg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunescreenagg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "thickness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displaceoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "displaceoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displaceoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaceoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacescale(mut self, val: f32) -> Self {
        self.params.insert(
            "displacescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacerefoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "displacerefoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacerefoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacerefoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacerefscale(mut self, val: f32) -> Self {
        self.params.insert(
            "displacerefscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacerefscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacerefscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_volumefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_volumefilterwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_volumefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_volumefilterwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_shadingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_shadingquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_shadingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_shadingquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_flatness(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_flatness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_flatness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_flatness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_pointscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_pointscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_pointscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_pointscale".to_string(),
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
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceframe(mut self, val: f32) -> Self {
        self.params.insert(
            "sourceframe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sourceframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_displaybboxcenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "displaybboxcenter".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaybboxcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaybboxcenter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displaybboxsize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "displaybboxsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaybboxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaybboxsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunecampos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "aprunecampos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_aprunecampos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunecampos".to_string(),
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
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

    // --- Float4 parameters ---
    pub fn with_aprunecamorient(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "aprunecamorient".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_aprunecamorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunecamorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_count(mut self, val: i32) -> Self {
        self.params.insert(
            "count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scatterrelaxiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "scatterrelaxiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scatterrelaxiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scatterrelaxiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidecount(mut self, val: i32) -> Self {
        self.params.insert(
            "maxguidecount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxguidecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguidecount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedsegments(mut self, val: i32) -> Self {
        self.params.insert(
            "unguidedsegments".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_unguidedsegments_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedsegments".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxsplitpasses(mut self, val: i32) -> Self {
        self.params.insert(
            "maxsplitpasses".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsplitpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxsplitpasses".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgeflippasses(mut self, val: i32) -> Self {
        self.params.insert(
            "edgeflippasses".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_edgeflippasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgeflippasses".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecoverage(mut self, val: i32) -> Self {
        self.params.insert(
            "guidecoverage".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guidecoverage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecoverage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidesegs(mut self, val: i32) -> Self {
        self.params.insert(
            "maxguidesegs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxguidesegs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguidesegs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expansioniters(mut self, val: i32) -> Self {
        self.params.insert(
            "expansioniters".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_expansioniters_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expansioniters".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunecamxres(mut self, val: i32) -> Self {
        self.params.insert(
            "aprunecamxres".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_aprunecamxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunecamxres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunepruneseed(mut self, val: i32) -> Self {
        self.params.insert(
            "aprunepruneseed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_aprunepruneseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunepruneseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_raypredice(mut self, val: ObjectHairgenVmRaypredice) -> Self {
        self.params.insert(
            "vm_raypredice".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_raypredice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_raypredice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_renderpoints(mut self, val: ObjectHairgenVmRenderpoints) -> Self {
        self.params.insert(
            "vm_renderpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_renderpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_renderpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_renderpointsas(mut self, val: ObjectHairgenVmRenderpointsas) -> Self {
        self.params.insert(
            "vm_renderpointsas".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_renderpointsas_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_renderpointsas".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_coving(mut self, val: ObjectHairgenVmCoving) -> Self {
        self.params.insert(
            "vm_coving".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_coving_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_coving".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cooking_in_engine(mut self, val: i32) -> Self {
        self.params.insert(
            "cooking_in_engine".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cooking_in_engine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cooking_in_engine".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sourcemode(mut self, val: ObjectHairgenSourcemode) -> Self {
        self.params.insert(
            "sourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbsource(mut self, val: ObjectHairgenVdbsource) -> Self {
        self.params.insert(
            "vdbsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vdbsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_densityoverride(mut self, val: ObjectHairgenDensityoverride) -> Self {
        self.params.insert(
            "densityoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_densityoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densityoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideblendmethod(mut self, val: ObjectHairgenGuideblendmethod) -> Self {
        self.params.insert(
            "guideblendmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guideblendmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideblendmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influenceradiusoverride(
        mut self,
        val: ObjectHairgenInfluenceradiusoverride,
    ) -> Self {
        self.params.insert(
            "influenceradiusoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_influenceradiusoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influenceradiusoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influencedecayoverride(mut self, val: ObjectHairgenInfluencedecayoverride) -> Self {
        self.params.insert(
            "influencedecayoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_influencedecayoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influencedecayoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidecountoverride(mut self, val: ObjectHairgenMaxguidecountoverride) -> Self {
        self.params.insert(
            "maxguidecountoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxguidecountoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguidecountoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguideangleoverride(mut self, val: ObjectHairgenMaxguideangleoverride) -> Self {
        self.params.insert(
            "maxguideangleoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxguideangleoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguideangleoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clumpcrossoveroverride(mut self, val: ObjectHairgenClumpcrossoveroverride) -> Self {
        self.params.insert(
            "clumpcrossoveroverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clumpcrossoveroverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clumpcrossoveroverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedlengthoverride(mut self, val: ObjectHairgenUnguidedlengthoverride) -> Self {
        self.params.insert(
            "unguidedlengthoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_unguidedlengthoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedlengthoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deformmethod(mut self, val: ObjectHairgenDeformmethod) -> Self {
        self.params.insert(
            "deformmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deformmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deformmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunescreensizeunit(mut self, val: ObjectHairgenAprunescreensizeunit) -> Self {
        self.params.insert(
            "aprunescreensizeunit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_aprunescreensizeunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunescreensizeunit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessoverride(mut self, val: ObjectHairgenThicknessoverride) -> Self {
        self.params.insert(
            "thicknessoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_thicknessoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_skinsubdmode(mut self, val: ObjectHairgenSkinsubdmode) -> Self {
        self.params.insert(
            "skinsubdmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_skinsubdmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skinsubdmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacemode(mut self, val: ObjectHairgenDisplacemode) -> Self {
        self.params.insert(
            "displacemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_displacemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectHairgenPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectHairgenXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectHairgenRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectHairgenUparmtype) -> Self {
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
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_onionskin(mut self, val: ObjectHairgenVportOnionskin) -> Self {
        self.params.insert(
            "vport_onionskin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_onionskin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: ObjectHairgenViewportlod) -> Self {
        self.params.insert(
            "viewportlod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewportlod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_thicknessramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "thicknessramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_thicknessramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_description(mut self, val: &str) -> Self {
        self.params.insert(
            "description".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_description_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "description".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomobject(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcegroomobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcegroomobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomfile(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcegroomfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcegroomfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcegroomfile".to_string(),
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
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbsop(mut self, val: &str) -> Self {
        self.params.insert(
            "vdbsop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbsop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbsopgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "vdbsopgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbsopgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbsopgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbfile(mut self, val: &str) -> Self {
        self.params.insert(
            "vdbfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbfilegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "vdbfilegroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vdbfilegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbfilegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_densityattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "densityattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_densityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densityattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_densitytexture(mut self, val: &str) -> Self {
        self.params.insert(
            "densitytexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_densitytexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densitytexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "guidegroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influenceradiusattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "influenceradiusattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influenceradiusattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influenceradiusattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influenceradiustexture(mut self, val: &str) -> Self {
        self.params.insert(
            "influenceradiustexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influenceradiustexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influenceradiustexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influencedecayattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "influencedecayattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influencedecayattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influencedecayattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influencedecaytexture(mut self, val: &str) -> Self {
        self.params.insert(
            "influencedecaytexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_influencedecaytexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "influencedecaytexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidecountattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "maxguidecountattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidecountattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguidecountattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidecounttexture(mut self, val: &str) -> Self {
        self.params.insert(
            "maxguidecounttexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguidecounttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguidecounttexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguideangleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "maxguideangleattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguideangleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguideangleattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguideangletexture(mut self, val: &str) -> Self {
        self.params.insert(
            "maxguideangletexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxguideangletexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxguideangletexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clumpcrossoverattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "clumpcrossoverattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clumpcrossoverattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clumpcrossoverattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clumpcrossovertexture(mut self, val: &str) -> Self {
        self.params.insert(
            "clumpcrossovertexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clumpcrossovertexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clumpcrossovertexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initdirattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "initdirattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initdirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initdirattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedlengthattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "unguidedlengthattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedlengthattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedlengthattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedlengthtexture(mut self, val: &str) -> Self {
        self.params.insert(
            "unguidedlengthtexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedlengthtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedlengthtexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resthairfile(mut self, val: &str) -> Self {
        self.params.insert(
            "resthairfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resthairfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resthairfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xformattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "xformattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xformattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunecam(mut self, val: &str) -> Self {
        self.params.insert(
            "aprunecam".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprunecam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprunecam".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "thicknessattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknesstexture(mut self, val: &str) -> Self {
        self.params.insert(
            "thicknesstexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknesstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknesstexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessramp_basis(mut self, val: &str) -> Self {
        self.params.insert(
            "thicknessramp_basis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessramp_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessramp_basis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessramp_keys(mut self, val: &str) -> Self {
        self.params.insert(
            "thicknessramp_keys".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessramp_keys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessramp_keys".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessramp_values(mut self, val: &str) -> Self {
        self.params.insert(
            "thicknessramp_values".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessramp_values_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessramp_values".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "pointattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vertattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "vertattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vertattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vertattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "primattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_detailattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "detailattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_detailattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detailattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidepointattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "guidepointattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidepointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidepointattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideprimattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "guideprimattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideprimattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppointattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keeppointattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppointattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepvertattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepvertattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepvertattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepvertattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepprimattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepprimattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepprimattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepdetailattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "keepdetailattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepdetailattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepdetailattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacerefshader(mut self, val: &str) -> Self {
        self.params.insert(
            "displacerefshader".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacerefshader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacerefshader".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacetex(mut self, val: &str) -> Self {
        self.params.insert(
            "displacetex".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacetex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacetex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displaceskinshader(mut self, val: &str) -> Self {
        self.params.insert(
            "displaceskinshader".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displaceskinshader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaceskinshader".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacereftex(mut self, val: &str) -> Self {
        self.params.insert(
            "displacereftex".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacereftex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacereftex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_auto_engine_procedural(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_auto_engine_procedural".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_auto_engine_procedural_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_auto_engine_procedural".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_auto_engine_boundsop(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_auto_engine_boundsop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_auto_engine_boundsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_auto_engine_boundsop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_rendervisibility(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_rendervisibility".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_rendervisibility_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rendervisibility".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflectmask(mut self, val: &str) -> Self {
        self.params.insert(
            "reflectmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflectmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refractmask(mut self, val: &str) -> Self {
        self.params.insert(
            "refractmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refractmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_categories(mut self, val: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_categories_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lightmask(mut self, val: &str) -> Self {
        self.params.insert(
            "lightmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lightmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_volumefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_volumefilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_volumefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_volumefilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_materialoverride(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_materialoverride".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_materialoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_materialoverride".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hasinput(mut self, val: bool) -> Self {
        self.params.insert(
            "hasinput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_renderguides(mut self, val: bool) -> Self {
        self.params.insert(
            "renderguides".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderguides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderguides".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useanim(mut self, val: bool) -> Self {
        self.params.insert(
            "useanim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useanim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forcecount(mut self, val: bool) -> Self {
        self.params.insert(
            "forcecount".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcecount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useguides(mut self, val: bool) -> Self {
        self.params.insert(
            "useguides".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useguides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useguides".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniformguidesegments(mut self, val: bool) -> Self {
        self.params.insert(
            "uniformguidesegments".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniformguidesegments_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformguidesegments".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_skininterp(mut self, val: bool) -> Self {
        self.params.insert(
            "skininterp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skininterp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skininterp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_growunguided(mut self, val: bool) -> Self {
        self.params.insert(
            "growunguided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_growunguided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "growunguided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useinitdirattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "useinitdirattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useinitdirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useinitdirattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "displayassubd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayassubd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_staticgen(mut self, val: bool) -> Self {
        self.params.insert(
            "staticgen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_staticgen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticgen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_splitclumps(mut self, val: bool) -> Self {
        self.params.insert(
            "splitclumps".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splitclumps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitclumps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singleguide(mut self, val: bool) -> Self {
        self.params.insert(
            "singleguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeradius(mut self, val: bool) -> Self {
        self.params.insert(
            "computeradius".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitguidesegs(mut self, val: bool) -> Self {
        self.params.insert(
            "limitguidesegs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitguidesegs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitguidesegs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expandradius(mut self, val: bool) -> Self {
        self.params.insert(
            "expandradius".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadresthairfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "loadresthairfromdisk".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadresthairfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadresthairfromdisk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bypasssopnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "bypasssopnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bypasssopnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bypasssopnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limittobbox(mut self, val: bool) -> Self {
        self.params.insert(
            "limittobbox".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limittobbox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limittobbox".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablepruning(mut self, val: bool) -> Self {
        self.params.insert(
            "enablepruning".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepruning_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablepruning".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prunethicken(mut self, val: bool) -> Self {
        self.params.insert(
            "prunethicken".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prunethicken_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prunethicken".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prunestable(mut self, val: bool) -> Self {
        self.params.insert(
            "prunestable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prunestable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prunestable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aprune(mut self, val: bool) -> Self {
        self.params.insert(
            "aprune".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aprune_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aprune".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_skinsubd(mut self, val: bool) -> Self {
        self.params.insert(
            "skinsubd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skinsubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skinsubd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_skinsubdenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "skinsubdenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skinsubdenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skinsubdenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displace(mut self, val: bool) -> Self {
        self.params.insert(
            "displace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_rendersubd(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rendersubd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rendersubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rendersubd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_matte(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_matte".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_matte_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_matte".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_rayshade(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rayshade".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rayshade_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rayshade".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geo_velocityblur(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_velocityblur".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_velocityblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_velocityblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_curvesurface(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_curvesurface".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_curvesurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_curvesurface".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_rmbackface(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rmbackface".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rmbackface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rmbackface".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_forcegeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_forcegeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_forcegeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_forcegeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_rendersubdcurves(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_rendersubdcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rendersubdcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_rendersubdcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_usenforpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_usenforpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_usenforpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_usenforpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_pscalediameter(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_pscalediameter".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_pscalediameter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_pscalediameter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_metavolume(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_metavolume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_metavolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_metavolume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_overridedetail(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_overridedetail".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_overridedetail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_overridedetail".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_procuseroottransform(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_procuseroottransform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_procuseroottransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_procuseroottransform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_renderable(mut self, val: bool) -> Self {
        self.params.insert(
            "renderable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayhairshaded(mut self, val: bool) -> Self {
        self.params.insert(
            "displayhairshaded".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayhairshaded_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayhairshaded".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectHairgen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairgen"
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
        Some("modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHairgenInnerExt {
    fn hairs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHairgenInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectHairgen> {
    fn hairs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("modify/HAIRS")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHandleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHandleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHandlePreXform {
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
pub enum ObjectHandleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectHandle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectHandle {
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

    /// Connects to input 0: "parent"
    pub fn set_input_parent<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_weight(mut self, val: f32) -> Self {
        self.params.insert(
            "weight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "falloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
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
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_lrx(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "lrx".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lrx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lrx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lry(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "lry".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lrz(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "lrz".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lrz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lrz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectHandleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectHandleRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectHandlePreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHandleUparmtype) -> Self {
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

    // --- String parameters ---
    pub fn with_obj_targetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "obj_targetpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_obj_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "obj_targetpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_twistonly(mut self, val: bool) -> Self {
        self.params.insert(
            "twistonly".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twistonly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twistonly".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dorxlimit(mut self, val: bool) -> Self {
        self.params.insert(
            "dorxlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorxlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorxlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dorylimit(mut self, val: bool) -> Self {
        self.params.insert(
            "dorylimit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorylimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dorzlimit(mut self, val: bool) -> Self {
        self.params.insert(
            "dorzlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorzlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorzlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectHandle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "handle"
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
pub trait ObjectHandleInnerExt {
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn box1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHandleInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectHandle> {
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add1")
    }
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add2")
    }
    fn box1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("box1")
    }
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn merge2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("merge2")
    }
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch2")
    }
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("x")
    }
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform1")
    }
    fn xform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform2")
    }
    fn xform6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform6")
    }
    fn y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("y")
    }
    fn z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("z")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightPreXform {
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
pub enum ObjectHlightUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightLightType {
    Point = 0,
    Line = 1,
    Grid = 2,
    Disk = 3,
    Sphere = 4,
    Tube = 5,
    Geometry = 6,
    Distant = 7,
    Sun = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightAttenType {
    NoAttenuation = 0,
    HalfDistanceAttenuation = 1,
    PhysicallyCorrect = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightShadowType {
    NoShadows = 0,
    /// Ray-Traced Shadows
    RayMinusTracedShadows = 1,
    DepthMapShadows = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightShadowmapResmenu {
    /// Tiny (128x128)
    Tiny128x128 = 0,
    /// Small (256x256)
    Small256x256 = 1,
    /// Normal (512x512)
    Normal512x512 = 2,
    /// Large (1024x1024)
    Large1024x1024 = 3,
    /// Huge (4096)
    Huge4096 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectHlightProjection2 {
    Perspective = 0,
    Orthographic = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectHlight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectHlight {
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

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "light_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "light_exposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_exposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_samplingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_samplingquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_samplingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_samplingquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_fov(mut self, val: f32) -> Self {
        self.params.insert(
            "light_fov".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_fov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_fov".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_conedelta(mut self, val: f32) -> Self {
        self.params.insert(
            "conedelta".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conedelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conedelta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coneroll(mut self, val: f32) -> Self {
        self.params.insert(
            "coneroll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coneroll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_near(mut self, val: f32) -> Self {
        self.params.insert(
            "near".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "near".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_far(mut self, val: f32) -> Self {
        self.params.insert(
            "far".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "far".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamapblur(mut self, val: f32) -> Self {
        self.params.insert(
            "areamapblur".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_areamapblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areamapblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamapscale(mut self, val: f32) -> Self {
        self.params.insert(
            "areamapscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_areamapscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areamapscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "edgewidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgewidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgerolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "edgerolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgerolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgerolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_shadingfactor(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_shadingfactor".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_shadingfactor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_shadingfactor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orthowidth(mut self, val: f32) -> Self {
        self.params.insert(
            "orthowidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_orthowidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orthowidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_envangle(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_envangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_envangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_envangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atten_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "atten_dist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_dist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atten_start(mut self, val: f32) -> Self {
        self.params.insert(
            "atten_start".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atten_rampstart(mut self, val: f32) -> Self {
        self.params.insert(
            "atten_rampstart".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_rampstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_rampstart".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atten_rampend(mut self, val: f32) -> Self {
        self.params.insert(
            "atten_rampend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_rampend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_rampend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "shadow_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_intensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "shadow_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "shadow_quality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_quality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "shadow_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "shadow_blur".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_blur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iconscale(mut self, val: f32) -> Self {
        self.params.insert(
            "iconscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iconscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iconscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_dimmer(mut self, val: f32) -> Self {
        self.params.insert(
            "l_dimmer".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_dimmer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_dimmer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dimmer(mut self, val: f32) -> Self {
        self.params.insert(
            "dimmer".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dimmer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dimmer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focal(mut self, val: f32) -> Self {
        self.params.insert(
            "focal".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aperture(mut self, val: f32) -> Self {
        self.params.insert(
            "aperture".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focus(mut self, val: f32) -> Self {
        self.params.insert(
            "focus".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focus".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_areasize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "areasize".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_areasize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areasize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "light_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shadow_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadow_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
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
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "pc_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pc_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pc_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmap_samples(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "shadowmap_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_shadowmap_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmap_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectHlightXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectHlightRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectHlightPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHlightUparmtype) -> Self {
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
    pub fn with_light_type(mut self, val: ObjectHlightLightType) -> Self {
        self.params.insert(
            "light_type".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_light_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atten_type(mut self, val: ObjectHlightAttenType) -> Self {
        self.params.insert(
            "atten_type".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_atten_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_type(mut self, val: ObjectHlightShadowType) -> Self {
        self.params.insert(
            "shadow_type".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadow_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmap_resmenu(mut self, val: ObjectHlightShadowmapResmenu) -> Self {
        self.params.insert(
            "shadowmap_resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadowmap_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmap_resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection2(mut self, val: ObjectHlightProjection2) -> Self {
        self.params.insert(
            "projection2".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_projection2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_attenramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "attenramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_attenramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_contribname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("light_contribname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_contribname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("light_contribname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_categories(mut self, val: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_categories_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "categories".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_lpetag(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_lpetag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_lpetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_lpetag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projmap(mut self, val: &str) -> Self {
        self.params.insert(
            "projmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamap(mut self, val: &str) -> Self {
        self.params.insert(
            "areamap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areamap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamapspace(mut self, val: &str) -> Self {
        self.params.insert(
            "areamapspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamapspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areamapspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamapnull(mut self, val: &str) -> Self {
        self.params.insert(
            "areamapnull".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areamapnull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areamapnull".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "light_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areageometry(mut self, val: &str) -> Self {
        self.params.insert(
            "areageometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areageometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areageometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_file(mut self, val: &str) -> Self {
        self.params.insert(
            "pc_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pc_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "pc_camera".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pc_camera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmask(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowcategories(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowcategories".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowcategories_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowcategories".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmap_file(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowmap_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmap_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmap_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_winmask(mut self, val: &str) -> Self {
        self.params.insert(
            "winmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_winmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_wrangler(mut self, val: &str) -> Self {
        self.params.insert(
            "light_wrangler".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_wrangler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_wrangler".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "light_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ogl_enablelight(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_enablelight".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_enablelight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_enablelight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_contribenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("light_contribenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_contribenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("light_contribenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_contribprimary(mut self, val: bool) -> Self {
        self.params.insert(
            "light_contribprimary".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_contribprimary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_contribprimary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_coneenable(mut self, val: bool) -> Self {
        self.params.insert(
            "coneenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coneenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coneenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_light_conefov(mut self, val: bool) -> Self {
        self.params.insert(
            "light_conefov".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_conefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_conefov".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normalizearea(mut self, val: bool) -> Self {
        self.params.insert(
            "normalizearea".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalizearea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalizearea".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_singlesided(mut self, val: bool) -> Self {
        self.params.insert(
            "singlesided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singlesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singlesided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reverse(mut self, val: bool) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharpspot(mut self, val: bool) -> Self {
        self.params.insert(
            "sharpspot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sharpspot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharpspot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_selfshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "selfshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_selfshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selfshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "edgeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_edgeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_intothisobject(mut self, val: bool) -> Self {
        self.params.insert(
            "intothisobject".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_intothisobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intothisobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "pc_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pc_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pc_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_render_pointcloud(mut self, val: bool) -> Self {
        self.params.insert(
            "render_pointcloud".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_render_pointcloud_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "render_pointcloud".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pc_camera_override(mut self, val: bool) -> Self {
        self.params.insert(
            "pc_camera_override".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pc_camera_override_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pc_camera_override".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activeradiusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "activeradiusenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_activeradiusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradiusenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attenrampenable(mut self, val: bool) -> Self {
        self.params.insert(
            "attenrampenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenrampenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenrampenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_transparent(mut self, val: bool) -> Self {
        self.params.insert(
            "shadow_transparent".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_transparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_transparent".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_render_shadowmap(mut self, val: bool) -> Self {
        self.params.insert(
            "render_shadowmap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_render_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "render_shadowmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowmotionblur(mut self, val: bool) -> Self {
        self.params.insert(
            "shadowmotionblur".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadowmotionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmotionblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadow_autofit(mut self, val: bool) -> Self {
        self.params.insert(
            "shadow_autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_autofit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clampprojmap(mut self, val: bool) -> Self {
        self.params.insert(
            "clampprojmap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clampprojmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clampprojmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_areafullsphere(mut self, val: bool) -> Self {
        self.params.insert(
            "areafullsphere".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_areafullsphere_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areafullsphere".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectHlight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hlight"
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
pub trait ObjectHlightInnerExt {
    fn origin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn areashape1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn carve1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn circle2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn circle4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cone_light1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn constraints(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn convert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn copy1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn copy2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn facet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn grid2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn grid4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn grid5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn line5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn line6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn line7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn object_merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn point_light1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sphere1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sphere2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tube1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tube2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHlightInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectHlight> {
    fn origin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Origin")
    }
    fn areashape1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("areashape1")
    }
    fn carve1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("carve1")
    }
    fn circle2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("circle2")
    }
    fn circle4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("circle4")
    }
    fn cone_light1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cone_light1")
    }
    fn constraints(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("constraints")
    }
    fn convert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("convert1")
    }
    fn copy1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("copy1")
    }
    fn copy2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("copy2")
    }
    fn facet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("facet1")
    }
    fn grid2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("grid2")
    }
    fn grid4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("grid4")
    }
    fn grid5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("grid5")
    }
    fn line5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("line5")
    }
    fn line6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("line6")
    }
    fn line7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("line7")
    }
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn merge3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("merge3")
    }
    fn object_merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("object_merge1")
    }
    fn point_light1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("point_light1")
    }
    fn sphere1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sphere1")
    }
    fn sphere2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sphere2")
    }
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn tube1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tube1")
    }
    fn tube2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tube2")
    }
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform1")
    }
    fn xform3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform3")
    }
    fn xform4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform4")
    }
    fn xform5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform5")
    }
    fn xform6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform6")
    }
    fn xform7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform7")
    }
}
