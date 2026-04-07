#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewateremitterShape {
    Box = 0,
    Sphere = 1,
    Cylinder = 2,
    Cone = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewateremitterInitvel {
    UseInheritedVelocity = 0,
    AddToInheritedVelocity = 1,
    SetInitialVelocity = 2,
}

#[derive(Debug, Clone)]
pub struct DopWhitewateremitter {
    pub base: crate::core::types::NodeBase,
}

impl DopWhitewateremitter {
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
    pub fn with_impulserate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "impulserate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulserate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impulserate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constantrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "constantrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constantrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constantrate".to_string(),
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
    pub fn with_life(mut self, val: f32) -> Self {
        self.base.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_life_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lifevar(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lifevar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifevar_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lifevar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "velscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veloffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "veloffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veloffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "veloffset".to_string(),
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
    pub fn with_amp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten(mut self, val: f32) -> Self {
        self.base.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inheritvel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "inheritvel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inheritvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inheritvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radialvel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radialvel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radialvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radialvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_freq(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_var(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_turb(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("turb".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_turb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "turb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shape(mut self, val: DopWhitewateremitterShape) -> Self {
        self.base.params.insert(
            "shape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shape_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initvel(mut self, val: DopWhitewateremitterInitvel) -> Self {
        self.base.params.insert(
            "initvel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source(mut self, val: &str) -> Self {
        self.base.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attrib".to_string(),
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
    pub fn with_type(mut self, val: &str) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_streamname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_streamname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_donoise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "donoise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donoise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "donoise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWhitewateremitter {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "whitewateremitter"
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
pub enum DopWhitewaterobjectGuidevismode {
    Ramp = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    Blackbody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewaterobjectSurfaceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewaterobjectSurfaceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    Blackbody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewaterobjectSurfaceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewaterobjectVelGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewaterobjectVelGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWhitewaterobjectVelGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    Blackbody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone)]
pub struct DopWhitewaterobject {
    pub base: crate::core::types::NodeBase,
}

impl DopWhitewaterobject {
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
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounceforward(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bounceforward".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounceforward_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bounceforward".to_string(),
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
    pub fn with_repellantscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "repellantscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repellantscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repellantscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidesmokedensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplaneval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangecenter(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideiso(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideiso_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideplaneval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidestreamerlen(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevisscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_guiderange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_guiderange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderange3(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "guiderange3".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_guiderange3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guiderange3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_surface_guiderange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guiderange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "vel_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vel_guiderange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_repellantcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "repellantcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_repellantcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repellantcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_surface_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_createframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_vel_guidediv(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "vel_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_vel_guidediv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_guidevismode(mut self, val: DopWhitewaterobjectGuidevismode) -> Self {
        self.base.params.insert(
            "guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidevismode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplane(mut self, val: DopWhitewaterobjectSurfaceGuideplane) -> Self {
        self.base.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guideplane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevismode(
        mut self,
        val: DopWhitewaterobjectSurfaceGuidevismode,
    ) -> Self {
        self.base.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevismode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode(
        mut self,
        val: DopWhitewaterobjectSurfaceGuidevisdensitymode,
    ) -> Self {
        self.base.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideplane(mut self, val: DopWhitewaterobjectVelGuideplane) -> Self {
        self.base.params.insert(
            "vel_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guideplane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevistype(mut self, val: DopWhitewaterobjectVelGuidevistype) -> Self {
        self.base.params.insert(
            "vel_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guidevistype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevismode(mut self, val: DopWhitewaterobjectVelGuidevismode) -> Self {
        self.base.params.insert(
            "vel_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guidevismode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_guidevisramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "guidevisramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_guidevisramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidevisramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevisramp2(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "guidevisramp2".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_guidevisramp2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidevisramp2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "repattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_repattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowcaching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowcaching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showtype(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showtype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showrepellants(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showrepellants".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showrepellants_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showrepellants".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visrepattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visrepattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visrepattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visrepattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidedetectrange(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidedetectrange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidedetectrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidedetectrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usebox(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usebox_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_useboxhash(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_useboxhash_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usesmoke(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usesmoke_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideremapsmoke(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideuseplane(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangemode(mut self, val: bool) -> Self {
        self.base.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusebox(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusebox_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideuseboxhash(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusesmoke(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideoverridediv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidebarbs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidepercomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusestreamers(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideuseplane(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWhitewaterobject {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "whitewaterobject"
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

#[derive(Debug, Clone)]
pub struct DopWhitewatersolver {
    pub base: crate::core::types::NodeBase,
}

impl DopWhitewatersolver {
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

    /// Connects to input 0: "Objects to Solve"
    pub fn set_input_objects_to_solve(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to Solve" and specifies the output index of the target node.
    pub fn set_input_objects_to_solve_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Extra Sources"
    pub fn set_input_extra_sources(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Extra Sources" and specifies the output index of the target node.
    pub fn set_input_extra_sources_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Particle Forces"
    pub fn set_input_particle_forces(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Particle Forces" and specifies the output index of the target node.
    pub fn set_input_particle_forces_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Post-Solve"
    pub fn set_input_post_solve(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Post-Solve" and specifies the output index of the target node.
    pub fn set_input_post_solve_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
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
    pub fn with_goaldepth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "goaldepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_goaldepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goaldepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthrange(mut self, val: f32) -> Self {
        self.base.params.insert(
            "depthrange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emissionamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "emissionamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emissionamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "emissionamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veloffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "veloffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veloffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "veloffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocitymultiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "velocitymultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocitymultiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velocitymultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxspeed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lifespan(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lifespan".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifespan_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lifespan".to_string(),
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
    pub fn with_bubblesagingrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bubblesagingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bubblesagingrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bubblesagingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foamagingrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "foamagingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foamagingrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "foamagingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sprayagingrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sprayagingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sprayagingrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sprayagingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten(mut self, val: f32) -> Self {
        self.base.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "resistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_resistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "multiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_multiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "defstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pbfinclusion(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pbfinclusion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pbfinclusion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pbfinclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxacceleration(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxacceleration".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxacceleration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxacceleration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tensileq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tensileq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tensileq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tensileq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tensilek(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tensilek".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tensilek_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tensilek".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosityc(mut self, val: f32) -> Self {
        self.base.params.insert(
            "viscosityc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosityc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viscosityc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_erosionrange(mut self, val: f32) -> Self {
        self.base.params.insert(
            "erosionrange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erosionrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "erosionrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_erosionstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "erosionstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erosionstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "erosionstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preservationstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "preservationstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_preservationstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preservationstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reppulserange(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reppulserange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reppulserange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reppulserange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repdensitythreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "repdensitythreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repdensitythreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repdensitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repellantseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "repellantseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repellantseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repellantseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthcontrolrange(mut self, val: f32) -> Self {
        self.base.params.insert(
            "depthcontrolrange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthcontrolrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depthcontrolrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "velangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velangle".to_string(),
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

    // --- Float2 parameters ---
    pub fn with_projectionrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "projectionrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_projectionrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projectionrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pbfdepthrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "pbfdepthrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pbfdepthrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pbfdepthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernelradiusrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "kernelradiusrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_kernelradiusrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kernelradiusrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repfeatsizerange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "repfeatsizerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repfeatsizerange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repfeatsizerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repmagrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "repmagrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repmagrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repmagrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repnoiserange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "repnoiserange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repnoiserange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repnoiserange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repliferange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "repliferange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repliferange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repliferange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repspeedrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "repspeedrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repspeedrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repspeedrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repaccelrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "repaccelrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repaccelrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repaccelrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_freq(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_limitsize(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "limitsize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_limitsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitt(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "limitt".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_limitt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vislimitcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vislimitcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vislimitcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vislimitcolor".to_string(),
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
    pub fn with_buoyancy(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "buoyancy".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_buoyancy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "buoyancy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_turb(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("turb".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_turb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "turb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_desiredneighbors(mut self, val: i32) -> Self {
        self.base.params.insert(
            "desiredneighbors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_desiredneighbors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "desiredneighbors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxneighbors(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxneighbors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "constraintiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_constraintiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repseedframes(mut self, val: i32) -> Self {
        self.base.params.insert(
            "repseedframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_repseedframes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repseedframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfhalfwidth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "sdfhalfwidth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdfhalfwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sdfhalfwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_streamname_input(mut self, val: i32) -> Self {
        self.base.params.insert(
            "streamname_input".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_streamname_input_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "streamname_input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_buoyancycurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "buoyancycurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_buoyancycurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "buoyancycurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advectioncurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "advectioncurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_advectioncurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "advectioncurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multipliercurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "multipliercurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_multipliercurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multipliercurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stiffnesscurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "stiffnesscurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_stiffnesscurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stiffnesscurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raddistributionramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "raddistributionramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_raddistributionramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "raddistributionramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strengthramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "strengthramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_strengthramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strengthramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "accelerationramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_accelerationramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "accelerationramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsioncurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "repulsioncurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_repulsioncurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repulsioncurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adhesionstiffnesscurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "adhesionstiffnesscurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_adhesionstiffnesscurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adhesionstiffnesscurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_source(mut self, val: &str) -> Self {
        self.base.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emissionsource(mut self, val: &str) -> Self {
        self.base.params.insert(
            "emissionsource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emissionsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "emissionsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type(mut self, val: &str) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionsop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collisionsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collisionsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionsdf(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collisionsdf".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collisionsdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionsdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "delattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_delattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_importvolumes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importvolumes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importvolumes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importvolumes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addstatevars(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addstatevars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addstatevars_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addstatevars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adddensityvar(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adddensityvar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adddensityvar_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adddensityvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitemission(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limitemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitemission_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectemission(mut self, val: bool) -> Self {
        self.base.params.insert(
            "projectemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectemission_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projectemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addnoise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addnoise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addnoise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addnoise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionpresent(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collisionpresent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionpresent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionpresent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closedends(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closedends_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexpos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexneg(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexneg_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeypos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeypos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeyneg(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeyneg_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezpos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezneg(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezneg_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vislimit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vislimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vislimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vislimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledensitycontrol(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledensitycontrol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledensitycontrol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledensitycontrol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablexpbdstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablexpbdstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablexpbdstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablexpbdstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivekernelradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adaptivekernelradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptivekernelradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adaptivekernelradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableviscosity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableviscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableerosion(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableerosion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableerosion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableerosion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerepellants(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerepellants".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerepellants_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerepellants".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mortalrepellants(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mortalrepellants".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mortalrepellants_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mortalrepellants".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repseedbydensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "repseedbydensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_repseedbydensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "repseedbydensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raddistribution(mut self, val: bool) -> Self {
        self.base.params.insert(
            "raddistribution".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_raddistribution_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "raddistribution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenuatebyspeed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "attenuatebyspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenuatebyspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attenuatebyspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenuatebyaccel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "attenuatebyaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenuatebyaccel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attenuatebyaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledepthcontrol(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledepthcontrol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledepthcontrol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledepthcontrol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthcontrolgrav(mut self, val: bool) -> Self {
        self.base.params.insert(
            "depthcontrolgrav".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_depthcontrolgrav_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depthcontrolgrav".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfactivate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sdfactivate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdfactivate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sdfactivate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useopencl(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useopencl".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useopencl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useopencl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWhitewatersolver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "whitewatersolver"
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
pub enum DopWindforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWindforceNoisetype {
    HermiteInterpolation = 0,
    SparseConvolution = 1,
    ImprovedHermite = 2,
    AlligatorNoise = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWindforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWindforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWindforce {
    pub base: crate::core::types::NodeBase,
}

impl DopWindforce {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_scaleforce(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scaleforce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scaleforce_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scaleforce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenuation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "attenuation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attenuation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attenuation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frequency(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "frequency".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_frequency_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frequency".to_string(),
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
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

    // --- Int parameters ---
    pub fn with_seed(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fractaldepth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "fractaldepth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fractaldepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fractaldepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_samplemode(mut self, val: DopWindforceSamplemode) -> Self {
        self.base.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisetype(mut self, val: DopWindforceNoisetype) -> Self {
        self.base.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopWindforceDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopWindforceSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWindforce {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "windforce"
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

#[derive(Debug, Clone)]
pub struct DopWireangularconstraint {
    pub base: crate::core::types::NodeBase,
}

impl DopWireangularconstraint {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.base.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goalpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalrotation(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goalrotation".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_ptgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "goalgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalpts(mut self, val: &str) -> Self {
        self.base.params.insert(
            "goalpts".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useanimation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useanimation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanimation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useanimation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireangularconstraint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireangularconstraint"
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

#[derive(Debug, Clone)]
pub struct DopWireangularspringconstraint {
    pub base: crate::core::types::NodeBase,
}

impl DopWireangularspringconstraint {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_strength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtorque(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxtorque".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtorque_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxtorque".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxrotation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxrotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.base.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goalpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalrotation(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goalrotation".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_ptgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "goalgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalpts(mut self, val: &str) -> Self {
        self.base.params.insert(
            "goalpts".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useanimation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useanimation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanimation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useanimation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limittorque(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limittorque".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limittorque_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limittorque".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitrotation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limitrotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireangularspringconstraint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireangularspringconstraint"
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

#[derive(Debug, Clone)]
pub struct DopWireconfigureobject {
    pub base: crate::core::types::NodeBase,
}

impl DopWireconfigureobject {
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

    /// Connects to input 0: "Input 0"
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 0" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fracturethreshold".to_string(),
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
    pub fn with_collisionwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "collisionwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionwidth".to_string(),
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
    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_torquescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_torquescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactsscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "impactsscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impactsscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactsscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axisscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "axisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_axisscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axisscale".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "widthcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_widthcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_penetrationcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_penetrationcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fexternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fexternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "texternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_texternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "finternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_finternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tinternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tinternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tinternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fcollisioncolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fcollisioncolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fcollisioncolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fcollisioncolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fconstraintcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fconstraintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fconstraintcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fconstraintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tconstraintcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tconstraintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tconstraintcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tconstraintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "impactscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impactscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "yaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialpose(mut self, val: &str) -> Self {
        self.base.params.insert(
            "initialpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initialpose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "restgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.base.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_animategeom(mut self, val: bool) -> Self {
        self.base.params.insert(
            "animategeom".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategeom_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animategeom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidevolume(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collidevolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidevolume_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidevolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidewire(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collidewire".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidewire_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidewire".to_string(),
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
    pub fn with_visualizewidth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visualizewidth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualizewidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualizewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualizepenetration(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visualizepenetration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualizepenetration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualizepenetration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fexternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "texternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_texternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tinternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tinternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tinternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fcollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fcollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fconstraint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tconstraint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impacts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "impacts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impacts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impacts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactssubstep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "impactssubstep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impactssubstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactssubstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireconfigureobject {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireconfigureobject"
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
pub enum DopWireelasticityParmopKlinear {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticityParmopDamplinear {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticityParmopKangular {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticityParmopDampangular {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticityParmopAdjustforlength {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticityParmopAdjustformass {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticityDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireelasticitySharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWireelasticity {
    pub base: crate::core::types::NodeBase,
}

impl DopWireelasticity {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_klinear(mut self, val: DopWireelasticityParmopKlinear) -> Self {
        self.base.params.insert(
            "parmop_klinear".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_klinear_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_klinear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_damplinear(mut self, val: DopWireelasticityParmopDamplinear) -> Self {
        self.base.params.insert(
            "parmop_damplinear".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_damplinear_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_damplinear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_kangular(mut self, val: DopWireelasticityParmopKangular) -> Self {
        self.base.params.insert(
            "parmop_kangular".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_kangular_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_kangular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_dampangular(mut self, val: DopWireelasticityParmopDampangular) -> Self {
        self.base.params.insert(
            "parmop_dampangular".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampangular_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_dampangular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_adjustforlength(
        mut self,
        val: DopWireelasticityParmopAdjustforlength,
    ) -> Self {
        self.base.params.insert(
            "parmop_adjustforlength".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_adjustforlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_adjustforlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_adjustformass(mut self, val: DopWireelasticityParmopAdjustformass) -> Self {
        self.base.params.insert(
            "parmop_adjustformass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_adjustformass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_adjustformass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopWireelasticityDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopWireelasticitySharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireelasticity {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireelasticity"
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

#[derive(Debug, Clone)]
pub struct DopWireglueconstraint {
    pub base: crate::core::types::NodeBase,
}

impl DopWireglueconstraint {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.base.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goalpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalrotation(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goalrotation".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_ptgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "goalgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalpts(mut self, val: &str) -> Self {
        self.base.params.insert(
            "goalpts".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goalpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useanimation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useanimation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanimation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useanimation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainposition(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constrainposition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainposition_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constrainposition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainrotation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constrainrotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constrainrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireglueconstraint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireglueconstraint"
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

#[derive(Debug, Clone)]
pub struct DopWireobject {
    pub base: crate::core::types::NodeBase,
}

impl DopWireobject {
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
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createframe".to_string(),
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
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fracturethreshold".to_string(),
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
    pub fn with_collisionwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "collisionwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionwidth".to_string(),
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
    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_torquescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_torquescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactsscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "impactsscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impactsscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactsscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axisscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "axisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_axisscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axisscale".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "widthcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_widthcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_penetrationcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_penetrationcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fexternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fexternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "texternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_texternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "finternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_finternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tinternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tinternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tinternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fcollisioncolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fcollisioncolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fcollisioncolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fcollisioncolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fconstraintcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fconstraintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fconstraintcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fconstraintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tconstraintcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tconstraintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tconstraintcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tconstraintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "impactscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impactscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "yaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialpose(mut self, val: &str) -> Self {
        self.base.params.insert(
            "initialpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initialpose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "restgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.base.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvefirstframe".to_string(),
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
    pub fn with_animategeom(mut self, val: bool) -> Self {
        self.base.params.insert(
            "animategeom".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategeom_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animategeom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "importtargetgeometry".to_string(),
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
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidevolume(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collidevolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidevolume_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidevolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidewire(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collidewire".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidewire_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidewire".to_string(),
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
    pub fn with_visualizewidth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visualizewidth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualizewidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualizewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualizepenetration(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visualizepenetration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualizepenetration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualizepenetration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fexternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "texternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_texternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tinternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tinternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tinternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fcollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fcollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fconstraint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tconstraint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impacts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "impacts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impacts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impacts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactssubstep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "impactssubstep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impactssubstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactssubstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireobject {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireobject"
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
pub enum DopWirephysparmsParmopComputemass {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsParmopDensity {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsParmopMass {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsParmopWidth {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsParmopFriction {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsParmopDynamicfriction {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirephysparmsSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWirephysparms {
    pub base: crate::core::types::NodeBase,
}

impl DopWirephysparms {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_computemass(mut self, val: DopWirephysparmsParmopComputemass) -> Self {
        self.base.params.insert(
            "parmop_computemass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_computemass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_computemass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_density(mut self, val: DopWirephysparmsParmopDensity) -> Self {
        self.base.params.insert(
            "parmop_density".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_density_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_mass(mut self, val: DopWirephysparmsParmopMass) -> Self {
        self.base.params.insert(
            "parmop_mass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_mass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_mass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_width(mut self, val: DopWirephysparmsParmopWidth) -> Self {
        self.base.params.insert(
            "parmop_width".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_width_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_friction(mut self, val: DopWirephysparmsParmopFriction) -> Self {
        self.base.params.insert(
            "parmop_friction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_friction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_dynamicfriction(
        mut self,
        val: DopWirephysparmsParmopDynamicfriction,
    ) -> Self {
        self.base.params.insert(
            "parmop_dynamicfriction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_dynamicfriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopWirephysparmsDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopWirephysparmsSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWirephysparms {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wirephysparms"
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
pub enum DopWireplasticityParmopPlasticstretchthreshold {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticityParmopPlasticstretchrate {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticityParmopPlasticstretchhardening {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticityParmopPlasticbendthreshold {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticityParmopPlasticbendrate {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticityParmopPlasticbendhardening {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticityDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWireplasticitySharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWireplasticity {
    pub base: crate::core::types::NodeBase,
}

impl DopWireplasticity {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_plasticstretchthreshold(
        mut self,
        val: DopWireplasticityParmopPlasticstretchthreshold,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticstretchthreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticstretchthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticstretchthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_plasticstretchrate(
        mut self,
        val: DopWireplasticityParmopPlasticstretchrate,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticstretchrate".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticstretchrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticstretchrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_plasticstretchhardening(
        mut self,
        val: DopWireplasticityParmopPlasticstretchhardening,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticstretchhardening".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticstretchhardening_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticstretchhardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_plasticbendthreshold(
        mut self,
        val: DopWireplasticityParmopPlasticbendthreshold,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticbendthreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticbendthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticbendthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_plasticbendrate(
        mut self,
        val: DopWireplasticityParmopPlasticbendrate,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticbendrate".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticbendrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticbendrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_plasticbendhardening(
        mut self,
        val: DopWireplasticityParmopPlasticbendhardening,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticbendhardening".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticbendhardening_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticbendhardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopWireplasticityDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopWireplasticitySharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWireplasticity {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireplasticity"
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
pub enum DopWiresolverParmopSpatialscale {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopMinsubsteps {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopTol {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopFinternal {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopFexternal {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopPlasticdeformation {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopCollisionhandling {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverCollisionhandling {
    Sdf = 0,
    LocalGeometric = 1,
    GlobalGeometric = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverParmopResolvemaxpasses {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWiresolverDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone)]
pub struct DopWiresolver {
    pub base: crate::core::types::NodeBase,
}

impl DopWiresolver {
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

    // --- Int parameters ---
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

    // --- Menu parameters ---
    pub fn with_parmop_spatialscale(mut self, val: DopWiresolverParmopSpatialscale) -> Self {
        self.base.params.insert(
            "parmop_spatialscale".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_spatialscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_spatialscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_minsubsteps(mut self, val: DopWiresolverParmopMinsubsteps) -> Self {
        self.base.params.insert(
            "parmop_minsubsteps".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minsubsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_minsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_tol(mut self, val: DopWiresolverParmopTol) -> Self {
        self.base.params.insert(
            "parmop_tol".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_tol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_finternal(mut self, val: DopWiresolverParmopFinternal) -> Self {
        self.base.params.insert(
            "parmop_finternal".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_finternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_finternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_fexternal(mut self, val: DopWiresolverParmopFexternal) -> Self {
        self.base.params.insert(
            "parmop_fexternal".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_fexternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_fexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_plasticdeformation(
        mut self,
        val: DopWiresolverParmopPlasticdeformation,
    ) -> Self {
        self.base.params.insert(
            "parmop_plasticdeformation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_plasticdeformation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_plasticdeformation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_collisionhandling(
        mut self,
        val: DopWiresolverParmopCollisionhandling,
    ) -> Self {
        self.base.params.insert(
            "parmop_collisionhandling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_collisionhandling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_collisionhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionhandling(mut self, val: DopWiresolverCollisionhandling) -> Self {
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
    pub fn with_parmop_resolvemaxpasses(
        mut self,
        val: DopWiresolverParmopResolvemaxpasses,
    ) -> Self {
        self.base.params.insert(
            "parmop_resolvemaxpasses".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_resolvemaxpasses_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_resolvemaxpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopWiresolverDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_finternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fexternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticdeformation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "plasticdeformation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_plasticdeformation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plasticdeformation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWiresolver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wiresolver"
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
pub enum DopWirevisualizationSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWirevisualization {
    pub base: crate::core::types::NodeBase,
}

impl DopWirevisualization {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_torquescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_torquescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactsscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "impactsscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impactsscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactsscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axisscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "axisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_axisscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_widthcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "widthcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_widthcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_penetrationcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_penetrationcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "penetrationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fexternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fexternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "texternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_texternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "finternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_finternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinternalcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tinternalcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tinternalcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tinternalcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fcollisioncolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fcollisioncolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fcollisioncolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fcollisioncolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fconstraintcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fconstraintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fconstraintcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fconstraintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tconstraintcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tconstraintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tconstraintcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tconstraintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "impactscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impactscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "yaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zaxiscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxiscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zaxiscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopWirevisualizationSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_width(mut self, val: bool) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_penetration(mut self, val: bool) -> Self {
        self.base.params.insert(
            "penetration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_penetration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "penetration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fexternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fexternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "texternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_texternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinternal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tinternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tinternal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tinternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fcollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fcollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fconstraint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tconstraint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tconstraint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tconstraint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impacts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "impacts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impacts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impacts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impactssubstep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "impactssubstep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impactssubstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "impactssubstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWirevisualization {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wirevisualization"
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
pub enum DopWirevolumecolliderParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirevolumecolliderParmopReverseobjectroles {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirevolumecolliderDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopWirevolumecolliderSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWirevolumecollider {
    pub base: crate::core::types::NodeBase,
}

impl DopWirevolumecollider {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_offset(mut self, val: DopWirevolumecolliderParmopOffset) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_reverseobjectroles(
        mut self,
        val: DopWirevolumecolliderParmopReverseobjectroles,
    ) -> Self {
        self.base.params.insert(
            "parmop_reverseobjectroles".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_reverseobjectroles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_reverseobjectroles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopWirevolumecolliderDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopWirevolumecolliderSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_reverseobjectroles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reverseobjectroles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reverseobjectroles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reverseobjectroles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWirevolumecollider {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wirevolumecollider"
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
pub enum DopWirewirecolliderSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopWirewirecollider {
    pub base: crate::core::types::NodeBase,
}

impl DopWirewirecollider {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopWirewirecolliderSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopWirewirecollider {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "wirewirecollider"
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
