#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigPreXform {
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
pub enum ObjectBipedAutoRigXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigLayout {
    Tabbed = 0,
    Vertical = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigDeformType {
    Muscles = 0,
    Bones = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigMode {
    CreateNewRig = 0,
    ModifySceneRig = 1,
    MatchAutorigWithSceneRig = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigSourceGeometry {
    FromFile = 0,
    FromScene = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigSymmetry {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigHeadAndNeckEyeSymmetry {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigProxyDisplayGeometry {
    None = 0,
    BoundingGeometry = 1,
    ProxyGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBipedAutoRigProxyDisplayHandles {
    Hidden = 0,
    Wireframe = 1,
    Shaded = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectBipedAutoRig {
    pub base: crate::core::types::NodeBase,
}

impl ObjectBipedAutoRig {
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
    pub fn trigger_set_rig(mut self) -> Self {
        self.base.params.insert(
            "set_rig".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_proxy_save_geometry(mut self) -> Self {
        self.base.params.insert(
            "proxy_save_geometry".to_string(),
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
    pub fn with_character_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "character_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_character_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "character_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_hand_direction_tx(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_arm_rig_hand_direction_tx".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_rig_hand_direction_tx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_hand_direction_tx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_hand_up_ty(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_arm_rig_hand_up_ty".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_rig_hand_up_ty_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_hand_up_ty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_hand_direction_tx(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_arm_rig_hand_direction_tx".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_rig_hand_direction_tx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_hand_direction_tx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_hand_up_ty(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_arm_rig_hand_up_ty".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_rig_hand_up_ty_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_hand_up_ty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle1_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle1_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle1_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle1_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle2_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle2_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle2_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle2_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_tip_tz(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_tip_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_tip_tz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_tip_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_ankle_ry(mut self, val: f32) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_ankle_ry".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_ankle_ry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_ankle_ry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_ankle_ry(mut self, val: f32) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_ankle_ry".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_ankle_ry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_ankle_ry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_control_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "control_scale".to_string(),
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
    pub fn with_spine_rig_ctrl_cog_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_cog_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_cog_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_cog_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_rig_ctrl_hip_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_hip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_hip_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_hip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_rig_ctrl_lower_back2_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_lower_back2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_lower_back2_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_lower_back2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_rig_ctrl_chest_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_chest_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_chest_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_chest_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_rig_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_neck_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_rig_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_rig_ctrl_neck_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_neck_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_neck_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_neck_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_neck_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_head_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_head_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_head_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_head_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_jaw_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_jaw_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_end_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_jaw_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_jaw_end_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_jaw_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_left_eye_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_left_eye_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_left_eye_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_left_eye_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_left_eye_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_left_eye_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_left_eye_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_right_eye_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_right_eye_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_right_eye_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_right_eye_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_right_eye_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_rig_ctrl_right_eye_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_rig_ctrl_right_eye_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_collarbone_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_collarbone_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_shoulder_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_shoulder_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_elbow_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_elbow_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_elbow_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_wrist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_wrist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_wrist_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_wrist_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_rig_ctrl_wrist_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_rig_ctrl_wrist_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_collarbone_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_collarbone_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_shoulder_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_shoulder_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_elbow_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_elbow_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_elbow_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_wrist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_wrist_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_wrist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_wrist_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_wrist_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_rig_ctrl_wrist_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_rig_ctrl_wrist_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_thumb_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_thumb_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_index_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_middle_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_ring_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_ring_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_rig_ctrl_pinky_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_thumb_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_thumb_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_index_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_index_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_middle_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_middle_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_ring_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_ring_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle1_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_rig_ctrl_pinky_middle2_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_rig_ctrl_pinky_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_thigh_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_knee_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_knee_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_knee_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_knee_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_ball_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_ball_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_rig_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_rig_ctrl_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_thigh_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_knee_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_knee_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_knee_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_knee_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_ankle_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_ball_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_ball_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_ball_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_ball_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_toe_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_rig_ctrl_toe_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_rig_ctrl_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_pelvis_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_pelvis_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_pelvis_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_pelvis_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_pelvis_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_pelvis_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_pelvis_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_pelvis_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_pelvis_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_pelvis_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_pelvis_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_pelvis_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_hip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_hip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_hip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_hip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_hip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_hip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_hip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_hip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_hip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_hip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_hip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_hip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_lower_back2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_lower_back2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_lower_back2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_lower_back2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_lower_back2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_lower_back2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_lower_back2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_lower_back2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_lower_back2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_lower_back2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_lower_back2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_lower_back2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_chest_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_chest_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_chest_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_chest_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_chest_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_chest_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_chest_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_chest_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_chest_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_chest_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_chest_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_chest_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_neck_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_neck_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_neck_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_neck_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_neck_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_neck_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_neck_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_neck_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_neck_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spine_neck_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_neck_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_neck_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_neck_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_neck_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_neck_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_neck_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_neck_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_neck_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_neck_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_neck_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_neck_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_neck_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_neck_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_neck_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_neck_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_head_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_head_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_head_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_head_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_head_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_head_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_head_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_jaw_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_jaw_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_jaw_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_jaw_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_jaw_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_jaw_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_jaw_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_jaw_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_jaw_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "head_and_neck_jaw_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_jaw_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_jaw_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_shoulder_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_shoulder_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_shoulder_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_shoulder_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_shoulder_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_shoulder_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_shoulder_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_shoulder_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_shoulder_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_shoulder_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_shoulder_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_shoulder_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_elbow_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_elbow_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_elbow_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_elbow_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_elbow_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_elbow_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_elbow_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_elbow_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_elbow_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_elbow_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_elbow_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_elbow_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_wrist_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_wrist_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_wrist_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_wrist_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_wrist_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_wrist_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_wrist_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_wrist_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_wrist_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_arm_wrist_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_wrist_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_wrist_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_shoulder_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_shoulder_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_shoulder_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_shoulder_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_shoulder_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_shoulder_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_shoulder_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_shoulder_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_shoulder_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_shoulder_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_shoulder_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_shoulder_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_elbow_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_elbow_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_elbow_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_elbow_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_elbow_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_elbow_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_elbow_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_elbow_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_elbow_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_elbow_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_elbow_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_elbow_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_wrist_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_wrist_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_wrist_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_wrist_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_wrist_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_wrist_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_wrist_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_wrist_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_wrist_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_arm_wrist_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_wrist_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_wrist_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_palm_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_palm_end_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_palm_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_palm_end_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_palm_endcut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_palm_endcut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_palm_endcut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_palm_endcut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_palm_endcut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_palm_endcut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_palm_endcut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_palm_endcut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_s2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_middle2_cut_control_s2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_thumb_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_thumb_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_index_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_index_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_middle_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_middle_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ring_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_ring_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ring_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_ring_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_hand_pinky_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_pinky_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_palm_end_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_palm_end_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_palm_end_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_palm_end_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_palm_endcut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_palm_endcut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_palm_endcut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_palm_endcut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_palm_endcut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_palm_endcut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_palm_endcut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_palm_endcut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_s2(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_s2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_middle2_cut_control_s2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_middle2_cut_control_s2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_thumb_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_thumb_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_thumb_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_thumb_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_index_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_index_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_index_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_index_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_middle_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_middle_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_middle_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_middle_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ring_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_ring_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ring_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_ring_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_root_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_root_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_root_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_root_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_root_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_root_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_root_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_root_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_root_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_middle1_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_middle1_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle1_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_middle1_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_middle1_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle1_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_middle1_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_middle1_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle1_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_middle2_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_middle2_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle2_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_middle2_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_middle2_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle2_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_middle2_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_middle2_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_middle2_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_tip_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_tip_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_tip_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_tip_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_tip_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_tip_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_pinky_tip_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_hand_pinky_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_pinky_tip_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_pinky_tip_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_thigh_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_thigh_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_thigh_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_thigh_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_thigh_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_thigh_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_thigh_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_thigh_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_thigh_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_thigh_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_thigh_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_thigh_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_knee_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_knee_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_knee_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_knee_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_knee_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_knee_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_knee_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_knee_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_knee_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_knee_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_knee_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_knee_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ankle_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_ankle_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ankle_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_ankle_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_ankle_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_ankle_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ankle_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_ankle_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ankle_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_ankle_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ball_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_ball_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ball_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_ball_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ball_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_ball_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ball_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_ball_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ball_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_ball_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ball_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_ball_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_toe_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_toe_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_toe_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_toe_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_toe_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_toe_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_toe_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "left_leg_toe_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_toe_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_toe_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_thigh_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_thigh_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_thigh_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_thigh_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_thigh_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_thigh_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_thigh_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_thigh_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_thigh_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_thigh_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_thigh_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_thigh_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_knee_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_knee_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_knee_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_knee_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_knee_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_knee_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_knee_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_knee_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_knee_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_knee_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_knee_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_knee_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ankle_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_ankle_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ankle_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_ankle_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ankle_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_ankle_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ankle_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_ankle_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ankle_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_ankle_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ankle_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_ankle_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ball_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_ball_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ball_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_ball_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ball_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_ball_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ball_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_ball_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ball_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_ball_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ball_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_ball_cut_control_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_toe_cut_control_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_toe_cut_control_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_toe_cut_control_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_toe_cut_control_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_toe_cut_control_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_toe_cut_control_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_toe_cut_control_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_toe_cut_control_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_toe_cut_control_s(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "right_leg_toe_cut_control_s".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_toe_cut_control_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_toe_cut_control_s".to_string(),
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
    pub fn with_layout(mut self, val: ObjectBipedAutoRigLayout) -> Self {
        self.base.params.insert(
            "layout".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_layout_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "layout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deform_type(mut self, val: ObjectBipedAutoRigDeformType) -> Self {
        self.base.params.insert(
            "deform_type".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_deform_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deform_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: ObjectBipedAutoRigMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_geometry(mut self, val: ObjectBipedAutoRigSourceGeometry) -> Self {
        self.base.params.insert(
            "source_geometry".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_source_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symmetry(mut self, val: ObjectBipedAutoRigSymmetry) -> Self {
        self.base.params.insert(
            "symmetry".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_symmetry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "symmetry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_proxy_display_geometry(
        mut self,
        val: ObjectBipedAutoRigProxyDisplayGeometry,
    ) -> Self {
        self.base.params.insert(
            "proxy_display_geometry".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_proxy_display_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "proxy_display_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_proxy_display_handles(
        mut self,
        val: ObjectBipedAutoRigProxyDisplayHandles,
    ) -> Self {
        self.base.params.insert(
            "proxy_display_handles".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_proxy_display_handles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "proxy_display_handles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectBipedAutoRigPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectBipedAutoRigXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectBipedAutoRigRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectBipedAutoRigUparmtype) -> Self {
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
    pub fn with_head_and_neck_eye_symmetry(
        mut self,
        val: ObjectBipedAutoRigHeadAndNeckEyeSymmetry,
    ) -> Self {
        self.base.params.insert(
            "head_and_neck_eye_symmetry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_head_and_neck_eye_symmetry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_eye_symmetry".to_string(),
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
    pub fn with_character_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "character_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_character_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "character_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_library_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "library_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_library_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "library_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rig_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rig_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rig_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rig_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_from_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "source_from_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_from_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source_from_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_from_scene(mut self, val: &str) -> Self {
        self.base.params.insert(
            "source_from_scene".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_from_scene_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source_from_scene".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_proxy_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "proxy_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_proxy_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "proxy_file".to_string(),
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
    pub fn with_source_display_geometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "source_display_geometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_display_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source_display_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_disable_toe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "left_leg_disable_toe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_disable_toe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_disable_toe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "right_leg_disable_toe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_leg_disable_toe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_disable_toe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_proxy_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "display_proxy_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_proxy_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display_proxy_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_wire(mut self, val: bool) -> Self {
        self.base.params.insert(
            "display_wire".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_wire_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "display_wire".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "character_placer_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_character_placer_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "character_placer_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "spine_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spine_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spine_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "head_and_neck_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_and_neck_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "head_and_neck_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "left_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_arm_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "right_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_arm_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "left_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_hand_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "right_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_hand_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "left_leg_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_leg_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_display_controls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "right_leg_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_leg_display_controls".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectBipedAutoRig {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "biped_auto_rig"
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
pub trait ObjectBipedAutoRigInnerExt {
    fn character_placer(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_and_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_arm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_hand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_leg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_arm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_hand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_leg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn save_proxy_geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn source_geometry(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectBipedAutoRigInnerExt for crate::core::graph::InnerGraph<'a, ObjectBipedAutoRig> {
    fn character_placer(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("character_placer")
    }
    fn head_and_neck(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("head_and_neck")
    }
    fn left_arm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("left_arm")
    }
    fn left_hand(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("left_hand")
    }
    fn left_leg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("left_leg")
    }
    fn right_arm(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("right_arm")
    }
    fn right_hand(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("right_hand")
    }
    fn right_leg(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("right_leg")
    }
    fn save_proxy_geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("save_proxy_geo")
    }
    fn source_geometry(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("source_geometry")
    }
    fn spine(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("spine")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendShortrotblend {
    Euler = 0,
    /// Shortest Path (Linear)
    ShortestPathLinear = 1,
    /// Shortest Path (Fast Linear)
    ShortestPathFastLinear = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendm1 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendrestord1 {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendm2 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendrestord2 {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendm3 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendrestord3 {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendm4 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendBlendrestord4 {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendPreXform {
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
pub enum ObjectBlendUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectBlend {
    pub base: crate::core::types::NodeBase,
}

impl ObjectBlend {
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
    pub fn with_blendw1(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendw1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendw1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendw1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendw2(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendw2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendw2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendw2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendw3(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendw3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendw3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendw3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendw4(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendw4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendw4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendw4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "noffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "noffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sequence_seq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sequence_seq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sequence_seq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sequence_seq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sequence_con(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sequence_con".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sequence_con_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sequence_con".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reset".to_string(),
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
    pub fn with_blendrest1(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "blendrest1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_blendrest1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrest1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrest2(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "blendrest2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_blendrest2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrest2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrest3(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "blendrest3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_blendrest3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrest3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrest4(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "blendrest4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_blendrest4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrest4".to_string(),
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
    pub fn with_shortrotblend(mut self, val: ObjectBlendShortrotblend) -> Self {
        self.base.params.insert(
            "shortrotblend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shortrotblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shortrotblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendm1(mut self, val: ObjectBlendBlendm1) -> Self {
        self.base.params.insert(
            "blendm1".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendm1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendm1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrestord1(mut self, val: ObjectBlendBlendrestord1) -> Self {
        self.base.params.insert(
            "blendrestord1".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendrestord1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrestord1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendm2(mut self, val: ObjectBlendBlendm2) -> Self {
        self.base.params.insert(
            "blendm2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendm2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendm2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrestord2(mut self, val: ObjectBlendBlendrestord2) -> Self {
        self.base.params.insert(
            "blendrestord2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendrestord2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrestord2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendm3(mut self, val: ObjectBlendBlendm3) -> Self {
        self.base.params.insert(
            "blendm3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendm3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendm3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrestord3(mut self, val: ObjectBlendBlendrestord3) -> Self {
        self.base.params.insert(
            "blendrestord3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendrestord3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrestord3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendm4(mut self, val: ObjectBlendBlendm4) -> Self {
        self.base.params.insert(
            "blendm4".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendm4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendm4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendrestord4(mut self, val: ObjectBlendBlendrestord4) -> Self {
        self.base.params.insert(
            "blendrestord4".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendrestord4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendrestord4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectBlendXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectBlendRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectBlendPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectBlendUparmtype) -> Self {
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

    // --- Toggle parameters ---
    pub fn with_userestangles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "userestangles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userestangles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "userestangles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axesorient(mut self, val: bool) -> Self {
        self.base.params.insert(
            "axesorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_axesorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axesorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shortrot(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shortrot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shortrot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shortrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignorescales(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ignorescales".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignorescales_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ignorescales".to_string(),
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
}

impl crate::core::types::HoudiniNode for ObjectBlend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "blend"
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
pub trait ObjectBlendInnerExt {
    fn point1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sphere1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectBlendInnerExt for crate::core::graph::InnerGraph<'a, ObjectBlend> {
    fn point1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("point1")
    }
    fn sphere1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sphere1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendstickyMask {
    U = 0,
    V = 1,
    R = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendstickyDisplayicon {
    Icon = 0,
    Axis = 1,
    IconAndAxis = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendstickyControltype {
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
pub enum ObjectBlendstickyOrientation {
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
pub enum ObjectBlendstickyXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendstickyRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBlendstickyPreXform {
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
pub enum ObjectBlendstickyUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectBlendsticky {
    pub base: crate::core::types::NodeBase,
}

impl ObjectBlendsticky {
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

    // --- Button parameters ---
    pub fn trigger_stickyclearuv(mut self) -> Self {
        self.base.params.insert(
            "stickyclearuv".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blend_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "geoscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_geoscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoscale".to_string(),
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
    pub fn with_stickyurange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "stickyurange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stickyurange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickyurange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickyvrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "stickyvrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stickyvrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickyvrange".to_string(),
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
    pub fn with_geosize(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "geosize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_geosize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geosize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geocenter(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "geocenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_geocenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geocenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georotate(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "georotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_georotate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georotate".to_string(),
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
    pub fn with_mask_inst(mut self, index1: usize, val: ObjectBlendstickyMask) -> Self {
        self.base.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayicon(mut self, val: ObjectBlendstickyDisplayicon) -> Self {
        self.base.params.insert(
            "displayicon".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_displayicon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayicon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_controltype(mut self, val: ObjectBlendstickyControltype) -> Self {
        self.base.params.insert(
            "controltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_controltype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "controltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientation(mut self, val: ObjectBlendstickyOrientation) -> Self {
        self.base.params.insert(
            "orientation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orientation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectBlendstickyXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectBlendstickyRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectBlendstickyPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectBlendstickyUparmtype) -> Self {
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

    // --- String parameters ---
    pub fn with_stickyattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stickyattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stickyattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickyattrib".to_string(),
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
    pub fn with_geocustom(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geocustom".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geocustom_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geocustom".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_stickyorient(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickyorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickyorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickyorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fetchworld(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fetchworld".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fetchworld_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fetchworld".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickywrapu(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickywrapu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickywrapu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickywrapu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickywrapv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickywrapv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickywrapv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickywrapv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickyuvconstant(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickyuvconstant".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickyuvconstant_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickyuvconstant".to_string(),
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
    pub fn with_renderspace(mut self, val: bool) -> Self {
        self.base.params.insert(
            "renderspace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "renderspace".to_string(),
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
    pub fn with_shadedmode(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shadedmode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadedmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadedmode".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectBlendsticky {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "blendsticky"
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
pub trait ObjectBlendstickyInnerExt {
    fn control1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn point1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectBlendstickyInnerExt for crate::core::graph::InnerGraph<'a, ObjectBlendsticky> {
    fn control1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("control1")
    }
    fn point1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("point1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBonePreXform {
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
pub enum ObjectBoneUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBoneXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectBoneRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct ObjectBone {
    pub base: crate::core::types::NodeBase,
}

impl ObjectBone {
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
    pub fn with_ikdamp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ikdamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikdamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ikdamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xdamp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "xdamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xdamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xdamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xrolloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "xrolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xrolloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xrolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ydamp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ydamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ydamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ydamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yrolloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "yrolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yrolloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yrolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zdamp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zdamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zdamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zdamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zrolloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zrolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zrolloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zrolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrtopheight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ccrtopheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ccrtopheight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrtopheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrbotheight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ccrbotheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ccrbotheight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrbotheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crtopheight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "crtopheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_crtopheight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crtopheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crbotheight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "crbotheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_crbotheight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crbotheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captposelen(mut self, val: f32) -> Self {
        self.base.params.insert(
            "captposelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_captposelen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "captposelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_xrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "xrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_xrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "yrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_yrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "zrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_zrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zrange".to_string(),
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
    pub fn with_r_1(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("R".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "R".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrcenter(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ccrcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ccrcenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrrotate(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ccrrotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ccrrotate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrscale(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ccrscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ccrscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrtopcap(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ccrtopcap".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ccrtopcap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrtopcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ccrbotcap(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ccrbotcap".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ccrbotcap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ccrbotcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crcenter(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "crcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_crcenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crrotate(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "crrotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_crrotate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crscale(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "crscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_crscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crtopcap(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "crtopcap".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_crtopcap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crtopcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crbotcap(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "crbotcap".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_crbotcap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "crbotcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captposet(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "captposet".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_captposet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "captposet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captposer(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "captposer".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_captposer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "captposer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captposes(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "captposes".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_captposes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "captposes".to_string(),
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
    pub fn with_pre_xform(mut self, val: ObjectBonePreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectBoneUparmtype) -> Self {
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
    pub fn with_xord(mut self, val: ObjectBoneXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectBoneRord) -> Self {
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
    pub fn with_solver(mut self, val: &str) -> Self {
        self.base.params.insert(
            "solver".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_displaylink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displaylink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaylink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaylink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaycapture(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displaycapture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaycapture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaycapture".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectBone {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bone"
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
pub trait ObjectBoneInnerExt {
    fn bonelink(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cregion(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectBoneInnerExt for crate::core::graph::InnerGraph<'a, ObjectBone> {
    fn bonelink(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bonelink")
    }
    fn cregion(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("cregion")
    }
}
