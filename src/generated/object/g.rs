#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGeoXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGeoRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGeoPreXform {
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
pub enum ObjectGeoUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGeoVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectGeo {
    pub base: crate::core::types::NodeBase,
}

impl ObjectGeo {
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
    pub fn with_xord(mut self, val: ObjectGeoXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectGeoRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectGeoPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectGeoUparmtype) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectGeoVportOnionskin) -> Self {
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
}

impl crate::core::types::HoudiniNode for ObjectGeo {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "geo"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGltfHierarchyXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGltfHierarchyRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGltfHierarchyPreXform {
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
pub enum ObjectGltfHierarchyUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectGltfHierarchy {
    pub base: crate::core::types::NodeBase,
}

impl ObjectGltfHierarchy {
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

    // --- Button parameters ---
    pub fn trigger_buildscene(mut self) -> Self {
        self.base.params.insert(
            "buildscene".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_pointconsolidatedist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pointconsolidatedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointconsolidatedist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointconsolidatedist".to_string(),
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

    // --- Float3 parameters ---
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
    pub fn with_xord(mut self, val: ObjectGltfHierarchyXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectGltfHierarchyRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectGltfHierarchyPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectGltfHierarchyUparmtype) -> Self {
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
    pub fn with_filename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assetfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "assetfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_assetfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "assetfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scene(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scene".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scene_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scene".to_string(),
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
    pub fn with_lockgeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "lockgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lockgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lockgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenhierarchy(mut self, val: bool) -> Self {
        self.base.params.insert(
            "flattenhierarchy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenhierarchy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "flattenhierarchy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_promotepointattrs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "promotepointattrs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_promotepointattrs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "promotepointattrs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importgeometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importcustomattributes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importcustomattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importcustomattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importcustomattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importmaterials(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importmaterials".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importmaterials_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importmaterials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importnongeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importnongeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importnongeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importnongeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importunusedmaterials(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importunusedmaterials".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importunusedmaterials_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importunusedmaterials".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectGltfHierarchy {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "gltf_hierarchy"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergePreXform {
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
pub enum ObjectGroommergeUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeVmRaypredice {
    DisablePredicing = 0,
    FullPredicing = 1,
    PrecomputeBounds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeVmRenderpoints {
    NoPointRendering = 0,
    RenderOnlyPoints = 1,
    RenderUnconnectedPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeVmRenderpointsas {
    Spheres = 0,
    Circles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeVmCoving {
    DisableCoving = 0,
    /// Coving for displacement/sub-d
    CovingForDisplacementSubMinusD = 1,
    CovingForAllPrimitives = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGroommergeVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectGroommerge {
    pub base: crate::core::types::NodeBase,
}

impl ObjectGroommerge {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), 0));
        self.base.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), output_index));
        self.base.next_input_index += 1;
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
    pub fn with_vm_raypredice(mut self, val: ObjectGroommergeVmRaypredice) -> Self {
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
    pub fn with_vm_renderpoints(mut self, val: ObjectGroommergeVmRenderpoints) -> Self {
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
    pub fn with_vm_renderpointsas(mut self, val: ObjectGroommergeVmRenderpointsas) -> Self {
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
    pub fn with_vm_coving(mut self, val: ObjectGroommergeVmCoving) -> Self {
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

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectGroommergeXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectGroommergeRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectGroommergePreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectGroommergeUparmtype) -> Self {
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
    pub fn with_viewportlod(mut self, val: ObjectGroommergeViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectGroommergeVportOnionskin) -> Self {
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

    // --- String parameters ---
    pub fn with_sourcegroomobjects(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegroomobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroomobjects_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegroomobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isolategroomobjects(mut self, val: &str) -> Self {
        self.base.params.insert(
            "isolategroomobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_isolategroomobjects_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isolategroomobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinsops(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinsops".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinsops_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinsops".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidenameattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidenameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidenameattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidenameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidepathattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidepathattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidepathattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidepathattrib".to_string(),
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
    pub fn with_guidevertexattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidevertexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidevertexattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidevertexattribs".to_string(),
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
    pub fn with_guidedetailattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidedetailattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidedetailattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidedetailattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinpointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinpointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinpointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinpointattribs".to_string(),
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
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_subdgroup".to_string(),
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
    pub fn with_lightcategories(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lightcategories".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lightcategories_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcategories".to_string(),
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
    pub fn with_shop_geometrypath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_geometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_geometrypath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_geometrypath".to_string(),
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
    pub fn with_creategroups(mut self, val: bool) -> Self {
        self.base.params.insert(
            "creategroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_creategroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "creategroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createguidenameattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createguidenameattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createguidenameattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createguidenameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createguidepathattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createguidepathattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createguidepathattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createguidepathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reseteids(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reseteids".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reseteids_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reseteids".to_string(),
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
    pub fn with_vm_renderable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_renderable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
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
}

impl crate::core::types::HoudiniNode for ObjectGroommerge {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "groommerge"
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
pub trait ObjectGroommergeInnerExt {
    fn display(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn menu_attribs(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_groom(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_restguides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_restskin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn points(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcomposite1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcopy2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribute1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribute2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn enumerate1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn feedback(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn file1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn file2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn file3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn file4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn guide_merge_begin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn guide_merge_end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn metadata(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name_and_path(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn packgroom1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn point(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointwrangle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_on_first_run(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn visualize1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn visualize4(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectGroommergeInnerExt for crate::core::graph::InnerGraph<'a, ObjectGroommerge> {
    fn display(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("DISPLAY")
    }
    fn menu_attribs(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("MENU_ATTRIBS")
    }
    fn out_groom(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_GROOM")
    }
    fn out_restguides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_RESTGUIDES")
    }
    fn out_restskin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_RESTSKIN")
    }
    fn points(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("POINTS")
    }
    fn attribcomposite1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcomposite1")
    }
    fn attribcopy2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcopy2")
    }
    fn attribute1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribute1")
    }
    fn attribute2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribute2")
    }
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete1")
    }
    fn end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("end")
    }
    fn enumerate1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("enumerate1")
    }
    fn feedback(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("feedback")
    }
    fn file1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("file1")
    }
    fn file2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("file2")
    }
    fn file3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("file3")
    }
    fn file4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("file4")
    }
    fn foreach_begin3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin3")
    }
    fn foreach_end3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_end3")
    }
    fn group1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("group1")
    }
    fn guide_merge_begin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("guide_merge_begin")
    }
    fn guide_merge_end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("guide_merge_end")
    }
    fn metadata(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("metadata")
    }
    fn name_and_path(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("name_and_path")
    }
    fn object_merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge1")
    }
    fn object_merge2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge2")
    }
    fn object_merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge3")
    }
    fn packgroom1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("packgroom1")
    }
    fn point(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("point")
    }
    fn pointwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pointwrangle1")
    }
    fn pointwrangle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pointwrangle4")
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
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch4")
    }
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch5")
    }
    fn switch_on_first_run(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_on_first_run")
    }
    fn unpackgroom2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom2")
    }
    fn unpackgroom3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom3")
    }
    fn unpackgroom4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom4")
    }
    fn unpackgroom6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom6")
    }
    fn visualize1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("visualize1")
    }
    fn visualize4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("visualize4")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformPreXform {
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
pub enum ObjectGuidedeformUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformVmRaypredice {
    DisablePredicing = 0,
    FullPredicing = 1,
    PrecomputeBounds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformVmRenderpoints {
    NoPointRendering = 0,
    RenderOnlyPoints = 1,
    RenderUnconnectedPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformVmRenderpointsas {
    Spheres = 0,
    Circles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformVmCoving {
    DisableCoving = 0,
    /// Coving for displacement/sub-d
    CovingForDisplacementSubMinusD = 1,
    CovingForAllPrimitives = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformSourcemode {
    GroomObject = 0,
    GroomFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformFilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformTrange {
    SaveCurrentFrame = 0,
    SaveFrameRange = 1,
    /// Save Frame Range Only (Strict)
    SaveFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformXformtype {
    None = 0,
    IntoWorldSpace = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformLoadtype {
    AllGeometry = 0,
    InfoBoundingBox = 1,
    Info = 2,
    PointCloud = 3,
    PackedDiskPrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformPackedviewedit {
    UseFileSetting = 0,
    FullGeometry = 1,
    PointCloud = 2,
    BoundingBox = 3,
    Centroid = 4,
    Hidden = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidedeformViewportlod2 {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone)]
pub struct ObjectGuidedeform {
    pub base: crate::core::types::NodeBase,
}

impl ObjectGuidedeform {
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

    /// Connects to input 2: "Animated Skin Geometry"
    pub fn set_input_animated_skin_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Animated Skin Geometry" and specifies the output index of the target node.
    pub fn set_input_animated_skin_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.base
            .params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_execute(mut self) -> Self {
        self.base.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.base.params.insert(
            "executebackground".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.base.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
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
    pub fn with_displaycolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displaycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaycolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaycolor".to_string(),
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
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "f".to_string(),
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
    pub fn with_vm_raypredice(mut self, val: ObjectGuidedeformVmRaypredice) -> Self {
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
    pub fn with_vm_renderpoints(mut self, val: ObjectGuidedeformVmRenderpoints) -> Self {
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
    pub fn with_vm_renderpointsas(mut self, val: ObjectGuidedeformVmRenderpointsas) -> Self {
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
    pub fn with_vm_coving(mut self, val: ObjectGuidedeformVmCoving) -> Self {
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
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_saveretry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "saveretry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_saveretry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "saveretry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectGuidedeformXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectGuidedeformRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectGuidedeformPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectGuidedeformUparmtype) -> Self {
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
    pub fn with_viewportlod(mut self, val: ObjectGuidedeformViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectGuidedeformVportOnionskin) -> Self {
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
    pub fn with_sourcemode(mut self, val: ObjectGuidedeformSourcemode) -> Self {
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
    pub fn with_filemode(mut self, val: ObjectGuidedeformFilemode) -> Self {
        self.base.params.insert(
            "filemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trange(mut self, val: ObjectGuidedeformTrange) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformtype(mut self, val: ObjectGuidedeformXformtype) -> Self {
        self.base.params.insert(
            "xformtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xformtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingframe(mut self, val: ObjectGuidedeformMissingframe) -> Self {
        self.base.params.insert(
            "missingframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "missingframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadtype(mut self, val: ObjectGuidedeformLoadtype) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_loadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedviewedit(mut self, val: ObjectGuidedeformPackedviewedit) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_packedviewedit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod2(mut self, val: ObjectGuidedeformViewportlod2) -> Self {
        self.base.params.insert(
            "viewportlod2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewportlod2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_subdgroup".to_string(),
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
    pub fn with_shop_geometrypath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_geometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_geometrypath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_geometrypath".to_string(),
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
    pub fn with_animskin(mut self, val: &str) -> Self {
        self.base.params.insert(
            "animskin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animskin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animskingroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "animskingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animskingroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animskingroup".to_string(),
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
    pub fn with_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.base.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deleteattributes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deleteattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deleteattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deleteattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletegroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deletegroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deletegroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deletegroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_class_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("class{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_class_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("class{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("attribs{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hasinputgroom(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hasinputgroom".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasinputgroom_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hasinputgroom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hasinputskin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hasinputskin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasinputskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hasinputskin".to_string(),
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
    pub fn with_vm_renderable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_renderable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
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
    pub fn with_enabledeformation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledeformation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledeformation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledeformation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computev(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computev".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computev_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computev".to_string(),
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
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.base.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savebackground(mut self, val: bool) -> Self {
        self.base.params.insert(
            "savebackground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebackground_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "savebackground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packexpanded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packexpanded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delayload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delayload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefetch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefetch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectGuidedeform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "guidedeform"
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
pub trait ObjectGuidedeformInnerExt {
    fn anim_skin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn display(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn enable_compute_point_v(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group_polys(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_animguides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_animskin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_groom(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_restguides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_restskin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_enable_deformation(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_enable_deformation1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn convert1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn filecache1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn first_frame_rest_guides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn first_frame_rest_skin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn from_disk(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn from_disk1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn from_disk2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn from_disk3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn groomfetch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn guidedeform1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn objnet1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn packgroom1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn packgroom2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn shaders(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeblend1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeblend2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeshift1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeshift2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn trail1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpack1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectGuidedeformInnerExt for crate::core::graph::InnerGraph<'a, ObjectGuidedeform> {
    fn anim_skin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ANIM_SKIN")
    }
    fn display(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("DISPLAY")
    }
    fn enable_compute_point_v(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ENABLE_COMPUTE_POINT_V")
    }
    fn group_polys(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("GROUP_POLYS")
    }
    fn out_animguides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_ANIMGUIDES")
    }
    fn out_animskin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_ANIMSKIN")
    }
    fn out_groom(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_GROOM")
    }
    fn out_restguides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_RESTGUIDES")
    }
    fn out_restskin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_RESTSKIN")
    }
    fn switch_enable_deformation(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SWITCH_ENABLE_DEFORMATION")
    }
    fn switch_enable_deformation1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SWITCH_ENABLE_DEFORMATION1")
    }
    fn convert1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("convert1")
    }
    fn filecache1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("filecache1")
    }
    fn first_frame_rest_guides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("first_frame_rest_guides")
    }
    fn first_frame_rest_skin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("first_frame_rest_skin")
    }
    fn from_disk(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("from_disk")
    }
    fn from_disk1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("from_disk1")
    }
    fn from_disk2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("from_disk2")
    }
    fn from_disk3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("from_disk3")
    }
    fn groomfetch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groomfetch1")
    }
    fn guidedeform1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("guidedeform1")
    }
    fn object_merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge1")
    }
    fn object_merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge3")
    }
    fn objnet1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("objnet1")
    }
    fn packgroom1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("packgroom1")
    }
    fn packgroom2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("packgroom2")
    }
    fn shaders(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("shaders")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch4")
    }
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch5")
    }
    fn switch6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch6")
    }
    fn timeblend1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeblend1")
    }
    fn timeblend2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeblend2")
    }
    fn timeshift1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeshift1")
    }
    fn timeshift2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeshift2")
    }
    fn trail1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("trail1")
    }
    fn unpack1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpack1")
    }
    fn unpackgroom1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomSourcemode {
    NoInputGroom = 0,
    GroomObject = 1,
    GroomFile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomSkinsource {
    SopGeometry = 0,
    File = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomVdbsource {
    FromSkinGeometry = 0,
    SopGeometry = 1,
    File = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomGuidetype {
    ScatterOnSurface = 0,
    GuidePerPoint = 1,
    UseExternalGeometry = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomDensityoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomLengthoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomFilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomTrange {
    SaveCurrentFrame = 0,
    SaveFrameRange = 1,
    /// Save Frame Range Only (Strict)
    SaveFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomLoadtype {
    AllGeometry = 0,
    InfoBoundingBox = 1,
    Info = 2,
    PointCloud = 3,
    PackedDiskPrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomPackedviewedit {
    UseFileSetting = 0,
    FullGeometry = 1,
    PointCloud = 2,
    BoundingBox = 3,
    Centroid = 4,
    Hidden = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomViewportlod2 {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomPreXform {
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
pub enum ObjectGuidegroomXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidegroomViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone)]
pub struct ObjectGuidegroom {
    pub base: crate::core::types::NodeBase,
}

impl ObjectGuidegroom {
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

    /// Connects to input 1: "Skin Geometry or Groom Data"
    pub fn set_input_skin_geometry_or_groom_data(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin Geometry or Groom Data" and specifies the output index of the target node.
    pub fn set_input_skin_geometry_or_groom_data_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.base
            .params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_execute(mut self) -> Self {
        self.base.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.base.params.insert(
            "executebackground".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.base.params.insert(
            "renderdialog".to_string(),
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
    pub fn with_scatterseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scatterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scatterseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scatterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_length(mut self, val: f32) -> Self {
        self.base.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_drawwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "drawwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_drawwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "drawwidth".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_displaycolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displaycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaycolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaycolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "f".to_string(),
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

    // --- Int parameters ---
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
    pub fn with_segments(mut self, val: i32) -> Self {
        self.base.params.insert(
            "segments".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_segments_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "segments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_saveretry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "saveretry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_saveretry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "saveretry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
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

    // --- Menu parameters ---
    pub fn with_sourcemode(mut self, val: ObjectGuidegroomSourcemode) -> Self {
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
    pub fn with_skinsource(mut self, val: ObjectGuidegroomSkinsource) -> Self {
        self.base.params.insert(
            "skinsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_skinsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbsource(mut self, val: ObjectGuidegroomVdbsource) -> Self {
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
    pub fn with_guidetype(mut self, val: ObjectGuidegroomGuidetype) -> Self {
        self.base.params.insert(
            "guidetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidetype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityoverride(mut self, val: ObjectGuidegroomDensityoverride) -> Self {
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
    pub fn with_lengthoverride(mut self, val: ObjectGuidegroomLengthoverride) -> Self {
        self.base.params.insert(
            "lengthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lengthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filemode(mut self, val: ObjectGuidegroomFilemode) -> Self {
        self.base.params.insert(
            "filemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trange(mut self, val: ObjectGuidegroomTrange) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingframe(mut self, val: ObjectGuidegroomMissingframe) -> Self {
        self.base.params.insert(
            "missingframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "missingframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadtype(mut self, val: ObjectGuidegroomLoadtype) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_loadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedviewedit(mut self, val: ObjectGuidegroomPackedviewedit) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_packedviewedit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod2(mut self, val: ObjectGuidegroomViewportlod2) -> Self {
        self.base.params.insert(
            "viewportlod2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewportlod2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ObjectGuidegroomPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectGuidegroomXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectGuidegroomRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectGuidegroomUparmtype) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectGuidegroomVportOnionskin) -> Self {
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
    pub fn with_viewportlod(mut self, val: ObjectGuidegroomViewportlod) -> Self {
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
    pub fn with_inputskinsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputskinsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputskinsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputskinsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinfile".to_string(),
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
    pub fn with_guidegeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidegeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidegeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidegeo".to_string(),
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
    pub fn with_lengthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lengthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lengthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lengthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lengthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthtexture".to_string(),
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
    pub fn with_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.base.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deleteattributes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deleteattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deleteattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deleteattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletegroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deletegroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deletegroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deletegroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_class_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("class{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_class_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("class{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("attribs{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("precision{}", index1),
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
    pub fn with_isinputgroomnode(mut self, val: bool) -> Self {
        self.base.params.insert(
            "isinputgroomnode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_isinputgroomnode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isinputgroomnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createguides(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createguides".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createguides_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createguides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideresample(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guideresample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideresample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideresample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidereverse(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidereverse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidereverse_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidereverse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidemovetoskin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidemovetoskin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidemovetoskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidemovetoskin".to_string(),
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
    pub fn with_drawwidecurves(mut self, val: bool) -> Self {
        self.base.params.insert(
            "drawwidecurves".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_drawwidecurves_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "drawwidecurves".to_string(),
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
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.base.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savebackground(mut self, val: bool) -> Self {
        self.base.params.insert(
            "savebackground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebackground_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "savebackground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packexpanded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packexpanded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delayload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delayload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefetch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefetch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
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
    pub fn with_vm_renderable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_renderable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectGuidegroom {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "guidegroom"
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
        Some("groom")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectGuidegroomInnerExt {
    fn guides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn skin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn skinvdb(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectGuidegroomInnerExt for crate::core::graph::InnerGraph<'a, ObjectGuidegroom> {
    fn guides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groom/GUIDES")
    }
    fn skin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groom/SKIN")
    }
    fn skinvdb(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groom/SKINVDB")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimPreXform {
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
pub enum ObjectGuidesimUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVmRaypredice {
    DisablePredicing = 0,
    FullPredicing = 1,
    PrecomputeBounds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVmRenderpoints {
    NoPointRendering = 0,
    RenderOnlyPoints = 1,
    RenderUnconnectedPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVmRenderpointsas {
    Spheres = 0,
    Circles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVmCoving {
    DisableCoving = 0,
    /// Coving for displacement/sub-d
    CovingForDisplacementSubMinusD = 1,
    CovingForAllPrimitives = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimSourcemode {
    GroomObject = 0,
    GroomFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimSolver {
    WireSolver = 0,
    Vellum = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimExtsimsource {
    DopNode = 0,
    SopNode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimRootconstraint {
    /// Position & Direction (gluetoanimation)
    PositionDirectionGluetoanimation = 0,
    /// Position (pintoanimation)
    PositionPintoanimation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimCollisionhandling {
    Sdf = 0,
    LocalGeometric = 1,
    GlobalGeometric = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimSkinvdbsource {
    ComputeFromGeometry = 0,
    SopVolume = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimExtvdbsource {
    ComputeFromGeometry = 0,
    SopVolume = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsRootrotation {
    None = 0,
    Hard = 1,
    Soft = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsStretchstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsStretchdampingscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsStretchplasticthresholdscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsStretchplasticratescalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsStretchplastichardeningscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsBendstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsBenddampingscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsBendplasticthresholdscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsBendplasticratescalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimVellumconstraintsBendplastichardeningscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimDopioTrange {
    SaveCurrentFrame = 0,
    SaveFrameRange = 1,
    /// Save Frame Range Only (Strict)
    SaveFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimDopioXformtype {
    None = 0,
    IntoWorldSpace = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimDopioMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimDopioLoadtype {
    AllGeometry = 0,
    InfoBoundingBox = 1,
    Info = 2,
    PointCloud = 3,
    PackedDiskPrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectGuidesimDopioPackedviewedit {
    UseFileSetting = 0,
    FullGeometry = 1,
    PointCloud = 2,
    BoundingBox = 3,
    Centroid = 4,
    Hidden = 5,
}

#[derive(Debug, Clone)]
pub struct ObjectGuidesim {
    pub base: crate::core::types::NodeBase,
}

impl ObjectGuidesim {
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
    pub fn trigger_resimulate(mut self) -> Self {
        self.base.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_createexternaldopnet(mut self) -> Self {
        self.base.params.insert(
            "createexternaldopnet".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dopio_reload(mut self) -> Self {
        self.base.params.insert(
            "dopio_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dopio_execute(mut self) -> Self {
        self.base.params.insert(
            "dopio_execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dopio_executebackground(mut self) -> Self {
        self.base.params.insert(
            "dopio_executebackground".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dopio_renderdialog(mut self) -> Self {
        self.base.params.insert(
            "dopio_renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
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
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spatialscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "spatialscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spatialscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spatialscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tol".to_string(),
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
    pub fn with_lengthdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengthdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mass".to_string(),
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
    pub fn with_friction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamicfriction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_klinear(mut self, val: f32) -> Self {
        self.base.params.insert(
            "klinear".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_klinear_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "klinear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damplinear(mut self, val: f32) -> Self {
        self.base.params.insert(
            "damplinear".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damplinear_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "damplinear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kangular(mut self, val: f32) -> Self {
        self.base.params.insert(
            "kangular".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kangular_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kangular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampangular(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dampangular".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampangular_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampangular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthmatch(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengthmatch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthmatch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthmatchref(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengthmatchref".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthmatchref_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthmatchref".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticstretchthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "plasticstretchthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticstretchthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticstretchthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticstretchrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "plasticstretchrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticstretchrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticstretchrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticstretchhardening(mut self, val: f32) -> Self {
        self.base.params.insert(
            "plasticstretchhardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticstretchhardening_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticstretchhardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticbendthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "plasticbendthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticbendthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticbendthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticbendrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "plasticbendrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticbendrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticbendrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticbendhardening(mut self, val: f32) -> Self {
        self.base.params.insert(
            "plasticbendhardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticbendhardening_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticbendhardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "targetstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "windstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windnoiseroughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "windnoiseroughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windnoiseroughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windnoiseroughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windnoiseattenuation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "windnoiseattenuation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windnoiseattenuation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windnoiseattenuation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.base.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsion(mut self, val: f32) -> Self {
        self.base.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinvdbvoxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "skinvdbvoxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skinvdbvoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinvdbvoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinvdbinteriorband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "skinvdbinteriorband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skinvdbinteriorband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinvdbinteriorband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extvdbvoxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "extvdbvoxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extvdbvoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extvdbvoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extvdbinteriorband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "extvdbinteriorband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extvdbinteriorband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extvdbinteriorband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_timescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_timescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_thickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_thickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_mass(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_mass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_mass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_mass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_friction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_friction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_selffriction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_selffriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_selffriction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_selffriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_static_threshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_static_threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_static_threshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_static_threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_kinetic_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_kinetic_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_kinetic_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_kinetic_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_rootbendstiffnessratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_rootbendstiffnessratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_rootbendstiffnessratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_rootbendstiffnessratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffnessscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffnessscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchrestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchrestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_compressstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_compressstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_compressstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_compressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticratescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticratescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticratescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticratescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardening(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardening_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardeningscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardeningscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardeningscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardeningscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffnessscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffnessscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendrestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendrestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthresholdscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthresholdscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthresholdscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthresholdscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticratescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticratescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticratescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticratescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardening(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardening_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardeningscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardeningscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardeningscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardeningscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_clumpstretchstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpstretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_clumpstretchstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpstretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_clumpstretchdampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpstretchdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_clumpstretchdampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpstretchdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_clumpstretchrestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpstretchrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_clumpstretchrestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpstretchrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_clumpcompressstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpcompressstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumconstraints_clumpcompressstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpcompressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_winddrag(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_winddrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_winddrag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_winddrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_dragnormal(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_dragnormal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_dragnormal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_dragnormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_dragtangent(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_dragtangent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_dragtangent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_dragtangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcecomputedvoxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_hairforcecomputedvoxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_hairforcecomputedvoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcecomputedvoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcevoxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_hairforcevoxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_hairforcevoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcevoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcedensitythreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_hairforcedensitythreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_hairforcedensitythreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcedensitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcedensitybandwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_hairforcedensitybandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_hairforcedensitybandwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcedensitybandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_hairforcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_hairforcescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcedrag(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellum_hairforcedrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellum_hairforcedrag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcedrag".to_string(),
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
    pub fn with_displaycolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displaycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displaycolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaycolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gravity(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windvel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "windvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_windvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windnoisefrequency(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "windnoisefrequency".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_windnoisefrequency_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windnoisefrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitude(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amplitude_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_gravity(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vellum_gravity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vellum_gravity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_gravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_windv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vellum_windv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vellum_windv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_windv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_f(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dopio_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dopio_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_f".to_string(),
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
    pub fn with_vm_raypredice(mut self, val: ObjectGuidesimVmRaypredice) -> Self {
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
    pub fn with_vm_renderpoints(mut self, val: ObjectGuidesimVmRenderpoints) -> Self {
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
    pub fn with_vm_renderpointsas(mut self, val: ObjectGuidesimVmRenderpointsas) -> Self {
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
    pub fn with_vm_coving(mut self, val: ObjectGuidesimVmCoving) -> Self {
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
    pub fn with_freezeframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "freezeframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_freezeframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freezeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minsubsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "minsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minsubsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvemaxpasses(mut self, val: i32) -> Self {
        self.base.params.insert(
            "resolvemaxpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolvemaxpasses_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resolvemaxpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpguidelimit(mut self, val: i32) -> Self {
        self.base.params.insert(
            "clumpguidelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_clumpguidelimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clumpguidelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "windseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_windseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windnoisefractaldepth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "windnoisefractaldepth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_windnoisefractaldepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windnoisefractaldepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_substeps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellum_substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellum_substeps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_niter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellum_niter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellum_niter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_niter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_collisioniter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellum_collisioniter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellum_collisioniter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_collisioniter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_postcollisioniter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellum_postcollisioniter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellum_postcollisioniter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_postcollisioniter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_saveretry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "dopio_saveretry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dopio_saveretry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_saveretry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_cachesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "dopio_cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dopio_cachesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectGuidesimXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectGuidesimRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectGuidesimPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectGuidesimUparmtype) -> Self {
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
    pub fn with_viewportlod(mut self, val: ObjectGuidesimViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectGuidesimVportOnionskin) -> Self {
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
    pub fn with_sourcemode(mut self, val: ObjectGuidesimSourcemode) -> Self {
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
    pub fn with_solver(mut self, val: ObjectGuidesimSolver) -> Self {
        self.base.params.insert(
            "solver".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solver_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extsimsource(mut self, val: ObjectGuidesimExtsimsource) -> Self {
        self.base.params.insert(
            "extsimsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extsimsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extsimsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootconstraint(mut self, val: ObjectGuidesimRootconstraint) -> Self {
        self.base.params.insert(
            "rootconstraint".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rootconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rootconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionhandling(mut self, val: ObjectGuidesimCollisionhandling) -> Self {
        self.base.params.insert(
            "collisionhandling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionhandling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinvdbsource(mut self, val: ObjectGuidesimSkinvdbsource) -> Self {
        self.base.params.insert(
            "skinvdbsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_skinvdbsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinvdbsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extvdbsource(mut self, val: ObjectGuidesimExtvdbsource) -> Self {
        self.base.params.insert(
            "extvdbsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extvdbsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extvdbsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_rootrotation(
        mut self,
        val: ObjectGuidesimVellumconstraintsRootrotation,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_rootrotation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_rootrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_rootrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffnessscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsStretchstiffnessscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsStretchdampingscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsStretchplasticthresholdscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdscalemode_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticratescalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsStretchplasticratescalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticratescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticratescalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticratescalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardeningscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsStretchplastichardeningscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardeningscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardeningscalemode_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardeningscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffnessscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsBendstiffnessscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsBenddampingscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthresholdscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsBendplasticthresholdscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthresholdscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthresholdscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthresholdscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticratescalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsBendplasticratescalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticratescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticratescalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticratescalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardeningscalemode(
        mut self,
        val: ObjectGuidesimVellumconstraintsBendplastichardeningscalemode,
    ) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardeningscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardeningscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardeningscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_trange(mut self, val: ObjectGuidesimDopioTrange) -> Self {
        self.base.params.insert(
            "dopio_trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dopio_trange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_xformtype(mut self, val: ObjectGuidesimDopioXformtype) -> Self {
        self.base.params.insert(
            "dopio_xformtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dopio_xformtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_xformtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_missingframe(mut self, val: ObjectGuidesimDopioMissingframe) -> Self {
        self.base.params.insert(
            "dopio_missingframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dopio_missingframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_missingframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_loadtype(mut self, val: ObjectGuidesimDopioLoadtype) -> Self {
        self.base.params.insert(
            "dopio_loadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dopio_loadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_loadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_packedviewedit(mut self, val: ObjectGuidesimDopioPackedviewedit) -> Self {
        self.base.params.insert(
            "dopio_packedviewedit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dopio_packedviewedit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_packedviewedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_subdstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vm_subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vm_subdgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_subdgroup".to_string(),
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
    pub fn with_shop_geometrypath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_geometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_geometrypath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_geometrypath".to_string(),
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
    pub fn with_activegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "activegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_activegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidesdopobject(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidesdopobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidesdopobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidesdopobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extsimsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "extsimsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extsimsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extsimsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinvdbsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinvdbsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinvdbsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinvdbsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "extsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extvdbsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "extvdbsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extvdbsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extvdbsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffnessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchdampingattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchdampingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticrateattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticrateattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticrateattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticrateattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardeningattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplastichardeningattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffnessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_benddampingattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_benddampingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthresholdattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticthresholdattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticrateattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticrateattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticrateattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticrateattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardeningattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplastichardeningattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_extcollisionsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vellum_extcollisionsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellum_extcollisionsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_extcollisionsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_take(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_take_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_prerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_prerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_lprerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_lprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_preframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_preframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_lpreframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_lpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_postframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_postframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_lpostframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_lpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_postrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_postrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_lpostrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopio_lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopio_lpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_lpostrender".to_string(),
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
    pub fn with_vm_renderable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_renderable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vm_renderable".to_string(),
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
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledynamics(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledynamics".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledynamics_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledynamics".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexternaldopnet(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useexternaldopnet".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexternaldopnet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useexternaldopnet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usefreezeframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usefreezeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usefreezeframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usefreezeframe".to_string(),
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
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computemass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computemass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjustforlength(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adjustforlength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjustforlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adjustforlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjustformass(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adjustformass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjustformass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adjustformass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createclumpconstraints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createclumpconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createclumpconstraints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createclumpconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useclumpguidelimit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useclumpguidelimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclumpguidelimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useclumpguidelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayclumpconstraints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displayclumpconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayclumpconstraints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayclumpconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletarget(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletarget".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletarget_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablegravity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablegravity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablegravity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablegravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablewind(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablewind".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablewind_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablewind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledrag(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledrag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledrag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skincollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "skincollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skincollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skincollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinvdbfillinterior(mut self, val: bool) -> Self {
        self.base.params.insert(
            "skinvdbfillinterior".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skinvdbfillinterior_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinvdbfillinterior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extcollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "extcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extcollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extvdbfillinterior(mut self, val: bool) -> Self {
        self.base.params.insert(
            "extvdbfillinterior".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extvdbfillinterior_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extvdbfillinterior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_userootbendstiffnessratio(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_userootbendstiffnessratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_userootbendstiffnessratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_userootbendstiffnessratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_docompress(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_docompress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_docompress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_docompress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdratio(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_stretchplasticthresholdratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_stretchplasticthresholdratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_bendplasticity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_bendplasticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_enableclump(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_enableclump".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_enableclump_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_enableclump".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumconstraints_clumpdocompress(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpdocompress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellumconstraints_clumpdocompress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumconstraints_clumpdocompress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_gravityenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_gravityenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_gravityenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_gravityenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_windenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_windenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_windenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_windenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforceenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_hairforceenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_hairforceenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforceenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_hairforcecomputevoxelsize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_hairforcecomputevoxelsize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_hairforcecomputevoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_hairforcecomputevoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_enablecollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_enablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_doselfcollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_doselfcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_doselfcollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_doselfcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_collideskin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_collideskin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_collideskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_collideskin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellum_collideext(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vellum_collideext".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vellum_collideext_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellum_collideext".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_loadfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_loadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_loadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_mkpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_mkpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_initsim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_initsim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_alfprogress(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_alfprogress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_reportnetwork(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_reportnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_reportnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_savebackground(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_savebackground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_savebackground_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_savebackground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_tprerender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_tprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_tpreframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_tpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_tpostframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_tpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_tpostrender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_tpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_delayload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_delayload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_delayload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_delayload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopio_prefetch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dopio_prefetch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dopio_prefetch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopio_prefetch".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectGuidesim {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "guidesim"
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
pub trait ObjectGuidesimInnerExt {
    fn display(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn ext_collision_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hairforce_voxelsize(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_extcollision(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_guides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_skin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn in_animguides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_animguides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_animskin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_groom(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_restguides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out_restskin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn prepared_guides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn skin_collision_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn solver_switch(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_create_clump_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_enable_dynamics(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_enable_dynamics2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_colliders(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_curves(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_input_guides(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wire_curves(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn at_least_startframe(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcomposite1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcomposite2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcopy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribpromote1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribpromote2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn clear_orient(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn create_clump_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete_clump_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete_clump_constraints1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn display_clump_constraints1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn enumerate1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn enumerate2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn external_collisions(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn filecache1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn filecache_no_error(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn freezeframe(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn freezeframe1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn freezeframe2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn groomfetch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn groupdelete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn guide_sim(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn hair_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn multiply_dampangular(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn multiply_kangular(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn multiply_width(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn next_step(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn null1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn object_merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pack_interpolated(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn packgroom1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pin_constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointwrangle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointwrangle3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pointwrangle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn polyfill1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn previous_step(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn remove_simulation_attribs(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rotate_input_orient(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rotate_input_orient_by_sim_orient(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn set_gluetoanimation(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn set_pintoanimation1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn shaders(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn shift_to_startframe(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sort1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch11(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch12(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch13(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch14(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch15(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch16(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch17(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch18(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch19(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_usefreezeframe(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_usefreezeframe1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_usefreezeframe2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeshift1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeshift2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn timeshift3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn try_next_step(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn unpackgroom4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn using_active_group(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn using_active_group1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn using_active_group2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn using_active_group3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vdbfrompolygons1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vdbfrompolygons2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_import(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_import_external(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_object_merge(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellum_sim(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vellumglue1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wire_import(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wire_import_external(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn wire_object_merge(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectGuidesimInnerExt for crate::core::graph::InnerGraph<'a, ObjectGuidesim> {
    fn display(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("DISPLAY")
    }
    fn ext_collision_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("EXT_COLLISION_VOLUME")
    }
    fn hairforce_voxelsize(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("HAIRFORCE_VOXELSIZE")
    }
    fn input_extcollision(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("INPUT_EXTCOLLISION")
    }
    fn input_guides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("INPUT_GUIDES")
    }
    fn input_skin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("INPUT_SKIN")
    }
    fn in_animguides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("IN_ANIMGUIDES")
    }
    fn out_animguides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_ANIMGUIDES")
    }
    fn out_animskin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_ANIMSKIN")
    }
    fn out_groom(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_GROOM")
    }
    fn out_restguides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_RESTGUIDES")
    }
    fn out_restskin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("OUT_RESTSKIN")
    }
    fn prepared_guides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("PREPARED_GUIDES")
    }
    fn skin_collision_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SKIN_COLLISION_VOLUME")
    }
    fn solver_switch(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SOLVER_SWITCH")
    }
    fn switch_create_clump_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SWITCH_CREATE_CLUMP_CONSTRAINTS")
    }
    fn switch_enable_dynamics(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SWITCH_ENABLE_DYNAMICS")
    }
    fn switch_enable_dynamics2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("SWITCH_ENABLE_DYNAMICS2")
    }
    fn vellum_colliders(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("VELLUM_COLLIDERS")
    }
    fn vellum_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("VELLUM_CONSTRAINTS")
    }
    fn vellum_curves(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("VELLUM_CURVES")
    }
    fn vellum_input_guides(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("VELLUM_INPUT_GUIDES")
    }
    fn wire_curves(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("WIRE_CURVES")
    }
    fn at_least_startframe(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("at_least_startframe")
    }
    fn attribcomposite1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcomposite1")
    }
    fn attribcomposite2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcomposite2")
    }
    fn attribcopy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcopy1")
    }
    fn attribpromote1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribpromote1")
    }
    fn attribpromote2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribpromote2")
    }
    fn attribwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle1")
    }
    fn clear_orient(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("clear_orient")
    }
    fn create_clump_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("create_clump_constraints")
    }
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete1")
    }
    fn delete2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete2")
    }
    fn delete3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete3")
    }
    fn delete4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete4")
    }
    fn delete5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete5")
    }
    fn delete_clump_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete_clump_constraints")
    }
    fn delete_clump_constraints1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete_clump_constraints1")
    }
    fn display_clump_constraints1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("display_clump_constraints1")
    }
    fn enumerate1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("enumerate1")
    }
    fn enumerate2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("enumerate2")
    }
    fn external_collisions(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("external_collisions")
    }
    fn filecache1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("filecache1")
    }
    fn filecache_no_error(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("filecache_no_error")
    }
    fn freezeframe(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("freezeframe")
    }
    fn freezeframe1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("freezeframe1")
    }
    fn freezeframe2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("freezeframe2")
    }
    fn groomfetch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groomfetch1")
    }
    fn group1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("group1")
    }
    fn group2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("group2")
    }
    fn groupdelete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groupdelete1")
    }
    fn guide_sim(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("guide_sim")
    }
    fn hair_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("hair_constraints")
    }
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge2")
    }
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge3")
    }
    fn multiply_dampangular(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("multiply_dampangular")
    }
    fn multiply_kangular(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("multiply_kangular")
    }
    fn multiply_width(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("multiply_width")
    }
    fn next_step(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("next_step")
    }
    fn null1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn object_merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("object_merge3")
    }
    fn pack_interpolated(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pack_interpolated")
    }
    fn packgroom1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("packgroom1")
    }
    fn pin_constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pin_constraints")
    }
    fn pointwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pointwrangle1")
    }
    fn pointwrangle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pointwrangle2")
    }
    fn pointwrangle3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pointwrangle3")
    }
    fn pointwrangle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pointwrangle4")
    }
    fn polyfill1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("polyfill1")
    }
    fn previous_step(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("previous_step")
    }
    fn remove_simulation_attribs(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("remove_simulation_attribs")
    }
    fn rotate_input_orient(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rotate_input_orient")
    }
    fn rotate_input_orient_by_sim_orient(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rotate_input_orient_by_sim_orient")
    }
    fn set_gluetoanimation(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("set_gluetoanimation")
    }
    fn set_pintoanimation1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("set_pintoanimation1")
    }
    fn shaders(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("shaders")
    }
    fn shift_to_startframe(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("shift_to_startframe")
    }
    fn sort1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sort1")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch10")
    }
    fn switch11(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch11")
    }
    fn switch12(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch12")
    }
    fn switch13(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch13")
    }
    fn switch14(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch14")
    }
    fn switch15(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch15")
    }
    fn switch16(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch16")
    }
    fn switch17(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch17")
    }
    fn switch18(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch18")
    }
    fn switch19(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch19")
    }
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch4")
    }
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch5")
    }
    fn switch6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch6")
    }
    fn switch7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch7")
    }
    fn switch8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch8")
    }
    fn switch9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch9")
    }
    fn switch_usefreezeframe(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_usefreezeframe")
    }
    fn switch_usefreezeframe1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_usefreezeframe1")
    }
    fn switch_usefreezeframe2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_usefreezeframe2")
    }
    fn timeshift1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeshift1")
    }
    fn timeshift2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeshift2")
    }
    fn timeshift3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("timeshift3")
    }
    fn try_next_step(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("try_next_step")
    }
    fn unpackgroom2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom2")
    }
    fn unpackgroom3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom3")
    }
    fn unpackgroom4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("unpackgroom4")
    }
    fn using_active_group(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("using_active_group")
    }
    fn using_active_group1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("using_active_group1")
    }
    fn using_active_group2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("using_active_group2")
    }
    fn using_active_group3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("using_active_group3")
    }
    fn vdbfrompolygons1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vdbfrompolygons1")
    }
    fn vdbfrompolygons2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vdbfrompolygons2")
    }
    fn vellum_import(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vellum_import")
    }
    fn vellum_import_external(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vellum_import_external")
    }
    fn vellum_object_merge(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vellum_object_merge")
    }
    fn vellum_sim(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vellum_sim")
    }
    fn vellumglue1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vellumglue1")
    }
    fn wire_import(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("wire_import")
    }
    fn wire_import_external(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("wire_import_external")
    }
    fn wire_object_merge(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("wire_object_merge")
    }
}
