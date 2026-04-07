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
    pub base: crate::core::types::NodeBase,
}

impl ObjectHaircardgen {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "parent"
    pub fn set_input_parent(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mincurvelength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mincurvelength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mincurvelength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mincurvelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clustercurveu(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clustercurveu".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clustercurveu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clustercurveu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snaprootsblenddist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "snaprootsblenddist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_snaprootsblenddist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snaprootsblenddist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snaprootsblendbias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "snaprootsblendbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_snaprootsblendbias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snaprootsblendbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "widthscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_widthscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "widthmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_widthmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extralength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "extralength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extralength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extralength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthseg(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengthseg".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthseg_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthseg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthrefinedivs(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengthrefinedivs".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthrefinedivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthrefinedivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_bendrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "bendrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bendrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bendrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numcards(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numcards".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numcards_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numcards".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusterseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "clusterseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_clusterseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clusterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientsmoothiters(mut self, val: i32) -> Self {
        self.base.params.insert(
            "orientsmoothiters".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_orientsmoothiters_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientsmoothiters".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthdivs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "widthdivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_widthdivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthdivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthuniformdivs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "lengthuniformdivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lengthuniformdivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthuniformdivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvregpadding(mut self, val: i32) -> Self {
        self.base.params.insert(
            "uvregpadding".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uvregpadding_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvregpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sourcemode(mut self, val: ObjectHaircardgenSourcemode) -> Self {
        self.base.params.insert(
            "sourcemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectHaircardgenXord) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectHaircardgenRord) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectHaircardgenPreXform) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHaircardgenUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_onionskin(mut self, val: ObjectHaircardgenVportOnionskin) -> Self {
        self.base.params.insert(
            "vport_onionskin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_onionskin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthmethod(mut self, val: ObjectHaircardgenWidthmethod) -> Self {
        self.base.params.insert(
            "widthmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_widthmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthdivmethod(mut self, val: ObjectHaircardgenLengthdivmethod) -> Self {
        self.base.params.insert(
            "lengthdivmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lengthdivmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthdivmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvmethod(mut self, val: ObjectHaircardgenUvmethod) -> Self {
        self.base.params.insert(
            "uvmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uvmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvregpackingmethod(mut self, val: ObjectHaircardgenUvregpackingmethod) -> Self {
        self.base.params.insert(
            "uvregpackingmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uvregpackingmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvregpackingmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_widthramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "widthramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_widthramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcegroomobject(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegroomobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroomobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegroomobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcegroomfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegroomfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroomfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegroomfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusterpointsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "clusterpointsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clusterpointsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clusterpointsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvreffile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "uvreffile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvreffile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvreffile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvrefchannel(mut self, val: &str) -> Self {
        self.base.params.insert(
            "uvrefchannel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvrefchannel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvrefchannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hasinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hasinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hasinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "renderable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignoreshortcurves(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ignoreshortcurves".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignoreshortcurves_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ignoreshortcurves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecustomclusterpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecustomclusterpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomclusterpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecustomclusterpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientsmooth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "orientsmooth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientsmooth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientsmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snaproots(mut self, val: bool) -> Self {
        self.base.params.insert(
            "snaproots".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_snaproots_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snaproots".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uvenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uvenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectHaircardgen {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "haircardgen"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHaircardgenInnerExt {
    fn hair(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHaircardgenInnerExt for crate::core::graph::InnerGraph<'a, ObjectHaircardgen> {
    fn hair(&mut self) -> crate::core::graph::ExistingNodeRef {
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
    pub base: crate::core::types::NodeBase,
}

impl ObjectHaircardtexexample {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectHaircardtexexampleXord) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectHaircardtexexampleRord) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectHaircardtexexamplePreXform) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHaircardtexexampleUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.base.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectHaircardtexexample {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "haircardtexexample"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHaircardtexexampleInnerExt {
    fn emitters(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn groom(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn haircardcam(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hairgen(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ropnet(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHaircardtexexampleInnerExt
    for crate::core::graph::InnerGraph<'a, ObjectHaircardtexexample>
{
    fn emitters(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("emitters")
    }
    fn groom(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groom")
    }
    fn haircardcam(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("haircardcam")
    }
    fn hairgen(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("hairgen")
    }
    fn materials(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("materials")
    }
    fn ropnet(&mut self) -> crate::core::graph::ExistingNodeRef {
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
    pub base: crate::core::types::NodeBase,
}

impl ObjectHairgen {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Parent"
    pub fn set_input_parent(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Parent" and specifies the output index of the target node.
    pub fn set_input_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Groom Data"
    pub fn set_input_groom_data(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Groom Data" and specifies the output index of the target node.
    pub fn set_input_groom_data_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_saveresthairfile(mut self) -> Self {
        self.base.params.insert(
            "saveresthairfile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.base.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influenceradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "influenceradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_influenceradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influenceradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influencedecay(mut self, val: f32) -> Self {
        self.base.params.insert(
            "influencedecay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_influencedecay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influencedecay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguideangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxguideangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxguideangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguideangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpcrossover(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clumpcrossover".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clumpcrossover_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clumpcrossover".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unguidedlength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "unguidedlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unguidedlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unguidedlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unguidedminlength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "unguidedminlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unguidedminlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unguidedminlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expansionfactor(mut self, val: f32) -> Self {
        self.base.params.insert(
            "expansionfactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expansionfactor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expansionfactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pruneratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pruneratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pruneratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pruneratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunecamfocal(mut self, val: f32) -> Self {
        self.base.params.insert(
            "aprunecamfocal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunecamfocal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunecamfocal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunecamaperture(mut self, val: f32) -> Self {
        self.base.params.insert(
            "aprunecamaperture".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunecamaperture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunecamaperture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunenormscreensize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "aprunenormscreensize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunenormscreensize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunenormscreensize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunepixelscreensize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "aprunepixelscreensize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunepixelscreensize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunepixelscreensize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunescreenagg(mut self, val: f32) -> Self {
        self.base.params.insert(
            "aprunescreenagg".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aprunescreenagg_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunescreenagg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaceoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "displaceoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displaceoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaceoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "displacescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacerefoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "displacerefoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacerefoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacerefoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacerefscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "displacerefscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacerefscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacerefscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_volumefilterwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_volumefilterwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_volumefilterwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_volumefilterwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_shadingquality(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_shadingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_shadingquality_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_shadingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_flatness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_flatness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_flatness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_flatness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_pointscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_pointscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_pointscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_pointscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sourceframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sourceframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourceframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_displaybboxcenter(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displaybboxcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaybboxcenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaybboxcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaybboxsize(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displaybboxsize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaybboxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaybboxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunecampos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "aprunecampos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_aprunecampos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunecampos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_aprunecamorient(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "aprunecamorient".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_aprunecamorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunecamorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_count(mut self, val: i32) -> Self {
        self.base.params.insert(
            "count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_count_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatterrelaxiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "scatterrelaxiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scatterrelaxiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scatterrelaxiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguidecount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxguidecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxguidecount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguidecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unguidedsegments(mut self, val: i32) -> Self {
        self.base.params.insert(
            "unguidedsegments".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_unguidedsegments_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unguidedsegments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsplitpasses(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxsplitpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsplitpasses_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsplitpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgeflippasses(mut self, val: i32) -> Self {
        self.base.params.insert(
            "edgeflippasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_edgeflippasses_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "edgeflippasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecoverage(mut self, val: i32) -> Self {
        self.base.params.insert(
            "guidecoverage".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guidecoverage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecoverage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguidesegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxguidesegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxguidesegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguidesegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expansioniters(mut self, val: i32) -> Self {
        self.base.params.insert(
            "expansioniters".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_expansioniters_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expansioniters".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunecamxres(mut self, val: i32) -> Self {
        self.base.params.insert(
            "aprunecamxres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_aprunecamxres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunecamxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunepruneseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "aprunepruneseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_aprunepruneseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunepruneseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_raypredice(mut self, val: ObjectHairgenVmRaypredice) -> Self {
        self.base.params.insert(
            "vm_raypredice".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_raypredice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_raypredice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_renderpoints(mut self, val: ObjectHairgenVmRenderpoints) -> Self {
        self.base.params.insert(
            "vm_renderpoints".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_renderpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_renderpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_renderpointsas(mut self, val: ObjectHairgenVmRenderpointsas) -> Self {
        self.base.params.insert(
            "vm_renderpointsas".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_renderpointsas_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_renderpointsas".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_coving(mut self, val: ObjectHairgenVmCoving) -> Self {
        self.base.params.insert(
            "vm_coving".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vm_coving_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_coving".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cooking_in_engine(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cooking_in_engine".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cooking_in_engine_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cooking_in_engine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sourcemode(mut self, val: ObjectHairgenSourcemode) -> Self {
        self.base.params.insert(
            "sourcemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbsource(mut self, val: ObjectHairgenVdbsource) -> Self {
        self.base.params.insert(
            "vdbsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vdbsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdbsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityoverride(mut self, val: ObjectHairgenDensityoverride) -> Self {
        self.base.params.insert(
            "densityoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_densityoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "densityoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideblendmethod(mut self, val: ObjectHairgenGuideblendmethod) -> Self {
        self.base.params.insert(
            "guideblendmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guideblendmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideblendmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influenceradiusoverride(
        mut self,
        val: ObjectHairgenInfluenceradiusoverride,
    ) -> Self {
        self.base.params.insert(
            "influenceradiusoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_influenceradiusoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influenceradiusoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influencedecayoverride(mut self, val: ObjectHairgenInfluencedecayoverride) -> Self {
        self.base.params.insert(
            "influencedecayoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_influencedecayoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influencedecayoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguidecountoverride(mut self, val: ObjectHairgenMaxguidecountoverride) -> Self {
        self.base.params.insert(
            "maxguidecountoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxguidecountoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguidecountoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguideangleoverride(mut self, val: ObjectHairgenMaxguideangleoverride) -> Self {
        self.base.params.insert(
            "maxguideangleoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxguideangleoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguideangleoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpcrossoveroverride(mut self, val: ObjectHairgenClumpcrossoveroverride) -> Self {
        self.base.params.insert(
            "clumpcrossoveroverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clumpcrossoveroverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clumpcrossoveroverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unguidedlengthoverride(mut self, val: ObjectHairgenUnguidedlengthoverride) -> Self {
        self.base.params.insert(
            "unguidedlengthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_unguidedlengthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unguidedlengthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformmethod(mut self, val: ObjectHairgenDeformmethod) -> Self {
        self.base.params.insert(
            "deformmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deformmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunescreensizeunit(mut self, val: ObjectHairgenAprunescreensizeunit) -> Self {
        self.base.params.insert(
            "aprunescreensizeunit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_aprunescreensizeunit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunescreensizeunit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessoverride(mut self, val: ObjectHairgenThicknessoverride) -> Self {
        self.base.params.insert(
            "thicknessoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_thicknessoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinsubdmode(mut self, val: ObjectHairgenSkinsubdmode) -> Self {
        self.base.params.insert(
            "skinsubdmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_skinsubdmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinsubdmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacemode(mut self, val: ObjectHairgenDisplacemode) -> Self {
        self.base.params.insert(
            "displacemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_displacemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectHairgenPreXform) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectHairgenXord) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectHairgenRord) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHairgenUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_onionskin(mut self, val: ObjectHairgenVportOnionskin) -> Self {
        self.base.params.insert(
            "vport_onionskin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vport_onionskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_onionskin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: ObjectHairgenViewportlod) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_thicknessramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "thicknessramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_thicknessramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_description(mut self, val: &str) -> Self {
        self.base.params.insert(
            "description".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_description_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "description".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcegroomobject(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegroomobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroomobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegroomobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcegroomfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegroomfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroomfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegroomfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vdbsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vdbsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdbsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbsopgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vdbsopgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vdbsopgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdbsopgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vdbfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vdbfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdbfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbfilegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vdbfilegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vdbfilegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdbfilegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "densityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_densityattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "densityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densitytexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "densitytexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_densitytexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "densitytexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influenceradiusattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "influenceradiusattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_influenceradiusattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influenceradiusattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influenceradiustexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "influenceradiustexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_influenceradiustexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influenceradiustexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influencedecayattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "influencedecayattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_influencedecayattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influencedecayattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_influencedecaytexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "influencedecaytexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_influencedecaytexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "influencedecaytexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguidecountattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maxguidecountattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxguidecountattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguidecountattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguidecounttexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maxguidecounttexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxguidecounttexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguidecounttexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguideangleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maxguideangleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxguideangleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguideangleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxguideangletexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maxguideangletexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxguideangletexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxguideangletexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpcrossoverattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "clumpcrossoverattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clumpcrossoverattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clumpcrossoverattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpcrossovertexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "clumpcrossovertexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clumpcrossovertexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clumpcrossovertexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initdirattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "initdirattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initdirattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initdirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unguidedlengthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "unguidedlengthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_unguidedlengthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unguidedlengthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unguidedlengthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "unguidedlengthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_unguidedlengthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unguidedlengthtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resthairfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "resthairfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resthairfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resthairfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "xformattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xformattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprunecam(mut self, val: &str) -> Self {
        self.base.params.insert(
            "aprunecam".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aprunecam_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprunecam".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknesstexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknesstexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknesstexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknesstexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessramp_basis(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknessramp_basis".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessramp_basis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessramp_basis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessramp_keys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknessramp_keys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessramp_keys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessramp_keys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessramp_values(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknessramp_values".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessramp_values_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessramp_values".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vertattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vertattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vertattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detailattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "detailattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_detailattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "detailattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidepointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidepointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidepointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidepointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideprimattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guideprimattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideprimattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideprimattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "keeppointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keeppointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepvertattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "keepvertattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepvertattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepvertattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepprimattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "keepprimattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepprimattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepprimattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepdetailattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "keepdetailattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keepdetailattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepdetailattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacerefshader(mut self, val: &str) -> Self {
        self.base.params.insert(
            "displacerefshader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displacerefshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacerefshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacetex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "displacetex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displacetex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacetex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaceskinshader(mut self, val: &str) -> Self {
        self.base.params.insert(
            "displaceskinshader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displaceskinshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaceskinshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displacereftex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "displacereftex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displacereftex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displacereftex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_auto_engine_procedural(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_auto_engine_procedural".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_auto_engine_procedural_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_auto_engine_procedural".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_auto_engine_boundsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_auto_engine_boundsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_auto_engine_boundsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_auto_engine_boundsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rendervisibility(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_rendervisibility".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_rendervisibility_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_rendervisibility".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "reflectmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflectmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reflectmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "refractmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refractmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_categories(mut self, val: &str) -> Self {
        self.base.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_categories_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lightmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lightmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lightmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_volumefilter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_volumefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_volumefilter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_volumefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_materialoverride(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_materialoverride".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_materialoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_materialoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.base.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hasinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hasinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hasinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderguides(mut self, val: bool) -> Self {
        self.base.params.insert(
            "renderguides".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderguides_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "renderguides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useanim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useanim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useanim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcecount(mut self, val: bool) -> Self {
        self.base.params.insert(
            "forcecount".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcecount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "forcecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useguides(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useguides".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useguides_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useguides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformguidesegments(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniformguidesegments".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniformguidesegments_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformguidesegments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skininterp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "skininterp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skininterp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skininterp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_growunguided(mut self, val: bool) -> Self {
        self.base.params.insert(
            "growunguided".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_growunguided_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "growunguided".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useinitdirattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useinitdirattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useinitdirattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useinitdirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayassubd(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displayassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayassubd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticgen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "staticgen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_staticgen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "staticgen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitclumps(mut self, val: bool) -> Self {
        self.base.params.insert(
            "splitclumps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splitclumps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitclumps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singleguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "singleguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singleguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computeradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitguidesegs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limitguidesegs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitguidesegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitguidesegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expandradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "expandradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expandradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadresthairfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loadresthairfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadresthairfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadresthairfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bypasssopnetwork(mut self, val: bool) -> Self {
        self.base.params.insert(
            "bypasssopnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bypasssopnetwork_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bypasssopnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limittobbox(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limittobbox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limittobbox_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limittobbox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepruning(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablepruning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepruning_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablepruning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prunethicken(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prunethicken".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prunethicken_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prunethicken".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prunestable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prunestable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prunestable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prunestable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aprune(mut self, val: bool) -> Self {
        self.base.params.insert(
            "aprune".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aprune_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aprune".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinsubd(mut self, val: bool) -> Self {
        self.base.params.insert(
            "skinsubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skinsubd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinsubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinsubdenabled(mut self, val: bool) -> Self {
        self.base.params.insert(
            "skinsubdenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skinsubdenabled_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinsubdenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displace(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rendersubd(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_rendersubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rendersubd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_rendersubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_matte(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_matte".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_matte_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_matte".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rayshade(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_rayshade".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rayshade_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_rayshade".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_velocityblur(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geo_velocityblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_velocityblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geo_velocityblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_curvesurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_curvesurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_curvesurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_curvesurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rmbackface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_rmbackface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rmbackface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_rmbackface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_forcegeometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_forcegeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_forcegeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_forcegeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_rendersubdcurves(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_rendersubdcurves".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_rendersubdcurves_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_rendersubdcurves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_usenforpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_usenforpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_usenforpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_usenforpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_pscalediameter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_pscalediameter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_pscalediameter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_pscalediameter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_metavolume(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_metavolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_metavolume_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_metavolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_overridedetail(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_overridedetail".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_overridedetail_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_overridedetail".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_procuseroottransform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_procuseroottransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_procuseroottransform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_procuseroottransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "renderable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayhairshaded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displayhairshaded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayhairshaded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayhairshaded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectHairgen {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairgen"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHairgenInnerExt {
    fn hairs(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHairgenInnerExt for crate::core::graph::InnerGraph<'a, ObjectHairgen> {
    fn hairs(&mut self) -> crate::core::graph::ExistingNodeRef {
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
    pub base: crate::core::types::NodeBase,
}

impl ObjectHandle {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "parent"
    pub fn set_input_parent(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "parent" and specifies the output index of the target node.
    pub fn set_input_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_weight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "weight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_lrx(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "lrx".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lrx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lrx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lry(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "lry".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lrz(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "lrz".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lrz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lrz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectHandleXord) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectHandleRord) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectHandlePreXform) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHandleUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_targetpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_targetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_targetpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_targetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_twistonly(mut self, val: bool) -> Self {
        self.base.params.insert(
            "twistonly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twistonly_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twistonly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorxlimit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dorxlimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorxlimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dorxlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorylimit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dorylimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorylimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dorylimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorzlimit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dorzlimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorzlimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dorzlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectHandle {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "handle"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHandleInnerExt {
    fn add1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn add2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn box1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn x(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn y(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn z(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHandleInnerExt for crate::core::graph::InnerGraph<'a, ObjectHandle> {
    fn add1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("add1")
    }
    fn add2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("add2")
    }
    fn box1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("box1")
    }
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge2")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch2")
    }
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn x(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("x")
    }
    fn xform1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform1")
    }
    fn xform2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform2")
    }
    fn xform6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform6")
    }
    fn y(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("y")
    }
    fn z(&mut self) -> crate::core::graph::ExistingNodeRef {
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
    pub base: crate::core::types::NodeBase,
}

impl ObjectHlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "light_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_exposure(mut self, val: f32) -> Self {
        self.base.params.insert(
            "light_exposure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_exposure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_exposure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_samplingquality(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_samplingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_samplingquality_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_samplingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_fov(mut self, val: f32) -> Self {
        self.base.params.insert(
            "light_fov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_fov_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_fov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conedelta(mut self, val: f32) -> Self {
        self.base.params.insert(
            "conedelta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conedelta_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "conedelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coneroll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coneroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneroll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coneroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_near(mut self, val: f32) -> Self {
        self.base.params.insert(
            "near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_near_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_far(mut self, val: f32) -> Self {
        self.base.params.insert(
            "far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_far_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "far".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areamapblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "areamapblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_areamapblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areamapblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areamapscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "areamapscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_areamapscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areamapscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgewidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "edgewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgewidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "edgewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgerolloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "edgerolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgerolloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "edgerolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_shadingfactor(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_shadingfactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_shadingfactor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_shadingfactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orthowidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "orthowidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_orthowidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orthowidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_envangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vm_envangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_envangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_envangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_dist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "atten_dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_dist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten_dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "atten_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_rampstart(mut self, val: f32) -> Self {
        self.base.params.insert(
            "atten_rampstart".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_rampstart_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten_rampstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_rampend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "atten_rampend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_rampend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten_rampend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shadow_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shadow_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_quality(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shadow_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_quality_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_softness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shadow_softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_softness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_blur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shadow_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_blur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iconscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "iconscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iconscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iconscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_dimmer(mut self, val: f32) -> Self {
        self.base.params.insert(
            "l_dimmer".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_dimmer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "l_dimmer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dimmer(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dimmer".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dimmer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dimmer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focal(mut self, val: f32) -> Self {
        self.base.params.insert(
            "focal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "focal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperture(mut self, val: f32) -> Self {
        self.base.params.insert(
            "aperture".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aperture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focus(mut self, val: f32) -> Self {
        self.base.params.insert(
            "focus".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focus_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "focus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_areasize(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "areasize".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_areasize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areasize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_light_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "shadow_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadow_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pc_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pc_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pc_samples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pc_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.base
            .params
            .insert("res".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmap_samples(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_shadowmap_samples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectHlightXord) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: ObjectHlightRord) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectHlightPreXform) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ObjectHlightUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_type(mut self, val: ObjectHlightLightType) -> Self {
        self.base.params.insert(
            "light_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_light_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_type(mut self, val: ObjectHlightAttenType) -> Self {
        self.base.params.insert(
            "atten_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_atten_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_type(mut self, val: ObjectHlightShadowType) -> Self {
        self.base.params.insert(
            "shadow_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadow_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmap_resmenu(mut self, val: ObjectHlightShadowmapResmenu) -> Self {
        self.base.params.insert(
            "shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shadowmap_resmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projection2(mut self, val: ObjectHlightProjection2) -> Self {
        self.base.params.insert(
            "projection2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_projection2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projection2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_attenramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "attenramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_attenramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attenramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_contribname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("light_contribname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_light_contribname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("light_contribname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_categories(mut self, val: &str) -> Self {
        self.base.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_categories_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "categories".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_lpetag(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_lpetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_lpetag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_lpetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projmap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "projmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_projmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areamap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "areamap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_areamap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areamap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areamapspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "areamapspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_areamapspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areamapspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areamapnull(mut self, val: &str) -> Self {
        self.base.params.insert(
            "areamapnull".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_areamapnull_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areamapnull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_texture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "light_texture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_light_texture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_texture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areageometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "areageometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_areageometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areageometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pc_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pc_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pc_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pc_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pc_camera(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pc_camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pc_camera_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pc_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shadowmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowcategories(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shadowcategories".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowcategories_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowcategories".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmap_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shadowmap_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowmap_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowmap_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_winmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "winmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_winmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "winmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_wrangler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "light_wrangler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_light_wrangler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_wrangler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_enable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "light_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_enable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_enablelight(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ogl_enablelight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_enablelight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_enablelight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_contribenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("light_contribenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_contribenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("light_contribenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_contribprimary(mut self, val: bool) -> Self {
        self.base.params.insert(
            "light_contribprimary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_contribprimary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_contribprimary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coneenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "coneenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coneenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coneenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_conefov(mut self, val: bool) -> Self {
        self.base.params.insert(
            "light_conefov".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_conefov_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light_conefov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalizearea(mut self, val: bool) -> Self {
        self.base.params.insert(
            "normalizearea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalizearea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normalizearea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singlesided(mut self, val: bool) -> Self {
        self.base.params.insert(
            "singlesided".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singlesided_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singlesided".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reverse(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharpspot(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sharpspot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sharpspot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharpspot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_selfshadow(mut self, val: bool) -> Self {
        self.base.params.insert(
            "selfshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_selfshadow_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "selfshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgeenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "edgeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_edgeenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "edgeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intothisobject(mut self, val: bool) -> Self {
        self.base.params.insert(
            "intothisobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_intothisobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intothisobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pc_enable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pc_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pc_enable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pc_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_render_pointcloud(mut self, val: bool) -> Self {
        self.base.params.insert(
            "render_pointcloud".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_render_pointcloud_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "render_pointcloud".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pc_camera_override(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pc_camera_override".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pc_camera_override_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pc_camera_override".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activeradiusenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "activeradiusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_activeradiusenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activeradiusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenrampenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "attenrampenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenrampenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attenrampenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_transparent(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shadow_transparent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_transparent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_transparent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_render_shadowmap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "render_shadowmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_render_shadowmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "render_shadowmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmotionblur(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shadowmotionblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadowmotionblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowmotionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_autofit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shadow_autofit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_autofit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadow_autofit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clampprojmap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "clampprojmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clampprojmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clampprojmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areafullsphere(mut self, val: bool) -> Self {
        self.base.params.insert(
            "areafullsphere".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_areafullsphere_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areafullsphere".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectHlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "hlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectHlightInnerExt {
    fn origin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn areashape1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn carve1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn circle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn circle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cone_light1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn convert1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copy2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn facet1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn grid2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn grid4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn grid5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn line5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn line6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn line7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn point_light1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sphere1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sphere2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn tube1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn tube2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn xform7(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectHlightInnerExt for crate::core::graph::InnerGraph<'a, ObjectHlight> {
    fn origin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("Origin")
    }
    fn areashape1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("areashape1")
    }
    fn carve1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("carve1")
    }
    fn circle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("circle2")
    }
    fn circle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("circle4")
    }
    fn cone_light1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cone_light1")
    }
    fn constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("constraints")
    }
    fn convert1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("convert1")
    }
    fn copy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copy1")
    }
    fn copy2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copy2")
    }
    fn facet1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("facet1")
    }
    fn grid2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("grid2")
    }
    fn grid4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("grid4")
    }
    fn grid5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("grid5")
    }
    fn line5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("line5")
    }
    fn line6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("line6")
    }
    fn line7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("line7")
    }
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge3")
    }
    fn object_merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge1")
    }
    fn point_light1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("point_light1")
    }
    fn sphere1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sphere1")
    }
    fn sphere2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sphere2")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn tube1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("tube1")
    }
    fn tube2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("tube2")
    }
    fn xform1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform1")
    }
    fn xform3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform3")
    }
    fn xform4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform4")
    }
    fn xform5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform5")
    }
    fn xform6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform6")
    }
    fn xform7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("xform7")
    }
}
