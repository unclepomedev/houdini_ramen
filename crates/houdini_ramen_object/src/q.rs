#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fPreXform {
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
pub enum ObjectQuadrupedAutoRig4fXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fLayout {
    Tabbed = 0,
    Vertical = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fDeformType {
    Muscles = 0,
    Bones = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fMode {
    CreateNewRig = 0,
    ModifySceneRig = 1,
    MatchAutorigWithSceneRig = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fSourceGeometry {
    FromFile = 0,
    FromScene = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fSymmetry {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fHeadAndNeckEyeSymmetry {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fProxyDisplayGeometry {
    None = 0,
    BoundingGeometry = 1,
    ProxyGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig4fProxyDisplayHandles {
    Hidden = 0,
    Wireframe = 1,
    Shaded = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectQuadrupedAutoRig4f {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectQuadrupedAutoRig4f {
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

    // --- Button parameters ---
    pub fn trigger_set_rig(mut self) -> Self {
        self.params.insert(
            "set_rig".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_proxy_save_geometry(mut self) -> Self {
        self.params.insert(
            "proxy_save_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
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
    pub fn with_character_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "character_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_character_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone2_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone2_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone3_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone3_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone4_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone4_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone5_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone5_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone5_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone5_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone6_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone6_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone6_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone6_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone7_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone7_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone7_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone7_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone8_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone8_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone8_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone8_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone9_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone9_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone9_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone9_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone10_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone10_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone10_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone10_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone11_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone11_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone11_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone11_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone12_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone12_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone12_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone12_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone13_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone13_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone13_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone13_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone14_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone14_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone14_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone14_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone15_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone15_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone15_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone15_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone16_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone16_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone16_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone16_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone17_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone17_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone17_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone17_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone18_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone18_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone18_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone18_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone2_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone2_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone3_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone3_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone4_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone4_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_r(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_r(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_r(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_r(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "control_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control_scale".to_string(),
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
    pub fn with_spine_rig_ctrl_cog_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_cog_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_cog_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_cog_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_ribcage_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_ribcage_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_ribcage_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_ribcage_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_hip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_hip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_hip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_hip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_sacrum_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_sacrum_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_sacrum_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_sacrum_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_head_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_head_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_head_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_head_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_jaw_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_jaw_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_arm_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_arm_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_elbow_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_finger_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_finger_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_knee_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_knee_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_arm_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_arm_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_elbow_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_finger_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_finger_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_knee_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_knee_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail3_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail3_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail3_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail3_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail4_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail4_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail4_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail4_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail5_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail5_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail5_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail5_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail6_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail6_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail6_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail6_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail7_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail7_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail7_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail7_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail8_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail8_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail8_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail8_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail9_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail9_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail9_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail9_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail10_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail10_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail10_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail10_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail11_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail11_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail11_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail11_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone17_cut_control(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone17_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone17_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_s".to_string(),
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
    pub fn with_layout(mut self, val: ObjectQuadrupedAutoRig4fLayout) -> Self {
        self.params.insert(
            "layout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_layout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deform_type(mut self, val: ObjectQuadrupedAutoRig4fDeformType) -> Self {
        self.params.insert(
            "deform_type".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_deform_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deform_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: ObjectQuadrupedAutoRig4fMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_geometry(mut self, val: ObjectQuadrupedAutoRig4fSourceGeometry) -> Self {
        self.params.insert(
            "source_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_source_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_symmetry(mut self, val: ObjectQuadrupedAutoRig4fSymmetry) -> Self {
        self.params.insert(
            "symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_symmetry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_display_geometry(
        mut self,
        val: ObjectQuadrupedAutoRig4fProxyDisplayGeometry,
    ) -> Self {
        self.params.insert(
            "proxy_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_proxy_display_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_display_handles(
        mut self,
        val: ObjectQuadrupedAutoRig4fProxyDisplayHandles,
    ) -> Self {
        self.params.insert(
            "proxy_display_handles".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_proxy_display_handles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy_display_handles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectQuadrupedAutoRig4fPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectQuadrupedAutoRig4fXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectQuadrupedAutoRig4fRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectQuadrupedAutoRig4fUparmtype) -> Self {
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
    pub fn with_head_and_neck_eye_symmetry(
        mut self,
        val: ObjectQuadrupedAutoRig4fHeadAndNeckEyeSymmetry,
    ) -> Self {
        self.params.insert(
            "head_and_neck_eye_symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_head_and_neck_eye_symmetry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_eye_symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_character_name(mut self, val: &str) -> Self {
        self.params.insert(
            "character_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_character_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_library_path(mut self, val: &str) -> Self {
        self.params.insert(
            "library_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_library_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "library_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rig_path(mut self, val: &str) -> Self {
        self.params.insert(
            "rig_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rig_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_file(mut self, val: &str) -> Self {
        self.params.insert(
            "source_from_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_from_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_scene(mut self, val: &str) -> Self {
        self.params.insert(
            "source_from_scene".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_scene_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_from_scene".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_file(mut self, val: &str) -> Self {
        self.params.insert(
            "proxy_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy_file".to_string(),
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
    pub fn with_source_display_geometry(mut self, val: bool) -> Self {
        self.params.insert(
            "source_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_display_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "front_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_left_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "back_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_left_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "front_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_right_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "back_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_right_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_proxy_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "display_proxy_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_proxy_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_proxy_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_wire(mut self, val: bool) -> Self {
        self.params.insert(
            "display_wire".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_wire_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_wire".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_character_placer_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "character_placer_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_character_placer_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "spine_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spine_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "head_and_neck_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_and_neck_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_left_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_right_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_left_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_right_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "tail_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tail_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_left_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_right_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_right_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_left_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_right_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_right_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_display_controls".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ObjectQuadrupedAutoRig4f {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "quadruped_auto_rig_4f"
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
pub trait ObjectQuadrupedAutoRig4fInnerExt {
    fn back_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn back_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn back_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn back_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn character_placer(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn head_and_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn save_proxy_geo(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn source_geometry(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn spine(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tail(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectQuadrupedAutoRig4fInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectQuadrupedAutoRig4f>
{
    fn back_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_left_leg")
    }
    fn back_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_left_toes")
    }
    fn back_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_right_leg")
    }
    fn back_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_right_toes")
    }
    fn character_placer(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("character_placer")
    }
    fn front_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_left_leg")
    }
    fn front_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_left_toes")
    }
    fn front_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_right_leg")
    }
    fn front_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_right_toes")
    }
    fn head_and_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("head_and_neck")
    }
    fn save_proxy_geo(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("save_proxy_geo")
    }
    fn source_geometry(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("source_geometry")
    }
    fn spine(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("spine")
    }
    fn tail(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tail")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fPreXform {
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
pub enum ObjectQuadrupedAutoRig5fXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fLayout {
    Tabbed = 0,
    Vertical = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fDeformType {
    Muscles = 0,
    Bones = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fMode {
    CreateNewRig = 0,
    ModifySceneRig = 1,
    MatchAutorigWithSceneRig = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fSourceGeometry {
    FromFile = 0,
    FromScene = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fSymmetry {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fHeadAndNeckEyeSymmetry {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fProxyDisplayGeometry {
    None = 0,
    BoundingGeometry = 1,
    ProxyGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectQuadrupedAutoRig5fProxyDisplayHandles {
    Hidden = 0,
    Wireframe = 1,
    Shaded = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectQuadrupedAutoRig5f {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectQuadrupedAutoRig5f {
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

    // --- Button parameters ---
    pub fn trigger_set_rig(mut self) -> Self {
        self.params.insert(
            "set_rig".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_proxy_save_geometry(mut self) -> Self {
        self.params.insert(
            "proxy_save_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
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
    pub fn with_character_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "character_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_character_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone2_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone2_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone3_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone3_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone4_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone4_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone5_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone5_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone5_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone5_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone6_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone6_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone6_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone6_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone7_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone7_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone7_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone7_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone8_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone8_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone8_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone8_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone9_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone9_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone9_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone9_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone10_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone10_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone10_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone10_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone11_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone11_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone11_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone11_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone12_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone12_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone12_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone12_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone13_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone13_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone13_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone13_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone14_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone14_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone14_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone14_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone15_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone15_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone15_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone15_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone16_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone16_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone16_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone16_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone17_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone17_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone17_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone17_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone18_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone18_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_bone18_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_bone18_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone2_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone2_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone2_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone3_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone3_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone3_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone4_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_bone4_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_bone4_pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_r(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_r(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_r(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_r(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_tip_tz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "control_scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control_scale".to_string(),
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
    pub fn with_spine_rig_ctrl_cog_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_cog_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_cog_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_cog_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_ribcage_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_ribcage_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_ribcage_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_ribcage_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_ribcage2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_ribcage2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_hip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_hip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_hip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_spine_hip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_spine_hip_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_rig_ctrl_sacrum_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_rig_ctrl_sacrum_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_sacrum_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_rig_ctrl_sacrum_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_head_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_head_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_head_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_head_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_jaw_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_jaw_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_left_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_right_eye_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_rig_ctrl_neck_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_arm_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_arm_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_elbow_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_palm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_finger_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_rig_ctrl_finger_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_knee_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_knee_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_rig_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_ring_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_ring_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_arm_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_arm_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_arm_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulder_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_shoulderblade_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_elbow_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_elbow_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_wrist_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_palm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_palm_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_finger_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_rig_ctrl_finger_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_rig_ctrl_finger_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_thigh_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_knee_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_knee_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_knee_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ankle_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_ball_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_rig_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_rig_ctrl_toe_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_ring_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_thumb_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_index_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_middle_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_ring_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_ring_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_root_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle1_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_rig_ctrl_pinky_middle2_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail1_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail2_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail3_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail3_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail3_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail3_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail4_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail4_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail4_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail4_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail5_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail5_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail5_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail5_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail6_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail6_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail6_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail6_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail7_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail7_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail7_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail7_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail8_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail8_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail8_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail8_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail9_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail9_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail9_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail9_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail10_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail10_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail10_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail10_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail11_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail11_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_rig_ctrl_tail11_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_rig_ctrl_tail11_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone3_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone5_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone7_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone9_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone11_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone13_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone13_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone15_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone15_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone17_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_bone17_cut_control(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_bone17_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_bone17_cut_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_bone17_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_spine_end_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_spine_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_sacrum_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_sacrum_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone3_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_bone4_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_bone4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_end_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_head_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulderblade_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_shoulder_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_elbow_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_wrist_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_palm_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_leg_finger_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_thigh_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_knee_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ankle_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_ball_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_leg_toe_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle2_cut_control(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_middle2_cut_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_ring_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_ring_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_left_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle2_cut_control(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_middle2_cut_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_ring_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_ring_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_left_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulderblade_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulderblade_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_shoulder_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_shoulder_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_elbow_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_elbow_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_wrist_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_wrist_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_palm_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_palm_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_leg_finger_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_finger_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_thigh_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_thigh_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_knee_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_knee_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ankle_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ankle_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_ball_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_ball_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_leg_toe_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_toe_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle2_cut_control(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_middle2_cut_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_ring_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_ring_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_front_right_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_middle2_cut_control_s2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_thumb_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_index_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_middle_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle2_cut_control(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_middle2_cut_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_middle2_cut_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_ring_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_ring_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_ring_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_root_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_middle2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_back_right_toes_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_pinky_tip_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail1_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail2_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail3_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail3_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail4_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail4_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail5_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail5_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail6_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail6_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail7_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail7_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail8_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail8_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail9_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail9_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail10_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail10_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail11_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail11_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tail_tail_end_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_tail_end_cut_control_s".to_string(),
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
    pub fn with_layout(mut self, val: ObjectQuadrupedAutoRig5fLayout) -> Self {
        self.params.insert(
            "layout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_layout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deform_type(mut self, val: ObjectQuadrupedAutoRig5fDeformType) -> Self {
        self.params.insert(
            "deform_type".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_deform_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deform_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: ObjectQuadrupedAutoRig5fMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_geometry(mut self, val: ObjectQuadrupedAutoRig5fSourceGeometry) -> Self {
        self.params.insert(
            "source_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_source_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_symmetry(mut self, val: ObjectQuadrupedAutoRig5fSymmetry) -> Self {
        self.params.insert(
            "symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_symmetry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_display_geometry(
        mut self,
        val: ObjectQuadrupedAutoRig5fProxyDisplayGeometry,
    ) -> Self {
        self.params.insert(
            "proxy_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_proxy_display_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_display_handles(
        mut self,
        val: ObjectQuadrupedAutoRig5fProxyDisplayHandles,
    ) -> Self {
        self.params.insert(
            "proxy_display_handles".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_proxy_display_handles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy_display_handles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectQuadrupedAutoRig5fPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectQuadrupedAutoRig5fXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectQuadrupedAutoRig5fRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectQuadrupedAutoRig5fUparmtype) -> Self {
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
    pub fn with_head_and_neck_eye_symmetry(
        mut self,
        val: ObjectQuadrupedAutoRig5fHeadAndNeckEyeSymmetry,
    ) -> Self {
        self.params.insert(
            "head_and_neck_eye_symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_head_and_neck_eye_symmetry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_eye_symmetry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_character_name(mut self, val: &str) -> Self {
        self.params.insert(
            "character_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_character_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_library_path(mut self, val: &str) -> Self {
        self.params.insert(
            "library_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_library_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "library_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rig_path(mut self, val: &str) -> Self {
        self.params.insert(
            "rig_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rig_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_file(mut self, val: &str) -> Self {
        self.params.insert(
            "source_from_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_from_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_scene(mut self, val: &str) -> Self {
        self.params.insert(
            "source_from_scene".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_from_scene_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_from_scene".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_file(mut self, val: &str) -> Self {
        self.params.insert(
            "proxy_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy_file".to_string(),
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
    pub fn with_source_display_geometry(mut self, val: bool) -> Self {
        self.params.insert(
            "source_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_display_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_display_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "front_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_left_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "back_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_left_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "front_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_right_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_disable_toe(mut self, val: bool) -> Self {
        self.params.insert(
            "back_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_right_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_disable_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_proxy_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "display_proxy_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_proxy_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_proxy_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_wire(mut self, val: bool) -> Self {
        self.params.insert(
            "display_wire".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_wire_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_wire".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_character_placer_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "character_placer_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_character_placer_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spine_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "spine_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spine_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_head_and_neck_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "head_and_neck_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_and_neck_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_left_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_right_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_left_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_right_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_leg_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tail_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "tail_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tail_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tail_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_left_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_left_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_front_right_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "front_right_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_front_right_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "front_right_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_left_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_left_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_left_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_back_right_toes_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "back_right_toes_display_controls".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_back_right_toes_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "back_right_toes_display_controls".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ObjectQuadrupedAutoRig5f {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "quadruped_auto_rig_5f"
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
pub trait ObjectQuadrupedAutoRig5fInnerExt {
    fn back_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn back_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn back_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn back_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn character_placer(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn front_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn head_and_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn save_proxy_geo(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn source_geometry(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn spine(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tail(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectQuadrupedAutoRig5fInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectQuadrupedAutoRig5f>
{
    fn back_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_left_leg")
    }
    fn back_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_left_toes")
    }
    fn back_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_right_leg")
    }
    fn back_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("back_right_toes")
    }
    fn character_placer(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("character_placer")
    }
    fn front_left_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_left_leg")
    }
    fn front_left_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_left_toes")
    }
    fn front_right_leg(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_right_leg")
    }
    fn front_right_toes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("front_right_toes")
    }
    fn head_and_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("head_and_neck")
    }
    fn save_proxy_geo(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("save_proxy_geo")
    }
    fn source_geometry(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("source_geometry")
    }
    fn spine(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("spine")
    }
    fn tail(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tail")
    }
}
