#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightPreXform {
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
pub enum ObjectThreePointLightUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightKeyLightLightType {
    PointLight = 0,
    SpotLight = 1,
    DistantLight = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightKeyLightShadowType {
    NoShadows = 0,
    /// Ray-Traced Shadows
    RayMinusTracedShadows = 1,
    DepthMapShadows = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightKeyLightShadowmapResmenu {
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
pub enum ObjectThreePointLightFillLightLightType {
    PointLight = 0,
    SpotLight = 1,
    DistantLight = 2,
    AreaLight = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightFillLightAreashape {
    /// Line/Tube
    LineTube = 0,
    Grid = 1,
    Disk = 2,
    Sphere = 3,
    Geometry = 4,
    /// Environment (Deprecated)
    EnvironmentDeprecated = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightFillLightShadowType {
    NoShadows = 0,
    /// Ray-Traced Shadows
    RayMinusTracedShadows = 1,
    DepthMapShadows = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightFillLightShadowmapResmenu {
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
pub enum ObjectThreePointLightRimLightLightType {
    PointLight = 0,
    SpotLight = 1,
    DistantLight = 2,
    AreaLight = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightRimLightShadowType {
    NoShadows = 0,
    /// Ray-Traced Shadows
    RayMinusTracedShadows = 1,
    DepthMapShadows = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightRimLightShadowmapResmenu {
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
pub enum ObjectThreePointLightBounceLightLightType {
    PointLight = 0,
    SpotLight = 1,
    DistantLight = 2,
    AreaLight = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightBounceLightAttenType {
    NoAttenuation = 0,
    HalfDistanceAttenuation = 1,
    PhysicallyCorrect = 2,
    Quadratic = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightBounceLightAreashape {
    /// Line/Tube
    LineTube = 0,
    Grid = 1,
    Disk = 2,
    Sphere = 3,
    Geometry = 4,
    /// Environment (Deprecated)
    EnvironmentDeprecated = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightBounceLightShadowType {
    NoShadows = 0,
    /// Ray-Traced Shadows
    RayMinusTracedShadows = 1,
    DepthMapShadows = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreePointLightBounceLightShadowmapResmenu {
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

#[derive(Debug, Clone)]
pub struct ObjectThreePointLight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectThreePointLight {
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

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_sub_network_input_1_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_key_light_light_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_light_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_conedelta(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_conedelta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_conedelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_conedelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_coneroll(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_coneroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_coneroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_coneroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_light_fov(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_light_fov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_light_fov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_fov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_orthowidth(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_orthowidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_shadow_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_shadow_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_shadow_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_shadow_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_shadow_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_near(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_far(mut self, val: f32) -> Self {
        self.params.insert(
            "key_light_far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_key_light_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_far".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_light_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_conedelta(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_conedelta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_conedelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_conedelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_coneroll(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_coneroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_coneroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_coneroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_fov(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_light_fov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_light_fov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_fov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_orthowidth(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_orthowidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areasize(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_areasize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_areasize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areasize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_shadow_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_shadow_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_shadow_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_shadow_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_shadow_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_near(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_far(mut self, val: f32) -> Self {
        self.params.insert(
            "fill_light_far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_light_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_far".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_light_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_conedelta(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_conedelta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_conedelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_conedelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_coneroll(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_coneroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_coneroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_coneroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_fov(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_light_fov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_light_fov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_fov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_orthowidth(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_orthowidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_shadow_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_shadow_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_shadow_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_shadow_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_shadow_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_near(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_far(mut self, val: f32) -> Self {
        self.params.insert(
            "rim_light_far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rim_light_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_far".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_light_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_atten_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_atten_dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_atten_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_atten_dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_atten_0(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_atten_0".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_atten_0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_atten_0".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_atten_1(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_atten_1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_atten_1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_atten_1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_atten_2(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_atten_2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_atten_2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_atten_2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_conedelta(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_conedelta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_conedelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_conedelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_coneroll(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_coneroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_coneroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_coneroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_fov(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_light_fov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_light_fov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_fov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_orthowidth(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_orthowidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_orthowidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areasize(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_areasize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_areasize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areasize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_shadow_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_shadow_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_shadow_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_shadow_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_shadow_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_near(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_far(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce_light_far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_light_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_far".to_string(),
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
    pub fn with_look_at_target(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "look_at_target".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_look_at_target_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "look_at_target".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_direction_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key_direction_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key_direction_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_direction_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_l_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key_light_l_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key_light_l_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_l_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_l_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key_light_l_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key_light_l_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_l_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "key_light_light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_key_light_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_direction_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fill_direction_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fill_direction_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_direction_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_l_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fill_light_l_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fill_light_l_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_l_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_l_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fill_light_l_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fill_light_l_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_l_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fill_light_light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fill_light_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_direction_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rim_direction_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rim_direction_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_direction_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_l_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rim_light_l_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rim_light_l_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_l_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_l_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rim_light_l_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rim_light_l_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_l_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rim_light_light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rim_light_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_direction_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bounce_direction_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bounce_direction_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_direction_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_l_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bounce_light_l_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bounce_light_l_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_l_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_l_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bounce_light_l_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bounce_light_l_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_l_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bounce_light_light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bounce_light_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_color".to_string(),
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
    pub fn with_fill_light_areasamples(mut self, val: i32) -> Self {
        self.params.insert(
            "fill_light_areasamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fill_light_areasamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areasamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areasamples(mut self, val: i32) -> Self {
        self.params.insert(
            "bounce_light_areasamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounce_light_areasamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areasamples".to_string(),
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

    // --- Int2 parameters ---
    pub fn with_key_light_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "key_light_res".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_key_light_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadowmap_samples(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "key_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_key_light_shadowmap_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "fill_light_res".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_fill_light_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadowmap_samples(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "fill_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_fill_light_shadowmap_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "rim_light_res".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_rim_light_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadowmap_samples(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "rim_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_rim_light_shadowmap_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bounce_light_res".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bounce_light_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadowmap_samples(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bounce_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bounce_light_shadowmap_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadowmap_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: ObjectThreePointLightXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectThreePointLightRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectThreePointLightPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectThreePointLightUparmtype) -> Self {
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
    pub fn with_key_light_light_type(
        mut self,
        val: ObjectThreePointLightKeyLightLightType,
    ) -> Self {
        self.params.insert(
            "key_light_light_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_key_light_light_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_type(
        mut self,
        val: ObjectThreePointLightKeyLightShadowType,
    ) -> Self {
        self.params.insert(
            "key_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_key_light_shadow_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadowmap_resmenu(
        mut self,
        val: ObjectThreePointLightKeyLightShadowmapResmenu,
    ) -> Self {
        self.params.insert(
            "key_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_key_light_shadowmap_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_type(
        mut self,
        val: ObjectThreePointLightFillLightLightType,
    ) -> Self {
        self.params.insert(
            "fill_light_light_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fill_light_light_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areashape(
        mut self,
        val: ObjectThreePointLightFillLightAreashape,
    ) -> Self {
        self.params.insert(
            "fill_light_areashape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fill_light_areashape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areashape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_type(
        mut self,
        val: ObjectThreePointLightFillLightShadowType,
    ) -> Self {
        self.params.insert(
            "fill_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fill_light_shadow_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadowmap_resmenu(
        mut self,
        val: ObjectThreePointLightFillLightShadowmapResmenu,
    ) -> Self {
        self.params.insert(
            "fill_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fill_light_shadowmap_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_type(
        mut self,
        val: ObjectThreePointLightRimLightLightType,
    ) -> Self {
        self.params.insert(
            "rim_light_light_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rim_light_light_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_type(
        mut self,
        val: ObjectThreePointLightRimLightShadowType,
    ) -> Self {
        self.params.insert(
            "rim_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rim_light_shadow_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadowmap_resmenu(
        mut self,
        val: ObjectThreePointLightRimLightShadowmapResmenu,
    ) -> Self {
        self.params.insert(
            "rim_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rim_light_shadowmap_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_type(
        mut self,
        val: ObjectThreePointLightBounceLightLightType,
    ) -> Self {
        self.params.insert(
            "bounce_light_light_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounce_light_light_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_atten_type(
        mut self,
        val: ObjectThreePointLightBounceLightAttenType,
    ) -> Self {
        self.params.insert(
            "bounce_light_atten_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounce_light_atten_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_atten_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areashape(
        mut self,
        val: ObjectThreePointLightBounceLightAreashape,
    ) -> Self {
        self.params.insert(
            "bounce_light_areashape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounce_light_areashape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areashape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_type(
        mut self,
        val: ObjectThreePointLightBounceLightShadowType,
    ) -> Self {
        self.params.insert(
            "bounce_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounce_light_shadow_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadowmap_resmenu(
        mut self,
        val: ObjectThreePointLightBounceLightShadowmapResmenu,
    ) -> Self {
        self.params.insert(
            "bounce_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounce_light_shadowmap_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadowmap_resMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_lookat_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "lookat_camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookat_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookat_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup_camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_projmap(mut self, val: &str) -> Self {
        self.params.insert(
            "key_light_projmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_key_light_projmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_projmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadowmask(mut self, val: &str) -> Self {
        self.params.insert(
            "key_light_shadowmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_key_light_shadowmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadowmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadowmap_file(mut self, val: &str) -> Self {
        self.params.insert(
            "key_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_key_light_shadowmap_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_projmap(mut self, val: &str) -> Self {
        self.params.insert(
            "fill_light_projmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fill_light_projmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_projmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areamap(mut self, val: &str) -> Self {
        self.params.insert(
            "fill_light_areamap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fill_light_areamap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areamap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadowmask(mut self, val: &str) -> Self {
        self.params.insert(
            "fill_light_shadowmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadowmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadowmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadowmap_file(mut self, val: &str) -> Self {
        self.params.insert(
            "fill_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadowmap_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_projmap(mut self, val: &str) -> Self {
        self.params.insert(
            "rim_light_projmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rim_light_projmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_projmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadowmask(mut self, val: &str) -> Self {
        self.params.insert(
            "rim_light_shadowmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadowmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadowmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadowmap_file(mut self, val: &str) -> Self {
        self.params.insert(
            "rim_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadowmap_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_projmap(mut self, val: &str) -> Self {
        self.params.insert(
            "bounce_light_projmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounce_light_projmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_projmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areamap(mut self, val: &str) -> Self {
        self.params.insert(
            "bounce_light_areamap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areamap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areamap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadowmask(mut self, val: &str) -> Self {
        self.params.insert(
            "bounce_light_shadowmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadowmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadowmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadowmap_file(mut self, val: &str) -> Self {
        self.params.insert(
            "bounce_light_shadowmap_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadowmap_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadowmap_file".to_string(),
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
    pub fn with_azimuth_only(mut self, val: bool) -> Self {
        self.params.insert(
            "azimuth_only".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_azimuth_only_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "azimuth_only".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "light_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_light_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_look_at_target(mut self, val: bool) -> Self {
        self.params.insert(
            "use_look_at_target".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_look_at_target_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_look_at_target".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_light_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_light_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_light_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_show_key_direction(mut self, val: bool) -> Self {
        self.params.insert(
            "show_key_direction".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_key_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_key_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_light_contribdiff(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_light_contribdiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_light_contribspec(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_light_contribspec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_light_conefov(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_light_conefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_clampprojmap(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_clampprojmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_shadow_transparent(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_shadow_transparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_render_shadowmap(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_render_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_light_allowmotionblur(mut self, val: bool) -> Self {
        self.params.insert(
            "key_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_key_light_allowmotionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "key_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_light_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_light_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_show_fill_direction(mut self, val: bool) -> Self {
        self.params.insert(
            "show_fill_direction".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_fill_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_fill_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_contribdiff(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_light_contribdiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_contribspec(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_light_contribspec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_light_conefov(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_light_conefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_clampprojmap(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_clampprojmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areacosine(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_areacosine".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_areacosine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areacosine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areasingle(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_areasingle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_areasingle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areasingle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areafullsphere(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_areafullsphere".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_areafullsphere_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areafullsphere".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_areausemap(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_areausemap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_areausemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_areausemap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_shadow_transparent(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_shadow_transparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_render_shadowmap(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_render_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill_light_allowmotionblur(mut self, val: bool) -> Self {
        self.params.insert(
            "fill_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fill_light_allowmotionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_light_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_light_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_show_rim_direction(mut self, val: bool) -> Self {
        self.params.insert(
            "show_rim_direction".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_rim_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_rim_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_contribdiff(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_light_contribdiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_contribspec(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_light_contribspec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_light_conefov(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_light_conefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_clampprojmap(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_clampprojmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_shadow_transparent(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_shadow_transparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_render_shadowmap(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_render_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rim_light_allowmotionblur(mut self, val: bool) -> Self {
        self.params.insert(
            "rim_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rim_light_allowmotionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rim_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_light_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_light_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_show_bounce_direction(mut self, val: bool) -> Self {
        self.params.insert(
            "show_bounce_direction".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_bounce_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_bounce_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_contribdiff(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_light_contribdiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_contribdiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_contribspec(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_light_contribspec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_contribspec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_light_conefov(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_light_conefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_light_conefov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_clampprojmap(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_clampprojmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_clampprojmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areacosine(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_areacosine".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_areacosine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areacosine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areasingle(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_areasingle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_areasingle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areasingle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areafullsphere(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_areafullsphere".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_areafullsphere_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areafullsphere".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_areausemap(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_areausemap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_areausemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_areausemap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_shadow_transparent(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_shadow_transparent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_shadow_transparent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_render_shadowmap(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_render_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_render_shadowmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce_light_allowmotionblur(mut self, val: bool) -> Self {
        self.params.insert(
            "bounce_light_allowmotionblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounce_light_allowmotionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce_light_allowmotionblur".to_string(),
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
}

impl crate::core::types::HoudiniNode for ObjectThreePointLight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "three_point_light"
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
pub trait ObjectThreePointLightInnerExt {
    fn bounce_direction(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bounce_light(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn camera_position(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn constraints(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn direction_geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn fill_direction(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn fill_light(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn key_direction(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn key_light(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn lighting_target(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rim_direction(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rim_light(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rotate_to_camera(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectThreePointLightInnerExt
    for crate::core::graph::InnerGraph<'a, ObjectThreePointLight>
{
    fn bounce_direction(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bounce_direction")
    }
    fn bounce_light(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bounce_light")
    }
    fn camera_position(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("camera_position")
    }
    fn constraints(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("constraints")
    }
    fn direction_geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("direction_geo")
    }
    fn fill_direction(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("fill_direction")
    }
    fn fill_light(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("fill_light")
    }
    fn key_direction(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("key_direction")
    }
    fn key_light(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("key_light")
    }
    fn lighting_target(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("lighting_target")
    }
    fn rim_direction(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rim_direction")
    }
    fn rim_light(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rim_light")
    }
    fn rotate_to_camera(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rotate_to_camera")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreepointmusclePreXform {
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
pub enum ObjectThreepointmuscleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreepointmuscleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectThreepointmuscleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectThreepointmuscle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectThreepointmuscle {
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

    /// Connects to input 0: "Muscle Anchor Input #1"
    pub fn set_input_muscle_anchor_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Muscle Anchor Input #1" and specifies the output index of the target node.
    pub fn set_input_muscle_anchor_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_muscle_anchor_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Muscle Anchor Input #2"
    pub fn set_input_muscle_anchor_input_2<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Muscle Anchor Input #2" and specifies the output index of the target node.
    pub fn set_input_muscle_anchor_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_muscle_anchor_input_2_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: "Muscle Anchor Input #3"
    pub fn set_input_muscle_anchor_input_3<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "Muscle Anchor Input #3" and specifies the output index of the target node.
    pub fn set_input_muscle_anchor_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_muscle_anchor_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
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
    pub fn with_muscle1_positionbias(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_positionbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_positionbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_positionbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_jiggle_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_jiggle_amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_jiggle_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_jiggle_amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_jiggle_stiff(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_jiggle_stiff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_jiggle_stiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_jiggle_stiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_jiggle_damp(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_jiggle_damp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_jiggle_damp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_jiggle_damp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_jiggle_limit(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_jiggle_limit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_jiggle_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_jiggle_limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_jiggle_flex(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle1_jiggle_flex".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle1_jiggle_flex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_jiggle_flex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_positionbias(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_positionbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_positionbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_positionbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_jiggle_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_jiggle_amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_jiggle_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_jiggle_amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_jiggle_stiff(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_jiggle_stiff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_jiggle_stiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_jiggle_stiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_jiggle_damp(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_jiggle_damp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_jiggle_damp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_jiggle_damp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_jiggle_limit(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_jiggle_limit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_jiggle_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_jiggle_limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_jiggle_flex(mut self, val: f32) -> Self {
        self.params.insert(
            "muscle2_jiggle_flex".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscle2_jiggle_flex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_jiggle_flex".to_string(),
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
    pub fn with_muscle1_musclecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_musclecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_musclecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_musclecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_anchor_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_anchor_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_anchor_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_anchor_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_middle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_anchor_middle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_anchor_middle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_middle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_middle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_anchor_middle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_anchor_middle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_middle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_anchor_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_anchor_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_anchor_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_anchor_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_musclescale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_musclescale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_musclescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_musclescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_jiggle_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle1_jiggle_mult".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle1_jiggle_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_jiggle_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_musclecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_musclecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_musclecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_musclecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_anchor_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_anchor_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_anchor_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_anchor_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_middle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_anchor_middle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_anchor_middle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_middle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_middle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_anchor_middle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_anchor_middle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_middle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_anchor_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_anchor_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_anchor_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_anchor_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_musclescale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_musclescale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_musclescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_musclescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_jiggle_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle2_jiggle_mult".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle2_jiggle_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_jiggle_mult".to_string(),
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
    pub fn with_muscle1_primspersegment(mut self, val: i32) -> Self {
        self.params.insert(
            "muscle1_primspersegment".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_muscle1_primspersegment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_primspersegment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_primspersegment(mut self, val: i32) -> Self {
        self.params.insert(
            "muscle2_primspersegment".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_muscle2_primspersegment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_primspersegment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectThreepointmusclePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectThreepointmuscleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectThreepointmuscleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectThreepointmuscleUparmtype) -> Self {
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
    pub fn with_muscle1_musclename(mut self, val: &str) -> Self {
        self.params.insert(
            "muscle1_musclename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_muscle1_musclename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_musclename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_restanchor(mut self, val: &str) -> Self {
        self.params.insert(
            "muscle1_restanchor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_muscle1_restanchor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_restanchor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_musclename(mut self, val: &str) -> Self {
        self.params.insert(
            "muscle2_musclename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_muscle2_musclename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_musclename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_restanchor(mut self, val: &str) -> Self {
        self.params.insert(
            "muscle2_restanchor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_muscle2_restanchor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_restanchor".to_string(),
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
    pub fn with_muscle_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle_anchor_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle_anchor_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle1_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle1_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle1_anchor_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle1_anchor_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle1_anchor_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle1_anchor_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle2_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle2_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle2_anchor_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle2_anchor_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle2_anchor_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle2_anchor_display".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectThreepointmuscle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "threepointmuscle"
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
pub trait ObjectThreepointmuscleInnerExt {
    fn blend1_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend2_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_jiggle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_jiggle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chopnet(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle1_anchor_end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle1_anchor_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle1_anchor_middle_jiggle_pos(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle1_anchor_root(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle2_anchor_end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle2_anchor_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle2_anchor_middle_jiggle_pos(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle2_anchor_root(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectThreepointmuscleInnerExt
    for crate::core::graph::InnerGraph<'a, ObjectThreepointmuscle>
{
    fn blend1_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend1__middle")
    }
    fn blend2_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend2__middle")
    }
    fn blend_jiggle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend__jiggle1")
    }
    fn blend_jiggle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend__jiggle2")
    }
    fn chopnet(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("chopnet")
    }
    fn muscle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle1")
    }
    fn muscle1_anchor_end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle1_anchor_end")
    }
    fn muscle1_anchor_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle1_anchor_middle")
    }
    fn muscle1_anchor_middle_jiggle_pos(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle1_anchor_middle_jiggle_pos")
    }
    fn muscle1_anchor_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle1_anchor_root")
    }
    fn muscle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle2")
    }
    fn muscle2_anchor_end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle2_anchor_end")
    }
    fn muscle2_anchor_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle2_anchor_middle")
    }
    fn muscle2_anchor_middle_jiggle_pos(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle2_anchor_middle_jiggle_pos")
    }
    fn muscle2_anchor_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle2_anchor_root")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterPreXform {
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
pub enum ObjectToonCharacterXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterToonCharacterDeformRigSkinLod {
    LowRes = 0,
    HighRes = 1,
    /// Low Res Body, High Res Head
    LowResBodyHighResHead = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterToonCharacterDeformRigHeadAndNeckLeftEyeUseCustomEye {
    None = 0,
    FromScene = 1,
    FromFile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterToonCharacterDeformRigHeadAndNeckRightEyeUseCustomEye {
    None = 0,
    FromScene = 1,
    FromFile = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectToonCharacter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectToonCharacter {
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
    pub fn trigger_set_animation_defaults(mut self) -> Self {
        self.params.insert(
            "set_animation_defaults".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_left_arm_match_fk_to_ik(mut self) -> Self {
        self.params.insert(
            "left_arm_match_fk_to_ik".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_left_arm_match_ik_to_fk(mut self) -> Self {
        self.params.insert(
            "left_arm_match_ik_to_fk".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_right_arm_match_fk_to_ik(mut self) -> Self {
        self.params.insert(
            "right_arm_match_fk_to_ik".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_right_arm_match_ik_to_fk(mut self) -> Self {
        self.params.insert(
            "right_arm_match_ik_to_fk".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_left_leg_match_ik_to_fk(mut self) -> Self {
        self.params.insert(
            "left_leg_match_ik_to_fk".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_left_leg_match_fk_to_ik(mut self) -> Self {
        self.params.insert(
            "left_leg_match_fk_to_ik".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_right_leg_match_ik_to_fk(mut self) -> Self {
        self.params.insert(
            "right_leg_match_ik_to_fk".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_right_leg_match_fk_to_ik(mut self) -> Self {
        self.params.insert(
            "right_leg_match_fk_to_ik".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_generate_mocap(mut self) -> Self {
        self.params.insert(
            "generate_mocap".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_read_autorig_data(mut self) -> Self {
        self.params.insert(
            "read_autorig_data".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_save_autorig_data(mut self) -> Self {
        self.params.insert(
            "save_autorig_data".to_string(),
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
    pub fn with_rig_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "rig_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rig_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rig_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_blend_head_space(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_blend__head_space".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_blend_head_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_blend__head_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_fk_ik_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_fk_ik_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_stretch_ratio(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_stretch_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_wrist_world_space_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_wrist_world_space_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_wrist_world_space_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_wrist_world_space_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_curve_upperarm_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_ctrl_curve_upperarm_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_curve_upperarm_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_curve_upperarm_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_blend_custom_control_space(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_blend_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_fk_ik_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_fk_ik_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_stretch_ratio(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_stretch_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_wrist_world_space_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_wrist_world_space_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_wrist_world_space_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_wrist_world_space_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_curve_upperarm_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_ctrl_curve_upperarm_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_curve_upperarm_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_curve_upperarm_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_blend_custom_control_space(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_blend_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_fk_ik_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_fk_ik_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_stretch_ratio(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_stretch_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_curve_thigh_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_ctrl_curve_thigh_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_curve_thigh_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_curve_thigh_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_blend_custom_control_space(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_blend_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_fk_ik_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_fk_ik_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_fk_ik_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_stretch_ratio(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_stretch_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_stretch_ratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_curve_thigh_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_ctrl_curve_thigh_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_curve_thigh_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_curve_thigh_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_blend_custom_control_space(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_blend_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_blend__custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_sad(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_sad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_sad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_sad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_sad(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_sad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_sad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_sad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_angry(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_angry".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_angry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_angry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_angry(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_angry".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_angry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_angry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_outter_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_outter_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_outter_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_outter_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_inner_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_inner_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_inner_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_inner_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_in(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_in(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_top_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "l_top_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_top_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_top_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_top_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "r_top_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_top_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_top_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_bottom_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "l_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_bottom_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_bottom_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "r_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_bottom_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_outer_eye_squint(mut self, val: f32) -> Self {
        self.params.insert(
            "l_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_outer_eye_squint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_outer_eye_squint(mut self, val: f32) -> Self {
        self.params.insert(
            "r_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_outer_eye_squint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_snarl(mut self, val: f32) -> Self {
        self.params.insert(
            "l_snarl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_snarl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_snarl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_snarl(mut self, val: f32) -> Self {
        self.params.insert(
            "r_snarl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_snarl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_snarl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_frown(mut self, val: f32) -> Self {
        self.params.insert(
            "l_frown".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_frown_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_frown".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_frown(mut self, val: f32) -> Self {
        self.params.insert(
            "r_frown".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_frown_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_frown".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_cheek_puff(mut self, val: f32) -> Self {
        self.params.insert(
            "l_cheek_puff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_cheek_puff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_cheek_puff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_cheek_puff(mut self, val: f32) -> Self {
        self.params.insert(
            "r_cheek_puff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_cheek_puff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_cheek_puff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_cheek_suck_in(mut self, val: f32) -> Self {
        self.params.insert(
            "l_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_cheek_suck_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_cheek_suck_in(mut self, val: f32) -> Self {
        self.params.insert(
            "r_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_cheek_suck_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_smile(mut self, val: f32) -> Self {
        self.params.insert(
            "l_smile".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_smile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_smile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_smile(mut self, val: f32) -> Self {
        self.params.insert(
            "r_smile".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_smile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_smile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_top_lip_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_top_lip_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_top_lip_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_top_lip_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_top_lip_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_top_lip_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_top_lip_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_top_lip_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bottom_lip_curl_out(mut self, val: f32) -> Self {
        self.params.insert(
            "bottom_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bottom_lip_curl_out_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottom_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bottom_lip_curl_in(mut self, val: f32) -> Self {
        self.params.insert(
            "bottom_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bottom_lip_curl_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottom_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_top_lip_curl_out(mut self, val: f32) -> Self {
        self.params.insert(
            "top_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_top_lip_curl_out_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "top_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_top_lip_curl_in(mut self, val: f32) -> Self {
        self.params.insert(
            "top_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_top_lip_curl_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "top_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eee(mut self, val: f32) -> Self {
        self.params.insert(
            "eee".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eee_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eee".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oh(mut self, val: f32) -> Self {
        self.params
            .insert("oh".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_oh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ooo(mut self, val: f32) -> Self {
        self.params.insert(
            "ooo".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ooo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ooo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_rx(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_forearm_rx".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_rx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_forearm_rx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_forearm_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_forearm_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_wrist_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_wrist_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_wrist_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_wrist_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_rx(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_forearm_rx".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_rx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_forearm_rx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_forearm_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_forearm_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_wrist_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_wrist_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_wrist_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_wrist_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_blend_ik_leg_twist_space_blendw1(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_blend_ik_leg_twist_space_blendw1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_blend_ik_leg_twist_space_blendw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_blend_ik_leg_twist_space_blendw1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_end_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_end_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_end_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_end_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_blend_ik_leg_twist_space_blendw1(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_blend_ik_leg_twist_space_blendw1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_blend_ik_leg_twist_space_blendw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_blend_ik_leg_twist_space_blendw1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_end_translate_tz(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_end_translate_tz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_end_translate_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_end_translate_tz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_character_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "character_placer_character_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_character_placer_character_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_character_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refit_tolerance(mut self, val: f32) -> Self {
        self.params.insert(
            "refit_tolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refit_tolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refit_tolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "character_placer_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_character_placer_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_control_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_control_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_control_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_control_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_control_scale".to_string(),
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
    pub fn with_spine_ctrl_cog_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_cog_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_cog_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_pivot_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_cog_pivot_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_pivot_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_pivot_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_pivot_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_cog_pivot_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_pivot_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_pivot_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_gimbal_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_cog_gimbal_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_gimbal_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_gimbal_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_hip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_hip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_hip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_hip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back3_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back3_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back3_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back3_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back3_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back3_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back3_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back3_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_chest1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_chest1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_chest1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_chest1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_chest1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_chest1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_chest1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_chest1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_chest2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_chest2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_chest2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_chest2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_chest2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_ctrl_chest2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_ctrl_chest2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_chest2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_head_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_head_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_head_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_head_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_head_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_head_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_head_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_head_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_gimbal_head_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_gimbal_head_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_gimbal_head_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_gimbal_head_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_jaw_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_jaw_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_jaw_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_jaw_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_jaw_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_jaw_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_jaw_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_jaw_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_eyes_look_at_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_eyes_look_at_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_eyes_look_at_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_eyes_look_at_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_eyes_look_at_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_eyes_look_at_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_eyes_look_at_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_eyes_look_at_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_look_at_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_look_at_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_look_at_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_look_at_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_look_at_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_look_at_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_look_at_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_look_at_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_collarbone_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_collarbone_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_wrist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_wrist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_twist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_twist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_twist_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_IK_twist_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_fk_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_fk_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_local_space_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_local_space_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_local_space_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_local_space_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_world_space_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_world_space_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_wrist_world_space_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_wrist_world_space_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_collarbone_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_collarbone_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_collarbone_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_wrist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_wrist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_wrist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_wrist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_twist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_twist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_twist_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_IK_twist_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_fk_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_fk_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_FK_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_local_space_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_local_space_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_local_space_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_local_space_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_world_space_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_world_space_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_wrist_world_space_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_wrist_world_space_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_wrist_world_space_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumb2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumb2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_index1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_index1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_index1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_index1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_index1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_index1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_index1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_index1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_index2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_index2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_index2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_index2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_index2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_index2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_index2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_index2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinky2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinky2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumb2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumb2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_index1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_index1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_index1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_index1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_index1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_index1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_index1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_index1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_index2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_index2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_index2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_index2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_index2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_index2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_index2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_index2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinky2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinky2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_pelvis_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_pelvis_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_pelvis_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_pelvis_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ankle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ankle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ankle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ankle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_rotate_from_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_rotate_from_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_rotate_from_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_rotate_from_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_rotate_from_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_rotate_from_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_rotate_from_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_rotate_from_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ball_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ball_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ball_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ball_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ball_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_leg_twist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_leg_twist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_leg_twist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_leg_twist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_rotate_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_rotate_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_rotate_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_rotate_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_rotate_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_rotate_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_rotate_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_rotate_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_rotate_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_rotate_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_rotate_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_rotate_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_pelvis_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_pelvis_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_pelvis_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_pelvis_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ankle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ankle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ankle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ankle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_rotate_from_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_rotate_from_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_rotate_from_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_rotate_from_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_rotate_from_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_rotate_from_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_rotate_from_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_rotate_from_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ball_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ball_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ball_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ball_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ball_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_leg_twist_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_leg_twist_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_leg_twist_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_leg_twist_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_rotate_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_rotate_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_rotate_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_rotate_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_rotate_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_rotate_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_rotate_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_rotate_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_rotate_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_rotate_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_rotate_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_rotate_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_character_placer_translate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "character_placer_character_placer_translate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_character_placer_character_placer_translate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_character_placer_translate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_character_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "character_placer_character_rotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_character_placer_character_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_character_rotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "character_placer_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_character_placer_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_control_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_control_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_control_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_control_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_control_color".to_string(),
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
    pub fn with_toon_character_deform_rig_skin_lod(
        mut self,
        val: ObjectToonCharacterToonCharacterDeformRigSkinLod,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_skin_lod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_toon_character_deform_rig_skin_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_character_deform_rig_skin_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "control_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_control_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_character_placer_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "character_placer_character_placer_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_character_placer_character_placer_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_character_placer_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_cog_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_pivot_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_cog_pivot_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_pivot_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_pivot_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_cog_gimbal_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_cog_gimbal_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_cog_gimbal_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_cog_gimbal_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_hip_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_hip_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_hip_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_hip_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back1_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back1_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back1_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back1_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back2_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back2_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back2_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back2_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back3_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back3_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_lower_back3_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_lower_back3_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_chest1_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_chest1_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_chest1_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_chest1_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_chest2_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_chest2_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_chest2_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_chest2_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_ctrl_neck_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "spine_ctrl_neck_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spine_ctrl_neck_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_ctrl_neck_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_root_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_root_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_root_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_root_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_neck_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_neck_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_head_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_head_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_head_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_head_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_gimbal_head_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_gimbal_head_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_gimbal_head_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_gimbal_head_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_jaw_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_jaw_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_jaw_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_jaw_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_eyes_look_at_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_eyes_look_at_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_eyes_look_at_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_eyes_look_at_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_look_at_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_look_at_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_look_at_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_look_at_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_look_at_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_look_at_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_look_at_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_look_at_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_left_eye_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_left_eye_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_and_neck_ctrl_right_eye_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_ctrl_right_eye_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_collarbone_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_collarbone_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_collarbone_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_collarbone_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_upperarm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_upperarm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_fk_upperarm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_fk_upperarm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_forearm_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_forearm_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_forearm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_forearm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_forearm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_wrist_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_wrist_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_fk_wrist_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_fk_wrist_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_twist_local_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_twist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_twist_local_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_twist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_local_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_local_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_world_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_wrist_world_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_wrist_world_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_gimbal_wrist_world_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_gimbal_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_wrist_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_wrist_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_wrist_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_wrist_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_shoulderblade_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_shoulderblade_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_shoulderblade_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_twist_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_shoulderblade_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_twist_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_twist_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_ik_twist_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_ik_twist_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_ctrl_curve_upperarm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_arm_ctrl_curve_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_arm_ctrl_curve_upperarm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_ctrl_curve_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_collarbone_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_collarbone_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_collarbone_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_collarbone_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_upperarm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_upperarm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_fk_upperarm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_fk_upperarm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_fk_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_forearm_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_forearm_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_forearm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_forearm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_forearm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_wrist_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_wrist_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_fk_wrist_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_fk_wrist_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_twist_local_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_twist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_twist_local_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_twist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_local_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_local_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_local_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_world_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_wrist_world_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_wrist_world_space_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_gimbal_wrist_world_space_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_gimbal_wrist_world_space_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_wrist_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_wrist_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_wrist_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_wrist_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_shoulderblade_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_shoulderblade_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_shoulderblade_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_twist_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_shoulderblade_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_twist_shoulderblade_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_twist_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_ik_twist_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_ik_twist_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_ctrl_curve_upperarm_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_arm_ctrl_curve_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_arm_ctrl_curve_upperarm_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_ctrl_curve_upperarm_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumbs_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumbs_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_thumbs_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_thumbs_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_indexes_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_hand_ctrl_indexes_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_indexes_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_indexes_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_middles_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_hand_ctrl_middles_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_middles_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_middles_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinkies_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinkies_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_hand_ctrl_pinkies_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_ctrl_pinkies_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumbs_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumbs_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_thumbs_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_thumbs_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_indexes_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_hand_ctrl_indexes_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_indexes_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_indexes_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_middles_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_hand_ctrl_middles_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_middles_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_middles_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinkies_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinkies_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_hand_ctrl_pinkies_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_ctrl_pinkies_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_pelvis_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_pelvis_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_pelvis_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_pelvis_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_foot_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_foot_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_foot_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_foot_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_rotate_from_toe_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_rotate_from_toe_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_rotate_from_toe_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_rotate_from_toe_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ball_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ball_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_ball_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_ball_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_toe_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_toe_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_toe_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_toe_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_leg_twist_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_leg_twist_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_ik_leg_twist_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_ik_leg_twist_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_thigh_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_thigh_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_thigh_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_thigh_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_rotate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_rotate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_shin_rotate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_shin_rotate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_rotate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_rotate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_ankle_rotate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_ankle_rotate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_rotate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_rotate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_rotate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_rotate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_end_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_end_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_fk_toe_end_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_fk_toe_end_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_ctrl_curve_thigh_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "left_leg_ctrl_curve_thigh_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_leg_ctrl_curve_thigh_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_ctrl_curve_thigh_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_pelvis_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_pelvis_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_pelvis_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_pelvis_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_foot_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_foot_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_foot_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_foot_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_rotate_from_toe_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_rotate_from_toe_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_rotate_from_toe_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_rotate_from_toe_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ball_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ball_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_ball_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_ball_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_toe_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_toe_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_toe_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_toe_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_leg_twist_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_leg_twist_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_ik_leg_twist_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_ik_leg_twist_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_thigh_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_thigh_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_thigh_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_thigh_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_rotate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_rotate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_shin_rotate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_shin_rotate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_rotate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_rotate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_ankle_rotate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_ankle_rotate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_rotate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_rotate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_rotate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_rotate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_end_translate_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_end_translate_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_fk_toe_end_translate_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_fk_toe_end_translate_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_ctrl_curve_thigh_lod(mut self, val: i32) -> Self {
        self.params.insert(
            "right_leg_ctrl_curve_thigh_lod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_leg_ctrl_curve_thigh_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_ctrl_curve_thigh_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectToonCharacterPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectToonCharacterXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectToonCharacterRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectToonCharacterUparmtype) -> Self {
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
    pub fn with_toon_character_deform_rig_head_and_neck_left_eye_use_custom_eye(
        mut self,
        val: ObjectToonCharacterToonCharacterDeformRigHeadAndNeckLeftEyeUseCustomEye,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_left_eye_use_custom_eye".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_left_eye_use_custom_eye_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_left_eye_use_custom_eye".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_right_eye_use_custom_eye(
        mut self,
        val: ObjectToonCharacterToonCharacterDeformRigHeadAndNeckRightEyeUseCustomEye,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_right_eye_use_custom_eye".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_right_eye_use_custom_eye_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_right_eye_use_custom_eye".to_string(),
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
    pub fn with_left_arm_custom_control_space(mut self, val: &str) -> Self {
        self.params.insert(
            "left_arm_custom_control_space".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_left_arm_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_custom_control_space(mut self, val: &str) -> Self {
        self.params.insert(
            "right_arm_custom_control_space".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_right_arm_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_custom_control_space(mut self, val: &str) -> Self {
        self.params.insert(
            "left_leg_custom_control_space".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_left_leg_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_custom_control_space(mut self, val: &str) -> Self {
        self.params.insert(
            "right_leg_custom_control_space".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_right_leg_custom_control_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_custom_control_space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_hook_object(mut self, val: &str) -> Self {
        self.params.insert(
            "character_placer_hook_object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_character_placer_hook_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_hook_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_skin_material(mut self, val: &str) -> Self {
        self.params.insert(
            "toon_character_deform_rig_skin_material".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_skin_material_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_character_deform_rig_skin_material".to_string(),
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
    pub fn with_skin_properties(mut self, val: &str) -> Self {
        self.params.insert(
            "skin_properties".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skin_properties_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skin_properties".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_left_eye_merge_custom_eye_objpath1(
        mut self,
        val: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_left_eye_merge__custom_eye_objpath1"
                .to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_left_eye_merge_custom_eye_objpath1_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_left_eye_merge__custom_eye_objpath1"
                .to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_left_eye_file_eye_from_file_file(
        mut self,
        val: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_left_eye_file__eye_from_file_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_left_eye_file_eye_from_file_file_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_left_eye_file__eye_from_file_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_right_eye_merge_custom_eye_objpath1(
        mut self,
        val: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_right_eye_merge__custom_eye_objpath1"
                .to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_right_eye_merge_custom_eye_objpath1_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_right_eye_merge__custom_eye_objpath1"
                .to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_right_eye_file_eye_from_file_file(
        mut self,
        val: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_right_eye_file__eye_from_file_file"
                .to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toon_character_deform_rig_head_and_neck_right_eye_file_eye_from_file_file_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "toon_character_deform_rig_head_and_neck_right_eye_file__eye_from_file_file"
                .to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_file_path(mut self, val: &str) -> Self {
        self.params.insert(
            "source_file_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_file_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_file_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_save_file_path(mut self, val: &str) -> Self {
        self.params.insert(
            "save_file_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_save_file_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "save_file_path".to_string(),
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
    pub fn with_toon_character_deform_rig_display_skin(mut self, val: bool) -> Self {
        self.params.insert(
            "toon_character_deform_rig_display_skin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_toon_character_deform_rig_display_skin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_character_deform_rig_display_skin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_bones(mut self, val: bool) -> Self {
        self.params.insert(
            "display_bones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_bones_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_bones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "character_placer_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_character_placer_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "spine_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spine_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "head_and_neck_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_and_neck_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "left_arm_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_arm_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "right_arm_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_arm_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "left_hand_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_hand_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "right_hand_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_hand_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "left_leg_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_leg_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_display_bodypart(mut self, val: bool) -> Self {
        self.params.insert(
            "right_leg_display_bodypart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_leg_display_bodypart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_display_bodypart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_character_placer_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "character_placer_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_character_placer_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "character_placer_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "spine_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spine_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "head_and_neck_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_and_neck_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "left_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_arm_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "right_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_arm_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "left_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_hand_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "right_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_hand_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "left_leg_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_display_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_display_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "right_leg_display_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_leg_display_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_display_controls".to_string(),
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
}

impl crate::core::types::HoudiniNode for ObjectToonCharacter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "toon_character"
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
pub trait ObjectToonCharacterInnerExt {
    fn character_placer(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn head_and_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_arm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_hand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_leg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_arm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_hand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_leg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn source_geometry(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn toon_character_deform_rig(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectToonCharacterInnerExt for crate::core::graph::InnerGraph<'a, ObjectToonCharacter> {
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
    fn source_geometry(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("source_geometry")
    }
    fn spine(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("spine")
    }
    fn toon_character_deform_rig(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("toon_character_deform_rig")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterDeformRigPreXform {
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
pub enum ObjectToonCharacterDeformRigXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterDeformRigRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterDeformRigUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterDeformRigSkinLod {
    LowRes = 0,
    HighRes = 1,
    /// Low Res Body, High Res Head
    LowResBodyHighResHead = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterDeformRigHeadAndNeckLeftEyeUseCustomEye {
    None = 0,
    FromScene = 1,
    FromFile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectToonCharacterDeformRigHeadAndNeckRightEyeUseCustomEye {
    None = 0,
    FromScene = 1,
    FromFile = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectToonCharacterDeformRig {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectToonCharacterDeformRig {
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
    pub fn with_l_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_sad(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_sad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_sad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_sad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_sad(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_sad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_sad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_sad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_angry(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_angry".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_angry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_angry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_angry(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_angry".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_angry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_angry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_outter_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_outter_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_outter_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_outter_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_outter_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_inner_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_inner_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_inner_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_inner_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_inner_brow_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_brow_in(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_brow_in(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_top_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "l_top_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_top_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_top_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_top_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "r_top_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_top_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_top_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_bottom_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "l_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_bottom_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_bottom_eyelid(mut self, val: f32) -> Self {
        self.params.insert(
            "r_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_bottom_eyelid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_bottom_eyelid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_outer_eye_squint(mut self, val: f32) -> Self {
        self.params.insert(
            "l_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_outer_eye_squint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_outer_eye_squint(mut self, val: f32) -> Self {
        self.params.insert(
            "r_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_outer_eye_squint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_outer_eye_squint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_snarl(mut self, val: f32) -> Self {
        self.params.insert(
            "l_snarl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_snarl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_snarl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_snarl(mut self, val: f32) -> Self {
        self.params.insert(
            "r_snarl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_snarl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_snarl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_frown(mut self, val: f32) -> Self {
        self.params.insert(
            "l_frown".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_frown_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_frown".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_frown(mut self, val: f32) -> Self {
        self.params.insert(
            "r_frown".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_frown_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_frown".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_cheek_puff(mut self, val: f32) -> Self {
        self.params.insert(
            "l_cheek_puff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_cheek_puff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_cheek_puff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_cheek_puff(mut self, val: f32) -> Self {
        self.params.insert(
            "r_cheek_puff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_cheek_puff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_cheek_puff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_cheek_suck_in(mut self, val: f32) -> Self {
        self.params.insert(
            "l_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_cheek_suck_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_cheek_suck_in(mut self, val: f32) -> Self {
        self.params.insert(
            "r_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_cheek_suck_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_cheek_suck_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_smile(mut self, val: f32) -> Self {
        self.params.insert(
            "l_smile".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_smile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_smile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_smile(mut self, val: f32) -> Self {
        self.params.insert(
            "r_smile".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_smile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_smile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_l_top_lip_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_top_lip_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_top_lip_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_top_lip_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_top_lip_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_top_lip_up".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_top_lip_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_top_lip_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bottom_lip_curl_out(mut self, val: f32) -> Self {
        self.params.insert(
            "bottom_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bottom_lip_curl_out_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottom_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bottom_lip_curl_in(mut self, val: f32) -> Self {
        self.params.insert(
            "bottom_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bottom_lip_curl_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottom_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_top_lip_curl_out(mut self, val: f32) -> Self {
        self.params.insert(
            "top_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_top_lip_curl_out_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "top_lip_curl_out".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_top_lip_curl_in(mut self, val: f32) -> Self {
        self.params.insert(
            "top_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_top_lip_curl_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "top_lip_curl_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eee(mut self, val: f32) -> Self {
        self.params.insert(
            "eee".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eee_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eee".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oh(mut self, val: f32) -> Self {
        self.params
            .insert("oh".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_oh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ooo(mut self, val: f32) -> Self {
        self.params.insert(
            "ooo".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ooo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ooo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_muscle_chest_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_muscle_chest_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "spine_muscle_chest_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control4position".to_string(),
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
    pub fn with_spine_muscle_anchor_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_neck_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_neck_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_neck_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_neck_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_neck_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_neck_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_chest1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_chest1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back3_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back3_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back3_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back3_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back3_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back3_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back3_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back3_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_anchor_lower_back1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_anchor_lower_back1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_chest_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_chest_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_chest_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_chest_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_muscle_chest_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spine_muscle_chest_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spine_muscle_chest_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_muscle_chest_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_neck_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_neck_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_neck_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_neck_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_neck_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_neck_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_neck_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_neck_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_neck_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_neck_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_neck_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_neck_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_neck_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_neck_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_neck_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_neck_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_head_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_head_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_head_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_head_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_head_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_head_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_anchor_blended_head_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_anchor_blended_head_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_and_neck_muscle_neck_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_muscle_neck_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_collarbone_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_collarbone_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_collarbone_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_collarbone_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_collarbone_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_collarbone_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_collarbone_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_collarbone_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulder_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulder_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulder_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulder_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulder_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulder_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_upperarm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_upperarm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_upperarm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_upperarm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_upperarm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_upperarm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_upperarm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_upperarm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_forearm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_forearm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_forearm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_forearm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_forearm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_forearm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_elbow_forearm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_elbow_forearm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_curved_forearm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_curved_forearm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_curved_forearm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_curved_forearm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_curved_forearm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_curved_forearm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_curved_forearm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_curved_forearm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_forearm_middle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_forearm_middle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_forearm_middle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_forearm_middle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_forearm_middle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_forearm_middle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_forearm_middle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_forearm_middle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_tip_forearm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_tip_forearm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_tip_forearm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_tip_forearm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_tip_forearm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_tip_forearm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_tip_forearm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_tip_forearm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_anchor_shoulderblade_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_anchor_shoulderblade_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_upperarm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_upperarm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_forearm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_forearm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_arm_muscle_shoulderblade_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_muscle_shoulderblade_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_collarbone_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_collarbone_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_collarbone_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_collarbone_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_collarbone_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_collarbone_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_collarbone_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_collarbone_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulder_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulder_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulder_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulder_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulder_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulder_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulder_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulder_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_upperarm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_upperarm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_upperarm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_upperarm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_upperarm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_upperarm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_upperarm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_upperarm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_upperarm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_upperarm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_upperarm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_upperarm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_forearm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_forearm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_forearm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_forearm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_forearm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_forearm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_elbow_forearm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_elbow_forearm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_curved_forearm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_curved_forearm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_curved_forearm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_curved_forearm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_curved_forearm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_curved_forearm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_curved_forearm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_curved_forearm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_forearm_middle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_forearm_middle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_forearm_middle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_forearm_middle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_forearm_middle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_forearm_middle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_forearm_middle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_forearm_middle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_tip_forearm_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_tip_forearm_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_tip_forearm_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_tip_forearm_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_tip_forearm_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_tip_forearm_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_tip_forearm_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_tip_forearm_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_anchor_shoulderblade_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_anchor_shoulderblade_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_upperarm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_upperarm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_forearm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_forearm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_arm_muscle_shoulderblade_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_muscle_shoulderblade_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_thumb2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_thumb2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_thumb_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_thumb_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_thumb3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_thumb3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_index2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_index2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_index_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_index_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_index3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_index3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_middle_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_middle_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_middle3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_middle3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_blended_pinky2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_blended_pinky2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_anchor_pinky_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_anchor_pinky_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_hand_muscle_pinky3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_muscle_pinky3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_thumb2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_thumb2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_thumb_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_thumb_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_thumb3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_thumb3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_index2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_index2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_index_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_index_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_index3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_index3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_middle_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_middle_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_middle3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_middle3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_root_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_root_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky1_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky1_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_blended_pinky2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_blended_pinky2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky2_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky2_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky2_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky2_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky2_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky2_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky2_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky2_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_anchor_pinky_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_anchor_pinky_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky_palm_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky_palm_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky1_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky1_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky2_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky2_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_hand_muscle_pinky3_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_muscle_pinky3_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_pelvis_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_pelvis_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_pelvis_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_pelvis_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_pelvis_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_pelvis_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_pelvis_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_pelvis_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_tip_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_tip_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_tip_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_tip_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_tip_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_tip_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_tip_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_tip_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_shin_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_shin_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_shin_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_shin_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_shin_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_shin_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_knee_shin_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_knee_shin_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_shin_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_shin_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_ankle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_ankle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_ankle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_ankle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ankle_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ankle_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_blended_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_blended_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ball_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ball_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ball_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ball_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_ball_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_anchor_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_anchor_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_thigh_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_thigh_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_calf_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_calf_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_ankle_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_ankle_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_left_leg_muscle_toe_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_muscle_toe_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_pelvis_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_pelvis_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_pelvis_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_pelvis_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_pelvis_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_pelvis_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_pelvis_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_pelvis_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_tip_thigh_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_tip_thigh_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_tip_thigh_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_tip_thigh_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_tip_thigh_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_tip_thigh_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_tip_thigh_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_tip_thigh_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_shin_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_shin_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_shin_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_shin_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_shin_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_shin_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_knee_shin_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_knee_shin_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_shin_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_shin_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_ankle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_ankle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_ankle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_ankle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_tip_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_tip_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_tip_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_tip_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_tip_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_tip_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ankle_tip_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ankle_tip_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_blended_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_blended_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ball_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ball_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ball_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ball_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ball_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ball_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_ball_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_ball_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_toe_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_toe_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_toe_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_toe_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_toe_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_toe_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_anchor_toe_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_anchor_toe_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_thigh_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_thigh_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_calf_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_calf_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_ankle_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_ankle_control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_right_leg_muscle_toe_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_muscle_toe_control5scale".to_string(),
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
    pub fn with_skin_lod(mut self, val: ObjectToonCharacterDeformRigSkinLod) -> Self {
        self.params.insert(
            "skin_lod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_skin_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skin_lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectToonCharacterDeformRigPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectToonCharacterDeformRigXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectToonCharacterDeformRigRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectToonCharacterDeformRigUparmtype) -> Self {
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
    pub fn with_head_and_neck_left_eye_use_custom_eye(
        mut self,
        val: ObjectToonCharacterDeformRigHeadAndNeckLeftEyeUseCustomEye,
    ) -> Self {
        self.params.insert(
            "head_and_neck_left_eye_use_custom_eye".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_head_and_neck_left_eye_use_custom_eye_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_left_eye_use_custom_eye".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_right_eye_use_custom_eye(
        mut self,
        val: ObjectToonCharacterDeformRigHeadAndNeckRightEyeUseCustomEye,
    ) -> Self {
        self.params.insert(
            "head_and_neck_right_eye_use_custom_eye".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_head_and_neck_right_eye_use_custom_eye_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_right_eye_use_custom_eye".to_string(),
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
    pub fn with_rig_path(mut self, val: &str) -> Self {
        self.params.insert(
            "rig_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rig_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rig_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skin_material(mut self, val: &str) -> Self {
        self.params.insert(
            "skin_material".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skin_material_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skin_material".to_string(),
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
    pub fn with_skin_properties(mut self, val: &str) -> Self {
        self.params.insert(
            "skin_properties".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skin_properties_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skin_properties".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_left_eye_merge_custom_eye_objpath1(mut self, val: &str) -> Self {
        self.params.insert(
            "head_and_neck_left_eye_merge__custom_eye_objpath1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_left_eye_merge_custom_eye_objpath1_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "head_and_neck_left_eye_merge__custom_eye_objpath1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_left_eye_file_eye_from_file_file(mut self, val: &str) -> Self {
        self.params.insert(
            "head_and_neck_left_eye_file__eye_from_file_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_left_eye_file_eye_from_file_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_left_eye_file__eye_from_file_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_right_eye_merge_custom_eye_objpath1(mut self, val: &str) -> Self {
        self.params.insert(
            "head_and_neck_right_eye_merge__custom_eye_objpath1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_right_eye_merge_custom_eye_objpath1_expr(
        mut self,
        expr: &str,
    ) -> Self {
        self.params.insert(
            "head_and_neck_right_eye_merge__custom_eye_objpath1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_right_eye_file_eye_from_file_file(mut self, val: &str) -> Self {
        self.params.insert(
            "head_and_neck_right_eye_file__eye_from_file_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_right_eye_file_eye_from_file_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_right_eye_file__eye_from_file_file".to_string(),
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
    pub fn with_display_skin(mut self, val: bool) -> Self {
        self.params.insert(
            "display_skin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_skin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_skin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_eyes(mut self, val: bool) -> Self {
        self.params.insert(
            "display_eyes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_eyes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_eyes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_global_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "global_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_global_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "global_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_muscle_controls(mut self, val: bool) -> Self {
        self.params.insert(
            "display_muscle_controls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_muscle_controls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_muscle_controls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display_guides(mut self, val: bool) -> Self {
        self.params.insert(
            "display_guides".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_guides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_guides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spine_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "spine_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spine_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spine_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_and_neck_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "head_and_neck_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_and_neck_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_and_neck_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_arm_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "left_arm_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_arm_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_arm_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_arm_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "right_arm_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_arm_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_arm_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_hand_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "left_hand_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_hand_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_hand_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_hand_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "right_hand_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_hand_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_hand_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_leg_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "left_leg_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_left_leg_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left_leg_display_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_leg_display_muscles(mut self, val: bool) -> Self {
        self.params.insert(
            "right_leg_display_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_right_leg_display_muscles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right_leg_display_muscles".to_string(),
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
}

impl crate::core::types::HoudiniNode for ObjectToonCharacterDeformRig {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "toon_character_deform_rig"
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
pub trait ObjectToonCharacterDeformRigInnerExt {
    fn head_and_neck(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_arm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_hand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn left_leg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_arm(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_hand(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn right_leg(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn skin(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn spine(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectToonCharacterDeformRigInnerExt
    for crate::core::graph::InnerGraph<'a, ObjectToonCharacterDeformRig>
{
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
    fn skin(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("skin")
    }
    fn spine(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("spine")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTopnetCheckpointformat {
    /// Python (Deprecated)
    PythonDeprecated = 0,
    Json = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTopnetCheckpointload {
    Never = 0,
    OnSceneLoad = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTopnetRegenerationtype {
    UpdateWorkItemsAndInvalidateCaches = 0,
    UpdateWorkItemsOnly = 1,
    IgnoreChanges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTopnetEvaluationtime {
    NetworkCookTime = 0,
    GlobalStartTime = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct ObjectTopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectTopnet {
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
    pub fn trigger_generatestatic(mut self) -> Self {
        self.params.insert(
            "generatestatic".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookbutton(mut self) -> Self {
        self.params.insert(
            "cookbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtybutton(mut self) -> Self {
        self.params.insert(
            "dirtybutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cancelbutton(mut self) -> Self {
        self.params.insert(
            "cancelbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savetaskgraph(mut self) -> Self {
        self.params.insert(
            "savetaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadtaskgraph(mut self) -> Self {
        self.params.insert(
            "loadtaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadcheckpoint(mut self) -> Self {
        self.params.insert(
            "loadcheckpoint".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_taskgraphsaverate(mut self, val: i32) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskgraphsaverate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointrate(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customtime(mut self, val: i32) -> Self {
        self.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_checkpointformat(mut self, val: ObjectTopnetCheckpointformat) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointload(mut self, val: ObjectTopnetCheckpointload) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regenerationtype(mut self, val: ObjectTopnetRegenerationtype) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_regenerationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_evaluationtime(mut self, val: ObjectTopnetEvaluationtime) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_evaluationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_taskgraphfile(mut self, val: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_taskgraphfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointfile(mut self, val: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpointfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultlabel(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_taskgraphautosave(mut self, val: bool) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_taskgraphautosave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savegraphattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savegraphattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedefaultlabel(mut self, val: bool) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedefaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savescenefile(mut self, val: bool) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savescenefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ObjectTopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "topnet"
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
pub trait ObjectTopnetInnerExt {
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectTopnetInnerExt for crate::core::graph::InnerGraph<'a, ObjectTopnet> {
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("localscheduler")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTwopointmusclePreXform {
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
pub enum ObjectTwopointmuscleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTwopointmuscleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTwopointmuscleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectTwopointmuscle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ObjectTwopointmuscle {
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

    /// Connects to input 0: "Muscle Anchor Input #1"
    pub fn set_input_muscle_anchor_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Muscle Anchor Input #1" and specifies the output index of the target node.
    pub fn set_input_muscle_anchor_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_muscle_anchor_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Muscle Anchor Input #2"
    pub fn set_input_muscle_anchor_input_2<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Muscle Anchor Input #2" and specifies the output index of the target node.
    pub fn set_input_muscle_anchor_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_muscle_anchor_input_2_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_positionbias(mut self, val: f32) -> Self {
        self.params.insert(
            "positionbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control2position(mut self, val: f32) -> Self {
        self.params.insert(
            "control2position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_control2position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control2position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control3position(mut self, val: f32) -> Self {
        self.params.insert(
            "control3position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_control3position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control3position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control4position(mut self, val: f32) -> Self {
        self.params.insert(
            "control4position".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_control4position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control4position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggle_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggle_amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggle_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggle_amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggle1_stiff(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggle1_stiff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggle1_stiff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggle1_stiff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggle1_damp(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggle1_damp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggle1_damp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggle1_damp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggle1_limit(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggle1_limit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggle1_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggle1_limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggle1_flex(mut self, val: f32) -> Self {
        self.params.insert(
            "jiggle1_flex".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jiggle1_flex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggle1_flex".to_string(),
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
    pub fn with_musclecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "musclecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_musclecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_root_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle_anchor_root_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle_anchor_root_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_root_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_root_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle_anchor_root_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle_anchor_root_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_root_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_middle_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle_anchor_middle_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle_anchor_middle_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_middle_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_middle_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle_anchor_middle_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle_anchor_middle_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_middle_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_end_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle_anchor_end_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle_anchor_end_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_end_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscle_anchor_end_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "muscle_anchor_end_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscle_anchor_end_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_anchor_end_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclescale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "musclescale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_musclescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control1scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "control1scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_control1scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control1scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control2scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "control2scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_control2scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control2scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control3scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "control3scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_control3scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control3scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control4scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "control4scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_control4scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control4scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_control5scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "control5scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_control5scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "control5scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jiggle1_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "jiggle1_mult".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_jiggle1_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jiggle1_mult".to_string(),
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
    pub fn with_primspersegment(mut self, val: i32) -> Self {
        self.params.insert(
            "primspersegment".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primspersegment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primspersegment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pre_xform(mut self, val: ObjectTwopointmusclePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectTwopointmuscleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectTwopointmuscleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectTwopointmuscleUparmtype) -> Self {
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
    pub fn with_musclename(mut self, val: &str) -> Self {
        self.params.insert(
            "musclename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_musclename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "musclename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restanchor(mut self, val: &str) -> Self {
        self.params.insert(
            "restanchor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restanchor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restanchor".to_string(),
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
    pub fn with_muscle_display(mut self, val: bool) -> Self {
        self.params.insert(
            "muscle_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_muscle_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "muscle_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anchor_display(mut self, val: bool) -> Self {
        self.params.insert(
            "anchor_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_anchor_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anchor_display".to_string(),
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

impl crate::core::types::HoudiniNode for ObjectTwopointmuscle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "twopointmuscle"
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
pub trait ObjectTwopointmuscleInnerExt {
    fn blend_jiggle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chopnet(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle_anchor_end(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle_anchor_middle(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle_anchor_middle_jiggle_pos(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn muscle_anchor_root(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectTwopointmuscleInnerExt for crate::core::graph::InnerGraph<'a, ObjectTwopointmuscle> {
    fn blend_jiggle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend__jiggle")
    }
    fn blend_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blend__middle")
    }
    fn chopnet(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("chopnet")
    }
    fn muscle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle")
    }
    fn muscle_anchor_end(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle_anchor_end")
    }
    fn muscle_anchor_middle(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle_anchor_middle")
    }
    fn muscle_anchor_middle_jiggle_pos(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle_anchor_middle_jiggle_pos")
    }
    fn muscle_anchor_root(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("muscle_anchor_root")
    }
}
