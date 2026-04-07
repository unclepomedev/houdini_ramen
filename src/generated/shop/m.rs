#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceReflMaskmonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceReflMetallicmonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceReflRoughmonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceReflMaskmonochannel2 {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceReflRoughmonochannel2 {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceRefrMaskmonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceRefrRoughmonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceSssPcmode {
    GenerateAtRenderTime = 0,
    ReadFromFile = 1,
    WriteToFile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceNormaltexchannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceNormaltexnormalspace {
    /// 0 to 1
    N0To1 = 0,
    /// -1 to 1
    Minus1To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceCoatnormaltexchannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceCoatnormaltexnormalspace {
    /// 0 to 1
    N0To1 = 0,
    /// -1 to 1
    Minus1To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceDisptexchannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceBakeNormalspace {
    /// 0 to 1
    N0To1 = 0,
    /// -1 to 1
    Minus1To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopMantrasurfaceOglRoughmapComp {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3,
}

#[derive(Debug, Clone)]
pub struct ShopMantrasurface {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopMantrasurface {
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
    pub fn with_diff_int(mut self, val: f32) -> Self {
        self.params.insert(
            "diff_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diff_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "diff_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diff_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_min(mut self, val: f32) -> Self {
        self.params.insert(
            "diff_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diff_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colortextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "diff_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diff_colortextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "diff_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diff_colortexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_int(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_maskTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_masktexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_min(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_colortextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_colortexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_metallic(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_metallic".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_metallic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_metallic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_metallicTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_metallictexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_edgeTintTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_edgetinttextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_edgeTintTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_edgetinttexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_roughTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_roughtexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_aniso(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_aniso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_aniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_aniso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_anisodir(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_anisodir".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_anisodir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_anisodir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_int(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_int2(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_int2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_int2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_int2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturefilterwidth2(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_maskTextureFilterWidth2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_masktexturefilterwidth2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureFilterWidth2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_min2(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_min2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_min2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_min2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortextureintensity2(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_colorTextureIntensity2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_colortextureintensity2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureIntensity2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturefilterwidth2(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_colorTextureFilterWidth2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_colortexturefilterwidth2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureFilterWidth2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_rough2(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_rough2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_rough2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_rough2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturefilterwidth2(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_roughTextureFilterWidth2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_roughtexturefilterwidth2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureFilterWidth2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_aniso2(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_aniso2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_aniso2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_aniso2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_anisodir2(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_anisodir2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_anisodir2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_anisodir2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_int2(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_int2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_int2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_int2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_rough2(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_rough2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_rough2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_rough2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_int(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_maskTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_masktexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_min(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colortextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_colortextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_colortexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_roughTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_roughtexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_aniso(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_aniso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_aniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_aniso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_anisodir(mut self, val: f32) -> Self {
        self.params.insert(
            "refr_anisodir".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refr_anisodir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_anisodir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_den(mut self, val: f32) -> Self {
        self.params.insert(
            "atten_den".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_den_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_den".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_int(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_min(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_phase(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_phase".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_phase".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_1intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_1intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_1intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_1intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_2intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_2intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_2intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_2intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_2quality(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_2quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_2quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_2quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colortextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_colortextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_colortexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_atten(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_atten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_atten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_atten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_attenColorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_attencolortextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "sss_attenColorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sss_attencolortexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emit_int(mut self, val: f32) -> Self {
        self.params.insert(
            "emit_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emit_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emit_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_textureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "emission_textureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emission_textureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_textureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_texturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "emission_textureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emission_texturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_textureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac_int(mut self, val: f32) -> Self {
        self.params.insert(
            "opac_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opac_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colortextureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "opacity_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacity_colortextureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorTextureIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "opacity_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacity_colortexturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorTextureFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac_para(mut self, val: f32) -> Self {
        self.params.insert(
            "opac_para".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opac_para_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_para".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac_perp(mut self, val: f32) -> Self {
        self.params.insert(
            "opac_perp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opac_perp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_perp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac_rolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "opac_rolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opac_rolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_rolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fake_transmit(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_transmit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_transmit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_transmit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fake_shadow(mut self, val: f32) -> Self {
        self.params.insert(
            "fake_shadow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fake_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexscale(mut self, val: f32) -> Self {
        self.params.insert(
            "normalTexScale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaltexscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexScale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexfilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "normalTexFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaltexfilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexscale(mut self, val: f32) -> Self {
        self.params.insert(
            "coatNormalTexScale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatnormaltexscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexScale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexfilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "coatNormalTexFilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatnormaltexfilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexFilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_displacebound(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_displacebound".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_displacebound_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_displacebound".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "dispTexOffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disptexoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexOffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dispTexScale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disptexscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexScale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexfilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "dispTexfilterWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disptexfilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexfilterWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseamp(mut self, val: f32) -> Self {
        self.params.insert(
            "dispNoiseAmp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispnoiseamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseAmp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiserough(mut self, val: f32) -> Self {
        self.params.insert(
            "dispNoiseRough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispnoiserough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseRough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseatten(mut self, val: f32) -> Self {
        self.params.insert(
            "dispNoiseAtten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispnoiseatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseAtten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_occlusionbias(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_occlusionbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_occlusionbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_occlusionbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_cavitydistance(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_cavitydistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_cavitydistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_cavitydistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_cavitybias(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_cavitybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_cavitybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_cavitybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_curvaturescale(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_curvaturescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_curvaturescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_curvaturescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ior_in(mut self, val: f32) -> Self {
        self.params.insert(
            "ior_in".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ior_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ior_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ior_out(mut self, val: f32) -> Self {
        self.params.insert(
            "ior_out".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ior_out_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ior_out".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_ior_inner(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_ior_inner".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_ior_inner_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_ior_inner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_ior_outer(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_ior_outer".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_ior_outer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_ior_outer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_envscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_curvaturebias(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_curvaturebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_curvaturebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_curvaturebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_ogl_shinyrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_shinyrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_diff_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_diff_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_edgetint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec_edgetint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_edgetint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_edgetint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl_edgetint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_edgetint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgetint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_color2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec_color2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_color2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_color2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_color2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl_color2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_color2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_color2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colorbasecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refr_colorBaseColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refr_colorbasecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorBaseColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "atten_clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_atten_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sss_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sss_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sss_attenColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sss_attencolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "emission_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emission_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opacity_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opacity_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoisefreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dispNoiseFreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dispnoisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseFreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dispNoiseOffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dispnoiseoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseOffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_emit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_emit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_envrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiglobclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "multiglobclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_multiglobclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multiglobclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuse_color_noshading(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diffuse_color_noshading".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_diffuse_color_noshading_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuse_color_noshading".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_reflectivity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_reflectivity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_reflectivity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_reflectivity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_refl_maskmonochannel(mut self, val: ShopMantrasurfaceReflMaskmonochannel) -> Self {
        self.params.insert(
            "refl_maskMonoChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refl_maskmonochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskMonoChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallicmonochannel(
        mut self,
        val: ShopMantrasurfaceReflMetallicmonochannel,
    ) -> Self {
        self.params.insert(
            "refl_metallicMonoChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refl_metallicmonochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicMonoChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughmonochannel(
        mut self,
        val: ShopMantrasurfaceReflRoughmonochannel,
    ) -> Self {
        self.params.insert(
            "refl_roughMonoChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refl_roughmonochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughMonoChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_maskmonochannel2(
        mut self,
        val: ShopMantrasurfaceReflMaskmonochannel2,
    ) -> Self {
        self.params.insert(
            "refl_maskMonoChannel2".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refl_maskmonochannel2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskMonoChannel2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughmonochannel2(
        mut self,
        val: ShopMantrasurfaceReflRoughmonochannel2,
    ) -> Self {
        self.params.insert(
            "refl_roughMonoChannel2".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refl_roughmonochannel2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughMonoChannel2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_maskmonochannel(mut self, val: ShopMantrasurfaceRefrMaskmonochannel) -> Self {
        self.params.insert(
            "refr_maskMonoChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refr_maskmonochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskMonoChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughmonochannel(
        mut self,
        val: ShopMantrasurfaceRefrRoughmonochannel,
    ) -> Self {
        self.params.insert(
            "refr_roughMonoChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_refr_roughmonochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughMonoChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_1quality(mut self, val: i32) -> Self {
        self.params.insert(
            "sss_1quality".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sss_1quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_1quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_pcmode(mut self, val: ShopMantrasurfaceSssPcmode) -> Self {
        self.params.insert(
            "sss_pcmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sss_pcmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_pcmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexchannel(mut self, val: ShopMantrasurfaceNormaltexchannel) -> Self {
        self.params.insert(
            "normalTexChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_normaltexchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexnormalspace(mut self, val: ShopMantrasurfaceNormaltexnormalspace) -> Self {
        self.params.insert(
            "normalTexNormalSpace".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_normaltexnormalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexNormalSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexchannel(mut self, val: ShopMantrasurfaceCoatnormaltexchannel) -> Self {
        self.params.insert(
            "coatNormalTexChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coatnormaltexchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexnormalspace(
        mut self,
        val: ShopMantrasurfaceCoatnormaltexnormalspace,
    ) -> Self {
        self.params.insert(
            "coatNormalTexNormalSpace".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coatnormaltexnormalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexNormalSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexchannel(mut self, val: ShopMantrasurfaceDisptexchannel) -> Self {
        self.params.insert(
            "dispTexChannel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_disptexchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexChannel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseturb(mut self, val: i32) -> Self {
        self.params.insert(
            "dispNoiseTurb".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dispnoiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseTurb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "bake_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bake_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_normalspace(mut self, val: ShopMantrasurfaceBakeNormalspace) -> Self {
        self.params.insert(
            "bake_normalspace".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_bake_normalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_normalspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_speclayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_speclayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_roughmap_comp(mut self, val: ShopMantrasurfaceOglRoughmapComp) -> Self {
        self.params.insert(
            "ogl_roughmap_comp".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_ogl_roughmap_comp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_roughmap_comp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitylayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_opacitylayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumplayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_bumplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normallayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_normallayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_diff_colortexture(mut self, val: &str) -> Self {
        self.params.insert(
            "diff_colorTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diff_colortexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "diff_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "diff_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "diff_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diff_colortexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_model(mut self, val: &str) -> Self {
        self.params.insert(
            "spec_model".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_spec_model_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_model".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_metallicTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_metallictexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_edgetinttexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_model2(mut self, val: &str) -> Self {
        self.params.insert(
            "spec_model2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_spec_model2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_model2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexture2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTexture2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturewrap2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTextureWrap2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturewrap2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureWrap2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturefilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTextureFilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturefilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureFilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturecolorspace2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_maskTextureColorSpace2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_masktexturecolorspace2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskTextureColorSpace2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexture2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTexture2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturewrap2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTextureWrap2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturewrap2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureWrap2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturefilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTextureFilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturefilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureFilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturecolorspace2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_colorTextureColorSpace2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_colortexturecolorspace2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorTextureColorSpace2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexture2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTexture2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturewrap2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTextureWrap2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturewrap2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureWrap2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturefilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTextureFilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturefilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureFilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturecolorspace2(mut self, val: &str) -> Self {
        self.params.insert(
            "refl_roughTextureColorSpace2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refl_roughtexturecolorspace2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughTextureColorSpace2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_model(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_model".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_model_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_model".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_masktexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_maskTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_masktexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_maskTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_maskTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_maskTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_masktexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colortexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_colorTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_colortexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_colortexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexture(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_roughTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_roughTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_roughTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "refr_roughTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refr_roughtexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_2model(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_2model".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_2model_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_2model".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_pcname(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_pcname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_pcname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_pcname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colortexture(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_colorTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_colortexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_colortexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexture(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_attenColorTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sss_attencolortexturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorTextureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "emission_texture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emission_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_texture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "emission_textureColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emission_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_textureColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexture(mut self, val: &str) -> Self {
        self.params.insert(
            "opacity_colorTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "opacity_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorTextureWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "opacity_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opacity_colortexturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorTextureFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texturesourcecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "textureSourceColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texturesourcecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "textureSourceColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltextype(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexType".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexcolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexvectorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexVectorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexvectorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexVectorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexture(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalteximageplane(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexImagePlane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normalteximageplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexImagePlane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltextype(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexType".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexcolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexvectorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexVectorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexvectorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexVectorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexture(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormalteximageplane(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexImagePlane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormalteximageplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexImagePlane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptextype(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexType".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexcolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexColorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexColorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexvectorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexVectorSpace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexvectorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexVectorSpace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptextexture(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexTexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptextexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexWrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexWrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoisetype(mut self, val: &str) -> Self {
        self.params.insert(
            "dispNoiseType".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dispnoisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseType".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_spec_model(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_spec_model".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_spec_model_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_spec_model".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_specmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_roughmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_roughmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_roughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_roughmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitymap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumptype(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_type(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalmap_type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalmap_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotorder(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_diff_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "diff_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colorusebasecolor(mut self, val: bool) -> Self {
        self.params.insert(
            "diff_colorUseBaseColor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_colorusebasecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorUseBaseColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colorusepointcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "diff_colorUsePointColor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_colorusepointcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorUsePointColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colorusepackedcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "diff_colorUsePackedColor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_colorusepackedcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorUsePackedColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colorusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "diff_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_colorusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff_colorudim(mut self, val: bool) -> Self {
        self.params.insert(
            "diff_colorUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_colorudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff_colorUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_lights(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_lights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_lights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_objs(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_objs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_objs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_objs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_maskusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_maskUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_maskusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_maskudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_maskUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_maskudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colorusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_colorusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colorudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_colorUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_colorudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallicusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_metallicUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_metallicusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_metallicudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_metallicUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_metallicudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_metallicUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetintusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_edgeTintUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_edgetintusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_edgetintudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_edgeTintUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_edgetintudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_edgeTintUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_roughUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_roughusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_roughUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_roughudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_sep(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_sep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_sep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_sep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_enable2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_enable2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_enable2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_enable2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_lights2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_lights2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_lights2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_lights2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_objs2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_objs2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_objs2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_objs2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_maskusetexture2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_maskUseTexture2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_maskusetexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskUseTexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_maskudim2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_maskUdim2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_maskudim2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_maskUdim2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colorusetexture2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_colorUseTexture2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_colorusetexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorUseTexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_colorudim2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_colorUdim2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_colorudim2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_colorUdim2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughusetexture2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_roughUseTexture2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_roughusetexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughUseTexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_roughudim2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_roughUdim2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_roughudim2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_roughUdim2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_sep2(mut self, val: bool) -> Self {
        self.params.insert(
            "refl_sep2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refl_sep2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_sep2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_lights(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_lights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_lights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_objs(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_objs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_objs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_objs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_thin(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_thin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_thin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_thin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_maskusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_maskUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_maskusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_maskudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_maskUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_maskudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_maskUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colorusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_colorusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_colorudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_colorUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_colorudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_colorUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_roughUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_roughusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refr_roughudim(mut self, val: bool) -> Self {
        self.params.insert(
            "refr_roughUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refr_roughudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refr_roughUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "atten_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_atten_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_1enable(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_1enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_1enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_1enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_2enable(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_2enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_2enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_2enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colorusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_colorusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_colorudim(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_colorUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_colorudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_colorUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolorusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_attenColorUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_attencolorusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_attencolorudim(mut self, val: bool) -> Self {
        self.params.insert(
            "sss_attenColorUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_attencolorudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss_attenColorUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emit_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "emit_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emit_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emit_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emit_illum(mut self, val: bool) -> Self {
        self.params.insert(
            "emit_illum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emit_illum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emit_illum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "emission_useTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emission_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_useTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission_udim(mut self, val: bool) -> Self {
        self.params.insert(
            "emission_udim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emission_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_udim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colorusetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "opacity_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_opacity_colorusetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorUseTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_colorudim(mut self, val: bool) -> Self {
        self.params.insert(
            "opacity_colorUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_opacity_colorudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_colorUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac_falloff(mut self, val: bool) -> Self {
        self.params.insert(
            "opac_falloff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_opac_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fake_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fake_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fake_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablebumpornormaltexture(mut self, val: bool) -> Self {
        self.params.insert(
            "enableBumpOrNormalTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablebumpornormaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableBumpOrNormalTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexudim(mut self, val: bool) -> Self {
        self.params.insert(
            "normalTexUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normaltexudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexnormalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "normalTexNormalFlipX".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normaltexnormalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexNormalFlipX".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexnormalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "normalTexNormalFlipY".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normaltexnormalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexNormalFlipY".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_separatecoatnormals(mut self, val: bool) -> Self {
        self.params.insert(
            "separateCoatNormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_separatecoatnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "separateCoatNormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoatnormaltexture(mut self, val: bool) -> Self {
        self.params.insert(
            "enableCoatNormalTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoatnormaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableCoatNormalTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexudim(mut self, val: bool) -> Self {
        self.params.insert(
            "coatNormalTexUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatnormaltexudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipX".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipX".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipY".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipY".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_disable_displace_shader(mut self, val: bool) -> Self {
        self.params.insert(
            "shop_disable_displace_shader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shop_disable_displace_shader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_disable_displace_shader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_truedisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_truedisplace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_truedisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_truedisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledisptexture(mut self, val: bool) -> Self {
        self.params.insert(
            "enableDispTexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledisptexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableDispTexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexudim(mut self, val: bool) -> Self {
        self.params.insert(
            "dispTexUdim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disptexudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexUdim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledispnoise(mut self, val: bool) -> Self {
        self.params.insert(
            "enableDispNoise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledispnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableDispNoise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_tangentnormalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "bake_tangentnormalflipx".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bake_tangentnormalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_tangentnormalflipx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_tangentnormalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "bake_tangentnormalflipy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bake_tangentnormalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_tangentnormalflipy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_includedispinnt(mut self, val: bool) -> Self {
        self.params.insert(
            "bake_includedispinnt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bake_includedispinnt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_includedispinnt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_facefwd(mut self, val: bool) -> Self {
        self.params.insert(
            "facefwd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_facefwd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "facefwd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conserveenergy(mut self, val: bool) -> Self {
        self.params.insert(
            "conserveenergy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_conserveenergy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conserveenergy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fres_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "fres_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fres_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fres_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_light(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_light_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_cutout(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_cutout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_adjustshiny(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_adjustshiny_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_bumpinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_normalflipx".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_normalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalflipx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_normalflipy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_normalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalflipy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopMantrasurface {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mantrasurface"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct ShopMaterial {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopMaterial {
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
}

impl crate::core::types::HoudiniNode for ShopMaterial {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "material"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct ShopMatnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopMatnet {
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
}

impl crate::core::types::HoudiniNode for ShopMatnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "matnet"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct ShopMergecoshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl ShopMergecoshader {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(self.next_input_index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(self.next_input_index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(self.next_input_index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }
}

impl crate::core::types::HoudiniNode for ShopMergecoshader {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mergecoshader"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
