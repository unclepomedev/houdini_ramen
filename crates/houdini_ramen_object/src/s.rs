#[derive(Debug, Clone)]
pub struct ObjectShopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectShopnet {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ObjectShopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "shopnet"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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
pub enum ObjectSimplebipedViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplebipedVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplebipedPreXform {
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
pub enum ObjectSimplebipedXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplebipedRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplebipedUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectSimplebiped {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectSimplebiped {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_sub_network_input_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_masteruniformscale(mut self, val: f32) -> Self {
        self.params.insert(
            "masteruniformscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_masteruniformscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masteruniformscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlmasterspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlmasterspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlmasterspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlmasterspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrllocalspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrllocalspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrllocalspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlheadspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlheadspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlheadspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlheadspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeleftlidrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeleftlidrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeleftlidrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeleftlidrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyerightlidrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "eyerightlidrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyerightlidrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyerightlidrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "eyelookatcustomspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyelookatcustomspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikleftarm(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikleftarm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikleftarm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikleftarm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristleftspace(mut self, val: f32) -> Self {
        self.params.insert(
            "ikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikwristleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikwristleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikwristleftspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikrightarm(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikrightarm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikrightarm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikrightarm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrightspace(mut self, val: f32) -> Self {
        self.params.insert(
            "ikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikwristrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikwristrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikwristrightspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle1rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle1rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikleftleg(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikleftleg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikleftleg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikleftleg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleglefttwistblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ikleglefttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikleglefttwistblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleglefttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikfootleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikfootleftspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikrightleg(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikrightleg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikrightleg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikrightleg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iklegrighttwistblend(mut self, val: f32) -> Self {
        self.params.insert(
            "iklegrighttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iklegrighttwistblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iklegrighttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikfootrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikfootrightspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspaceblend".to_string(),
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
    pub fn with_mastertranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mastertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mastertranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mastertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_masterrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "masterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_masterrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localmastertraslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localmastertraslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localmastertraslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localmastertraslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localmasterrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localmasterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localmasterrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localmasterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cogtranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cogtranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cogtranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cogtranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cogrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cogrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cogrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cogrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipstranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hipstranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hipstranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipstranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipsrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hipsrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hipsrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipsrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate1".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_neckrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "neckrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_neckrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "neckrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "headrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_headrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeslookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeslookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeslookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeslookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeleftlookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeleftlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeleftlookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeleftlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyerightlookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyerightlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyerightlookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyerightlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shoulderlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shoulderlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shoulderlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shoulderlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleftarmtwist(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikleftarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikleftarmtwist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleftarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperarmleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperarmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperarmleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperarmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forearmleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "forearmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_forearmleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forearmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightshouldertranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightshouldertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightshouldertranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightshouldertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristrighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikrightarmtwist(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikrightarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikrightarmtwist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikrightarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperarmrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperarmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperarmrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperarmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forearmrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "forearmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_forearmrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forearmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb2rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftthumb2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb3rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftthumb3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb2rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightthumb2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb3rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightthumb3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lefthiptranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lefthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lefthiptranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lefthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiplefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiplefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiplefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiplefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetipleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetipleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetipleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetipleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootballleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootballleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootballleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootballleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoeleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoeleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleglefttwisttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikleglefttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikleglefttwisttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleglefttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkthighleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkthighleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkthighleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkthighleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkcalfleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkcalfleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkcalfleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkcalfleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkfootleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkfootleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fktoeleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fktoeleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_righthiptranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "righthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_righthiptranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "righthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootrighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootrighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiprighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiprighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiprighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiprighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiprightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiprightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiprightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiprightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootballrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootballrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootballrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootballrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoerightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoerightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iklegrighttwisttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iklegrighttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iklegrighttwisttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iklegrighttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkthighrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkthighrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkthighrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkthighrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkcalfrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkcalfrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkcalfrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkcalfrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkfootrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkfootrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fktoerightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fktoerightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fktoerightrotate".to_string(),
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
    pub fn with_viewportlod(mut self, val: ObjectSimplebipedViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectSimplebipedVportOnionskin) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectSimplebipedPreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectSimplebipedXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectSimplebipedRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectSimplebipedUparmtype) -> Self {
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
    pub fn with_ctrlmasterspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlmasterspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlmasterspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlmasterspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrllocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrllocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspace(mut self, val: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
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
    pub fn with_enablesns(mut self, val: bool) -> Self {
        self.params.insert(
            "enableSnS".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableSnS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showbones(mut self, val: bool) -> Self {
        self.params.insert(
            "showbones".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showbones_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showbones".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showcontrols(mut self, val: bool) -> Self {
        self.params.insert(
            "showcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showcontrols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showdetailcontrols(mut self, val: bool) -> Self {
        self.params.insert(
            "showdetailcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showdetailcontrols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showdetailcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "showgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showgeometry".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ObjectSimplebiped {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "simplebiped"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectSimplebipedOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectSimplebipedOutputs for ObjectSimplebiped {}
impl ObjectSimplebipedOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSimplebiped>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectSimplebipedInnerExt {
    fn ik_chops(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_calves_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_calves_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_forearms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_forearms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_thighs_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_thighs_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_upperarms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sns_upperarms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_football_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_leg_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_leg_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_cog(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eye_lookat_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eye_lookat_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyelid_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyelid_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyes_lookat(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hips_rotate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hips_translate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_mirror_arm_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_mirror_arm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_customspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_eyes_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_foot_customspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_foot_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_localspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_wrist_custom_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_wrist_custom_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_calves_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_calves_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_chest(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_elbowjoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_elbowjoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_eyelids_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_eyelids_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_eyes_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_eyes_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_fingersbase_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_fingersbase_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_fingersmid_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_fingersmid_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_fingerstips_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_fingerstips_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_forearms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_forearms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_heels_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_heels_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_hipjoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_hipjoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_kneejoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_kneejoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_ribs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_shoulders_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_shoulders_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_stomache(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thighs_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thighs_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thumbbase_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thumbbase_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thumbmid_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thumbmid_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thumbtip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_thumbtip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_toes_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_toes_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_upperarms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_upperarms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_wristjoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_wristjoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geoexport(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master_space1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fkik_arm_left_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fkik_arm_right_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fk_arm_left_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fk_arm_left_tip1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_foot_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_foot_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_left_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_right_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_leg_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_customspace_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_customspace_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_eyes_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_leg_twistspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_leg_twistspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_wrist_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_wrist_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_calf_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_calf_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_cog_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_cog_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_eye_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_eye_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_eyes_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_ankle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_ankle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_footleft_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_footleft_pretransform1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_head_space_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_head_worldspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_hip_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_hip_pretransform1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_arm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_custom_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_foot_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_spine_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_space_cog_world(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_space_cog_world1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_spaceblend_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_spaceblend_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn righ_wrist_worldspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn righ_wrist_worldspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectSimplebipedInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectSimplebiped>
{
    fn ik_chops(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_CHOPS")
    }
    fn sns_calves_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_Calves_left")
    }
    fn sns_calves_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_Calves_right")
    }
    fn sns_forearms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_Forearms_left")
    }
    fn sns_forearms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_Forearms_right")
    }
    fn sns_thighs_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_Thighs_left")
    }
    fn sns_thighs_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_Thighs_right")
    }
    fn sns_upperarms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_UpperArms_left")
    }
    fn sns_upperarms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("SnS_UpperArms_right")
    }
    fn blend1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend1")
    }
    fn ctrl_fk_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_left")
    }
    fn ctrl_fk_calf_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_left_preT")
    }
    fn ctrl_fk_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_right")
    }
    fn ctrl_fk_calf_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_right_preT")
    }
    fn ctrl_fk_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_left")
    }
    fn ctrl_fk_foot_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_left_preT")
    }
    fn ctrl_fk_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_right")
    }
    fn ctrl_fk_foot_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_right_preT")
    }
    fn ctrl_fk_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_left")
    }
    fn ctrl_fk_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_right")
    }
    fn ctrl_fk_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_left")
    }
    fn ctrl_fk_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_right")
    }
    fn ctrl_fk_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_1_left")
    }
    fn ctrl_fk_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_1_right")
    }
    fn ctrl_fk_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_2_left")
    }
    fn ctrl_fk_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_2_right")
    }
    fn ctrl_fk_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_3_left")
    }
    fn ctrl_fk_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_3_right")
    }
    fn ctrl_fk_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_1")
    }
    fn ctrl_fk_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_2")
    }
    fn ctrl_fk_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_3")
    }
    fn ctrl_fk_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thigh_left")
    }
    fn ctrl_fk_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thigh_right")
    }
    fn ctrl_fk_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_1_left")
    }
    fn ctrl_fk_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_1_right")
    }
    fn ctrl_fk_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_2_left")
    }
    fn ctrl_fk_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_2_right")
    }
    fn ctrl_fk_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_3_left")
    }
    fn ctrl_fk_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_3_right")
    }
    fn ctrl_fk_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_left")
    }
    fn ctrl_fk_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_right")
    }
    fn ctrl_fk_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_upperarm_left")
    }
    fn ctrl_fk_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_upperarm_right")
    }
    fn ctrl_ik_arm_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_left")
    }
    fn ctrl_ik_arm_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_right")
    }
    fn ctrl_ik_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_foot_left")
    }
    fn ctrl_ik_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_foot_right")
    }
    fn ctrl_ik_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_football_left")
    }
    fn ctrl_ik_football_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_football_right")
    }
    fn ctrl_ik_leg_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_leg_twist_left")
    }
    fn ctrl_ik_leg_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_leg_twist_right")
    }
    fn ctrl_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toe_left")
    }
    fn ctrl_ik_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toetip_left")
    }
    fn ctrl_ik_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toetip_right")
    }
    fn ctrl_ik_wrist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_left")
    }
    fn ctrl_ik_wrist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_right")
    }
    fn ctrl_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_clavicle_left")
    }
    fn ctrl_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_clavicle_right")
    }
    fn ctrl_cog(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_cog")
    }
    fn ctrl_eye_lookat_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eye_lookat_left")
    }
    fn ctrl_eye_lookat_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eye_lookat_right")
    }
    fn ctrl_eyelid_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyelid_left")
    }
    fn ctrl_eyelid_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyelid_right")
    }
    fn ctrl_eyes_lookat(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyes_lookat")
    }
    fn ctrl_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_head")
    }
    fn ctrl_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_left")
    }
    fn ctrl_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_right")
    }
    fn ctrl_hips_rotate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hips_rotate")
    }
    fn ctrl_hips_translate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hips_translate")
    }
    fn ctrl_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_master")
    }
    fn ctrl_mirror_arm_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_mirror_arm_customspace_right")
    }
    fn ctrl_mirror_arm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_mirror_arm_right")
    }
    fn ctrl_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_neck")
    }
    fn ctrl_rig_bone_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_forearm_left")
    }
    fn ctrl_rig_bone_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_forearm_right")
    }
    fn ctrl_rig_bone_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_hand_left")
    }
    fn ctrl_rig_bone_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_hand_right")
    }
    fn ctrl_rig_bone_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_middle_1_left")
    }
    fn ctrl_rig_bone_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_middle_1_right")
    }
    fn ctrl_rig_bone_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_middle_2_left")
    }
    fn ctrl_rig_bone_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_middle_2_right")
    }
    fn ctrl_rig_bone_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_middle_3_left")
    }
    fn ctrl_rig_bone_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_middle_3_right")
    }
    fn ctrl_rig_bone_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_neck")
    }
    fn ctrl_rig_bone_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_1")
    }
    fn ctrl_rig_bone_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_2")
    }
    fn ctrl_rig_bone_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_3")
    }
    fn ctrl_rig_bone_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_thumb_1_left")
    }
    fn ctrl_rig_bone_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_thumb_1_right")
    }
    fn ctrl_rig_bone_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_thumb_2_left")
    }
    fn ctrl_rig_bone_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_thumb_2_right")
    }
    fn ctrl_rig_bone_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_thumb_3_left")
    }
    fn ctrl_rig_bone_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_thumb_3_right")
    }
    fn ctrl_rig_bone_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_upperarm_left")
    }
    fn ctrl_rig_bone_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_upperarm_right")
    }
    fn ctrl_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_toetip_right")
    }
    fn fetch_rig_customspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_customspace")
    }
    fn fetch_rig_eyes_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_eyes_custom_space")
    }
    fn fetch_rig_foot_customspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_foot_customspace_left")
    }
    fn fetch_rig_foot_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_foot_customspace_right")
    }
    fn fetch_rig_localspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_localspace")
    }
    fn fetch_rig_wrist_custom_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_wrist_custom_space_left")
    }
    fn fetch_rig_wrist_custom_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_wrist_custom_space_right")
    }
    fn geo_calves_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Calves_left")
    }
    fn geo_calves_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Calves_right")
    }
    fn geo_chest(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Chest")
    }
    fn geo_elbowjoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ElbowJoints_left")
    }
    fn geo_elbowjoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ElbowJoints_right")
    }
    fn geo_eyelids_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_EyeLids_left")
    }
    fn geo_eyelids_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_EyeLids_right")
    }
    fn geo_eyes_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Eyes_left")
    }
    fn geo_eyes_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Eyes_right")
    }
    fn geo_fingersbase_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_FingersBase_left")
    }
    fn geo_fingersbase_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_FingersBase_right")
    }
    fn geo_fingersmid_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_FingersMid_left")
    }
    fn geo_fingersmid_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_FingersMid_right")
    }
    fn geo_fingerstips_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_FingersTips_left")
    }
    fn geo_fingerstips_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_FingersTips_right")
    }
    fn geo_forearms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Forearms_left")
    }
    fn geo_forearms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Forearms_right")
    }
    fn geo_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Hand_left")
    }
    fn geo_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Hand_right")
    }
    fn geo_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Head")
    }
    fn geo_heels_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Heels_left")
    }
    fn geo_heels_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Heels_right")
    }
    fn geo_hipjoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_HipJoints_left")
    }
    fn geo_hipjoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_HipJoints_right")
    }
    fn geo_kneejoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_KneeJoints_left")
    }
    fn geo_kneejoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_KneeJoints_right")
    }
    fn geo_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Neck")
    }
    fn geo_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Pelvis")
    }
    fn geo_ribs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Ribs")
    }
    fn geo_shoulders_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Shoulders_left")
    }
    fn geo_shoulders_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Shoulders_right")
    }
    fn geo_stomache(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Stomache")
    }
    fn geo_thighs_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Thighs_left")
    }
    fn geo_thighs_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Thighs_right")
    }
    fn geo_thumbbase_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ThumbBase_left")
    }
    fn geo_thumbbase_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ThumbBase_right")
    }
    fn geo_thumbmid_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ThumbMid_left")
    }
    fn geo_thumbmid_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ThumbMid_right")
    }
    fn geo_thumbtip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ThumbTip_left")
    }
    fn geo_thumbtip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_ThumbTip_right")
    }
    fn geo_toes_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Toes_left")
    }
    fn geo_toes_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_Toes_right")
    }
    fn geo_upperarms_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_UpperArms_left")
    }
    fn geo_upperarms_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_UpperArms_right")
    }
    fn geo_wristjoints_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_WristJoints_left")
    }
    fn geo_wristjoints_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_WristJoints_right")
    }
    fn geoexport(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geoexport")
    }
    fn local_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master")
    }
    fn local_master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master_space")
    }
    fn local_master_space1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master_space1")
    }
    fn master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("master_space")
    }
    fn rig_fkik_arm_left_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FKIK_arm_left_blend")
    }
    fn rig_fkik_arm_right_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FKIK_arm_right_blend")
    }
    fn rig_fk_arm_left_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FK_arm_left_tip")
    }
    fn rig_fk_arm_left_tip1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FK_arm_left_tip1_right")
    }
    fn rig_ik_foot_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_foot_left_pretransform")
    }
    fn rig_ik_foot_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_foot_right_pretransform")
    }
    fn rig_ik_wrist_left_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_left_orientation")
    }
    fn rig_ik_wrist_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_left_pretransform")
    }
    fn rig_ik_wrist_right_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_right_orientation")
    }
    fn rig_ik_wrist_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_right_pretransform")
    }
    fn rig_blend_fk_ik_leg_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_leg_left")
    }
    fn rig_blend_fk_ik_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_leg_right")
    }
    fn rig_blend_fk_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_toe_left")
    }
    fn rig_blend_fk_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_toe_right")
    }
    fn rig_blend_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_custom_space")
    }
    fn rig_blend_customspace_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_customspace_foot_left")
    }
    fn rig_blend_customspace_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_customspace_foot_right")
    }
    fn rig_blend_eyes_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_eyes_space")
    }
    fn rig_blend_leg_twistspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_leg_twistspace_left")
    }
    fn rig_blend_leg_twistspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_leg_twistspace_right")
    }
    fn rig_blend_wrist_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_wrist_space_left")
    }
    fn rig_blend_wrist_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_wrist_space_right")
    }
    fn rig_bone_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_calf_left")
    }
    fn rig_bone_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_calf_right")
    }
    fn rig_bone_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_clavicle_left")
    }
    fn rig_bone_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_clavicle_right")
    }
    fn rig_bone_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_foot_left")
    }
    fn rig_bone_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_foot_right")
    }
    fn rig_bone_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_head")
    }
    fn rig_bone_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_hip_left")
    }
    fn rig_bone_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_hip_right")
    }
    fn rig_bone_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_pelvis")
    }
    fn rig_bone_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thigh_left")
    }
    fn rig_bone_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thigh_right")
    }
    fn rig_bone_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_toe_left")
    }
    fn rig_bone_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_toe_right")
    }
    fn rig_calf_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_calf_tip_left")
    }
    fn rig_calf_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_calf_tip_right")
    }
    fn rig_cog_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_cog_pretransform")
    }
    fn rig_cog_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_cog_pretransform1")
    }
    fn rig_eye_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_eye_left_pretransform")
    }
    fn rig_eye_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_eye_right_pretransform")
    }
    fn rig_eyes_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_eyes_pretransform")
    }
    fn rig_foot_ankle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_ankle_left")
    }
    fn rig_foot_ankle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_ankle_right")
    }
    fn rig_foot_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_tip_left")
    }
    fn rig_foot_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_tip_right")
    }
    fn rig_footleft_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_footleft_pretransform")
    }
    fn rig_footleft_pretransform1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_footleft_pretransform1_right")
    }
    fn rig_head_space_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_head_space_blend")
    }
    fn rig_head_worldspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_head_worldspace")
    }
    fn rig_hip_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_hip_pretransform")
    }
    fn rig_hip_pretransform1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_hip_pretransform1_right")
    }
    fn rig_master_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform")
    }
    fn rig_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform1")
    }
    fn rig_master_pretransform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform2")
    }
    fn rig_mirror_arm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_arm_right")
    }
    fn rig_mirror_custom_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_custom_right")
    }
    fn rig_mirror_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_foot_right")
    }
    fn rig_mirror_foot_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_foot_twist_right")
    }
    fn rig_mirror_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_leg_right")
    }
    fn rig_spine_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_spine_tip")
    }
    fn rig_wrist_space_cog_world(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_space_cog_world")
    }
    fn rig_wrist_space_cog_world1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_space_cog_world1")
    }
    fn rig_wrist_spaceblend_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_spaceblend_left")
    }
    fn rig_wrist_spaceblend_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_spaceblend_right")
    }
    fn righ_wrist_worldspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("righ_wrist_worldspace_left")
    }
    fn righ_wrist_worldspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("righ_wrist_worldspace_right")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemalePlayback {
    Deform = 0,
    CachedDeform = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleDisplayTop {
    None = 0,
    TankTop = 1,
    /// T-Shirt
    TMinusShirt = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleTShirtColor {
    Blue = 0,
    Red = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleDisplayBottom {
    None = 0,
    Leggings = 1,
    Pants = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleGeotype {
    Polygon = 0,
    Polysoup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemalePreXform {
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
pub enum ObjectSimplefemaleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplefemaleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectSimplefemale {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectSimplefemale {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_parent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_masteruniformscale(mut self, val: f32) -> Self {
        self.params.insert(
            "masteruniformscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_masteruniformscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masteruniformscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlmasterspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlmasterspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlmasterspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlmasterspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrllocalspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrllocalspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrllocalspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlheadspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlheadspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlheadspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlheadspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "eyelookatcustomspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyelookatcustomspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikleftarm(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikleftarm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikleftarm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikleftarm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristleftspace(mut self, val: f32) -> Self {
        self.params.insert(
            "ikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikwristleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikwristleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikwristleftspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikrightarm(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikrightarm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikrightarm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikrightarm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrightspace(mut self, val: f32) -> Self {
        self.params.insert(
            "ikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikwristrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikwristrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikwristrightspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftthumb2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftthumb3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftindex2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftindex2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftindex3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftindex3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftring2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftring2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftring3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftring3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftlittle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftlittle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftlittle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftlittle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightthumb2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightthumb3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightindex2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightindex2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightindex3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightindex3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightring2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightring2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightring3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightring3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightlittle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightlittle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightlittle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightlittle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikleftleg(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikleftleg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikleftleg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikleftleg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleglefttwistblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ikleglefttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikleglefttwistblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleglefttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_foot_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "l_foot_roll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_foot_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_foot_roll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_roll_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "l_roll_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_roll_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_roll_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_heel_roll_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "l_heel_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_heel_roll_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_heel_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_foot_roll_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "l_foot_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_foot_roll_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_foot_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_toe_roll_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "l_toe_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_toe_roll_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_toe_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikfootleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikfootleftspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikrightleg(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikrightleg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikrightleg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikrightleg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iklegrighttwistblend(mut self, val: f32) -> Self {
        self.params.insert(
            "iklegrighttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iklegrighttwistblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iklegrighttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_foot_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "r_foot_roll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_foot_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_foot_roll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_roll_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "r_roll_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_roll_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_roll_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_heel_roll_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "r_heel_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_heel_roll_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_heel_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_foot_roll_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "r_foot_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_foot_roll_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_foot_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_toe_roll_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "r_toe_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_toe_roll_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_toe_roll_angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikfootrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikfootrightspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_eye_open(mut self, val: f32) -> Self {
        self.params.insert(
            "l_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_eye_open_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_eye_open(mut self, val: f32) -> Self {
        self.params.insert(
            "r_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_eye_open_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_jaw(mut self, val: f32) -> Self {
        self.params.insert(
            "jaw".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jaw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jaw".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_curl(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_curl".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_curl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_curl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_curl_fade(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_curl_fade".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_curl_fade_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_curl_fade".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_twist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_twist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_extend(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_extend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_extend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_extend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aa(mut self, val: f32) -> Self {
        self.params.insert(
            "AA".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aa_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "AA".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ee(mut self, val: f32) -> Self {
        self.params.insert(
            "EE".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ee_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "EE".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_oh(mut self, val: f32) -> Self {
        self.params.insert(
            "OH".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "OH".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uu(mut self, val: f32) -> Self {
        self.params.insert(
            "UU".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "UU".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cee(mut self, val: f32) -> Self {
        self.params.insert(
            "CEE".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cee_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "CEE".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eff(mut self, val: f32) -> Self {
        self.params.insert(
            "EFF".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "EFF".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mm(mut self, val: f32) -> Self {
        self.params.insert(
            "MM".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "MM".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_qw(mut self, val: f32) -> Self {
        self.params.insert(
            "QW".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_qw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "QW".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_brow_up(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_up".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_brow_down(mut self, val: f32) -> Self {
        self.params.insert(
            "r_brow_down".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_brow_down_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_brow_down".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_brow_down(mut self, val: f32) -> Self {
        self.params.insert(
            "l_brow_down".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_brow_down_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_brow_down".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_smile(mut self, val: f32) -> Self {
        self.params.insert(
            "smile".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_frown(mut self, val: f32) -> Self {
        self.params.insert(
            "frown".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frown_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frown".to_string(),
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
    pub fn with_mastertranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mastertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mastertranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mastertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_masterrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "masterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_masterrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localmastertraslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localmastertraslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localmastertraslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localmastertraslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localmasterrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localmasterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localmasterrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localmasterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cogtranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cogtranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cogtranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cogtranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cogrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cogrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cogrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cogrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipstranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hipstranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hipstranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipstranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipsrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hipsrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hipsrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipsrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate1".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_neckrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "neckrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_neckrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "neckrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "headrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_headrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeslookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeslookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeslookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeslookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeleftlookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeleftlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeleftlookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeleftlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyerightlookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyerightlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyerightlookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyerightlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shoulderlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shoulderlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shoulderlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shoulderlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleftarmtwist(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikleftarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikleftarmtwist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleftarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperarmleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperarmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperarmleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperarmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forearmleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "forearmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_forearmleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forearmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightshouldertranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightshouldertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightshouldertranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightshouldertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristrighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikrightarmtwist(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikrightarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikrightarmtwist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikrightarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperarmrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperarmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperarmrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperarmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forearmrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "forearmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_forearmrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forearmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftthumb1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftindex1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftindex1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftmiddle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftring1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftring1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftlittle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftlittle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightthumb1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightindex1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightindex1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightmiddle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightring1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightring1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightlittle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightlittle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lefthiptranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lefthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lefthiptranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lefthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiplefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiplefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiplefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiplefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetipleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetipleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetipleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetipleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootballleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootballleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootballleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootballleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoeleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoeleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleglefttwisttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikleglefttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikleglefttwisttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleglefttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkthighleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkthighleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkthighleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkthighleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkcalfleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkcalfleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkcalfleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkcalfleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkfootleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkfootleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fktoeleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fktoeleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_righthiptranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "righthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_righthiptranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "righthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootrighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootrighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiprighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiprighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiprighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiprighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiprightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiprightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiprightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiprightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootballrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootballrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootballrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootballrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoerightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoerightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iklegrighttwisttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iklegrighttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iklegrighttwisttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iklegrighttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkthighrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkthighrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkthighrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkthighrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkcalfrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkcalfrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkcalfrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkcalfrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkfootrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkfootrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fktoerightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fktoerightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_jaw_ik(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "jaw_IK".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_jaw_ik_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jaw_IK".to_string(),
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
    pub fn with_playback(mut self, val: ObjectSimplefemalePlayback) -> Self {
        self.params.insert(
            "playback".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_playback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "playback".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_top(mut self, val: ObjectSimplefemaleDisplayTop) -> Self {
        self.params.insert(
            "display_top".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_display_top_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_top".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t_shirt_color(mut self, val: ObjectSimplefemaleTShirtColor) -> Self {
        self.params.insert(
            "t_shirt_color".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_t_shirt_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t_shirt_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_bottom(mut self, val: ObjectSimplefemaleDisplayBottom) -> Self {
        self.params.insert(
            "display_bottom".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_display_bottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_bottom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geotype(mut self, val: ObjectSimplefemaleGeotype) -> Self {
        self.params.insert(
            "geoType".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_geotype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geoType".to_string(),
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
    pub fn with_viewportlod(mut self, val: ObjectSimplefemaleViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectSimplefemaleVportOnionskin) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectSimplefemalePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectSimplefemaleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectSimplefemaleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectSimplefemaleUparmtype) -> Self {
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
    pub fn with_ctrlmasterspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlmasterspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlmasterspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlmasterspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrllocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrllocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspace(mut self, val: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
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
    pub fn with_enablesns(mut self, val: bool) -> Self {
        self.params.insert(
            "enableSnS".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableSnS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showbones(mut self, val: bool) -> Self {
        self.params.insert(
            "showbones".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showbones_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showbones".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showcontrols(mut self, val: bool) -> Self {
        self.params.insert(
            "showcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showcontrols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showdetailcontrols(mut self, val: bool) -> Self {
        self.params.insert(
            "showdetailcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showdetailcontrols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showdetailcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "showgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedeletamush(mut self, val: bool) -> Self {
        self.params.insert(
            "useDeletaMush".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedeletamush_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useDeletaMush".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ObjectSimplefemale {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "simplefemale"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectSimplefemaleOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectSimplefemaleOutputs for ObjectSimplefemale {}
impl ObjectSimplefemaleOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSimplefemale>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectSimplefemaleInnerExt {
    fn fkik_arm_right_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fk_arm_right_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fk_hand_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_chops(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_arm_twist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_wrist_left_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_wrist_right_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_wrist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lower_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn upper_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn auto_foot_roll_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn auto_foot_roll_left1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn auto_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn auto_football_left1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn auto_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn auto_toetip_left1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend_wrist_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bone_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bone_clavicle_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cog_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_football_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_leg_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_leg_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_blend_fk_ik_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_blend_fk_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_calf_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_cog(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eye_lookat_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eye_lookat_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyes_lookat(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyes_lookat_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_foot_ankle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_foot_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hips_rotate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hips_translate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_jaw_ik(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_customspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_eyes_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_foot_customspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_foot_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_localspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_wrist_custom_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_wrist_custom_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_skin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn hair(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn hair1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master_space1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn master_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn materials(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mirror_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fkik_arm_left_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fk_arm_left_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_foot_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_foot_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_left_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_leg_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_customspace_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_customspace_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_eyes_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_leg_twistspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_leg_twistspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_wrist_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_clavicle_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_calf_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_cog_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_ankle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_footleft_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_footright_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_head_space_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_head_worldspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_hip_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_hip_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_spine_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_space_cog_world1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_spaceblend_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn righ_wrist_worldspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_root1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wrist_space_cog_world_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wrist_spaceblend_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wrist_worldspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectSimplefemaleInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectSimplefemale>
{
    fn fkik_arm_right_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("FKIK_arm_right_blend")
    }
    fn fk_arm_right_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("FK_arm_right_tip")
    }
    fn fk_hand_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("FK_hand_right_ptr")
    }
    fn ik_chops(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_CHOPS")
    }
    fn ik_arm_twist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_arm_twist_right_ptr")
    }
    fn ik_wrist_left_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_wrist_left_pretransform_right")
    }
    fn ik_wrist_right_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_wrist_right_orientation")
    }
    fn ik_wrist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_wrist_right_ptr")
    }
    fn l_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye")
    }
    fn l_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_bone")
    }
    fn l_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_1")
    }
    fn l_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_2")
    }
    fn l_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_3")
    }
    fn l_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_4")
    }
    fn l_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_null")
    }
    fn l_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_offset")
    }
    fn l_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_1")
    }
    fn l_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_2")
    }
    fn l_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_3")
    }
    fn l_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_4")
    }
    fn l_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone1")
    }
    fn l_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone2")
    }
    fn l_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone3")
    }
    fn l_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone4")
    }
    fn l_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone5")
    }
    fn l_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone6")
    }
    fn l_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone7")
    }
    fn l_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone8")
    }
    fn l_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone9")
    }
    fn lower_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Lower_Teeth_Gums")
    }
    fn r_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye")
    }
    fn r_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_bone")
    }
    fn r_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_1")
    }
    fn r_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_2")
    }
    fn r_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_3")
    }
    fn r_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_4")
    }
    fn r_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_null")
    }
    fn r_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_offset")
    }
    fn r_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_1")
    }
    fn r_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_2")
    }
    fn r_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_3")
    }
    fn r_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_4")
    }
    fn r_null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null1")
    }
    fn r_null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null2")
    }
    fn r_null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null3")
    }
    fn r_null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null4")
    }
    fn r_null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null5")
    }
    fn r_null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null6")
    }
    fn r_null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null7")
    }
    fn r_null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null8")
    }
    fn r_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone1")
    }
    fn r_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone2")
    }
    fn r_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone3")
    }
    fn r_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone4")
    }
    fn r_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone5")
    }
    fn r_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone6")
    }
    fn r_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone7")
    }
    fn r_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone8")
    }
    fn r_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone9")
    }
    fn upper_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Upper_Teeth_Gums")
    }
    fn auto_foot_roll_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("auto_foot_roll_left")
    }
    fn auto_foot_roll_left1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("auto_foot_roll_left1")
    }
    fn auto_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("auto_football_left")
    }
    fn auto_football_left1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("auto_football_left1")
    }
    fn auto_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("auto_toetip_left")
    }
    fn auto_toetip_left1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("auto_toetip_left1")
    }
    fn blend1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend1")
    }
    fn blend_wrist_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend_wrist_space_right")
    }
    fn bone_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bone_clavicle_right")
    }
    fn bone_clavicle_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bone_clavicle_right_ptr")
    }
    fn cog_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cog_pretransform_right")
    }
    fn ctrl_fk_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_left")
    }
    fn ctrl_fk_calf_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_left_ptr")
    }
    fn ctrl_fk_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_right")
    }
    fn ctrl_fk_calf_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_right_ptr")
    }
    fn ctrl_fk_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_left")
    }
    fn ctrl_fk_foot_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_left_preT")
    }
    fn ctrl_fk_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_right")
    }
    fn ctrl_fk_foot_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_right_preT")
    }
    fn ctrl_fk_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_left")
    }
    fn ctrl_fk_forearm_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_left_ptr")
    }
    fn ctrl_fk_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_right")
    }
    fn ctrl_fk_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_left")
    }
    fn ctrl_fk_hand_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_left_ptr")
    }
    fn ctrl_fk_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_right")
    }
    fn ctrl_fk_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_1_left")
    }
    fn ctrl_fk_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_1_right")
    }
    fn ctrl_fk_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_2_left")
    }
    fn ctrl_fk_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_2_right")
    }
    fn ctrl_fk_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_3_left")
    }
    fn ctrl_fk_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_3_right")
    }
    fn ctrl_fk_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_1_left")
    }
    fn ctrl_fk_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_1_right")
    }
    fn ctrl_fk_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_2_left")
    }
    fn ctrl_fk_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_2_right")
    }
    fn ctrl_fk_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_3_left")
    }
    fn ctrl_fk_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_3_right")
    }
    fn ctrl_fk_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_1_left")
    }
    fn ctrl_fk_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_1_right")
    }
    fn ctrl_fk_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_2_left")
    }
    fn ctrl_fk_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_2_right")
    }
    fn ctrl_fk_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_3_left")
    }
    fn ctrl_fk_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_3_right")
    }
    fn ctrl_fk_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_1_left")
    }
    fn ctrl_fk_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_1_right")
    }
    fn ctrl_fk_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_2_left")
    }
    fn ctrl_fk_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_2_right")
    }
    fn ctrl_fk_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_3_left")
    }
    fn ctrl_fk_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_3_right")
    }
    fn ctrl_fk_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_1")
    }
    fn ctrl_fk_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_2")
    }
    fn ctrl_fk_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_3")
    }
    fn ctrl_fk_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thigh_left")
    }
    fn ctrl_fk_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thigh_right")
    }
    fn ctrl_fk_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_1_left")
    }
    fn ctrl_fk_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_1_right")
    }
    fn ctrl_fk_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_2_left")
    }
    fn ctrl_fk_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_2_right")
    }
    fn ctrl_fk_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_3_left")
    }
    fn ctrl_fk_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_3_right")
    }
    fn ctrl_fk_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_left")
    }
    fn ctrl_fk_toe_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_left_ptr")
    }
    fn ctrl_fk_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_right")
    }
    fn ctrl_fk_toe_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_right_ptr")
    }
    fn ctrl_fk_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_upperarm_left")
    }
    fn ctrl_fk_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_upperarm_right")
    }
    fn ctrl_ik_arm_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_left")
    }
    fn ctrl_ik_arm_twist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_left_ptr")
    }
    fn ctrl_ik_arm_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_right")
    }
    fn ctrl_ik_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_foot_left")
    }
    fn ctrl_ik_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_foot_right")
    }
    fn ctrl_ik_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_football_left")
    }
    fn ctrl_ik_football_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_football_right")
    }
    fn ctrl_ik_leg_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_leg_twist_left")
    }
    fn ctrl_ik_leg_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_leg_twist_right")
    }
    fn ctrl_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toe_left")
    }
    fn ctrl_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toe_right")
    }
    fn ctrl_ik_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toetip_left")
    }
    fn ctrl_ik_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toetip_right")
    }
    fn ctrl_ik_wrist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_left")
    }
    fn ctrl_ik_wrist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_left_ptr")
    }
    fn ctrl_ik_wrist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_right")
    }
    fn ctrl_blend_fk_ik_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_blend_FK_IK_leg_right")
    }
    fn ctrl_blend_fk_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_blend_FK_IK_toe_right")
    }
    fn ctrl_calf_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_calf_tip_right")
    }
    fn ctrl_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_clavicle_left")
    }
    fn ctrl_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_clavicle_right")
    }
    fn ctrl_cog(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_cog")
    }
    fn ctrl_eye_lookat_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eye_lookat_left")
    }
    fn ctrl_eye_lookat_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eye_lookat_right")
    }
    fn ctrl_eyes_lookat(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyes_lookat")
    }
    fn ctrl_eyes_lookat_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyes_lookat_ptr")
    }
    fn ctrl_foot_ankle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_foot_ankle_right")
    }
    fn ctrl_foot_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_foot_tip_right")
    }
    fn ctrl_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_head")
    }
    fn ctrl_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_left")
    }
    fn ctrl_hip_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_left_ptr")
    }
    fn ctrl_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_right")
    }
    fn ctrl_hip_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_right_ptr")
    }
    fn ctrl_hips_rotate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hips_rotate")
    }
    fn ctrl_hips_translate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hips_translate")
    }
    fn ctrl_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_jaw")
    }
    fn ctrl_jaw_ik(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_jaw_IK")
    }
    fn ctrl_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_master")
    }
    fn ctrl_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_neck")
    }
    fn ctrl_rig_bone_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_forearm_left")
    }
    fn ctrl_rig_bone_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_hand_left")
    }
    fn ctrl_rig_bone_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_hand_right")
    }
    fn ctrl_rig_bone_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_neck")
    }
    fn ctrl_rig_bone_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_1")
    }
    fn ctrl_rig_bone_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_2")
    }
    fn ctrl_rig_bone_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_3")
    }
    fn ctrl_rig_bone_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_upperarm_left")
    }
    fn eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("eye_offset")
    }
    fn fetch_rig_customspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_customspace")
    }
    fn fetch_rig_eyes_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_eyes_custom_space")
    }
    fn fetch_rig_foot_customspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_foot_customspace_left")
    }
    fn fetch_rig_foot_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_foot_customspace_right")
    }
    fn fetch_rig_localspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_localspace")
    }
    fn fetch_rig_wrist_custom_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_wrist_custom_space_left")
    }
    fn fetch_rig_wrist_custom_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_wrist_custom_space_right")
    }
    fn geo_skin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_skin")
    }
    fn hair(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("hair")
    }
    fn hair1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("hair1")
    }
    fn local_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master")
    }
    fn local_master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master_space")
    }
    fn local_master_space1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master_space1")
    }
    fn master_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("master_pretransform_right")
    }
    fn master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("master_space")
    }
    fn materials(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materials")
    }
    fn mirror_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mirror_master_pretransform1")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn null10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null10")
    }
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null2")
    }
    fn null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null3")
    }
    fn null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null4")
    }
    fn null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null5")
    }
    fn null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null6")
    }
    fn null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null7")
    }
    fn null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null8")
    }
    fn null9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null9")
    }
    fn rig_fkik_arm_left_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FKIK_arm_left_blend")
    }
    fn rig_fk_arm_left_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FK_arm_left_tip")
    }
    fn rig_ik_foot_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_foot_left_pretransform")
    }
    fn rig_ik_foot_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_foot_right_pretransform")
    }
    fn rig_ik_wrist_left_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_left_orientation")
    }
    fn rig_ik_wrist_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_left_pretransform")
    }
    fn rig_blend_fk_ik_leg_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_leg_left")
    }
    fn rig_blend_fk_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_toe_left")
    }
    fn rig_blend_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_custom_space")
    }
    fn rig_blend_customspace_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_customspace_foot_left")
    }
    fn rig_blend_customspace_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_customspace_foot_right")
    }
    fn rig_blend_eyes_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_eyes_space")
    }
    fn rig_blend_leg_twistspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_leg_twistspace_left")
    }
    fn rig_blend_leg_twistspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_leg_twistspace_right")
    }
    fn rig_blend_wrist_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_wrist_space_left")
    }
    fn rig_bone_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_calf_left")
    }
    fn rig_bone_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_calf_right")
    }
    fn rig_bone_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_clavicle_left")
    }
    fn rig_bone_clavicle_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_clavicle_left_ptr")
    }
    fn rig_bone_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_foot_left")
    }
    fn rig_bone_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_foot_right")
    }
    fn rig_bone_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_forearm_right")
    }
    fn rig_bone_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_head")
    }
    fn rig_bone_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_hip_left")
    }
    fn rig_bone_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_hip_right")
    }
    fn rig_bone_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_1_left")
    }
    fn rig_bone_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_1_right")
    }
    fn rig_bone_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_2_left")
    }
    fn rig_bone_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_2_right")
    }
    fn rig_bone_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_3_left")
    }
    fn rig_bone_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_3_right")
    }
    fn rig_bone_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_jaw")
    }
    fn rig_bone_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_1_left")
    }
    fn rig_bone_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_1_right")
    }
    fn rig_bone_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_2_left")
    }
    fn rig_bone_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_2_right")
    }
    fn rig_bone_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_3_left")
    }
    fn rig_bone_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_3_right")
    }
    fn rig_bone_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_1_left")
    }
    fn rig_bone_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_1_right")
    }
    fn rig_bone_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_2_left")
    }
    fn rig_bone_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_2_right")
    }
    fn rig_bone_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_3_left")
    }
    fn rig_bone_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_3_right")
    }
    fn rig_bone_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_pelvis")
    }
    fn rig_bone_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_1_left")
    }
    fn rig_bone_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_1_right")
    }
    fn rig_bone_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_2_left")
    }
    fn rig_bone_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_2_right")
    }
    fn rig_bone_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_3_left")
    }
    fn rig_bone_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_3_right")
    }
    fn rig_bone_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thigh_left")
    }
    fn rig_bone_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thigh_right")
    }
    fn rig_bone_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_1_left")
    }
    fn rig_bone_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_1_right")
    }
    fn rig_bone_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_2_left")
    }
    fn rig_bone_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_2_right")
    }
    fn rig_bone_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_3_left")
    }
    fn rig_bone_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_3_right")
    }
    fn rig_bone_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_toe_left")
    }
    fn rig_bone_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_toe_right")
    }
    fn rig_bone_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_upperarm_right")
    }
    fn rig_calf_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_calf_tip_left")
    }
    fn rig_cog_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_cog_pretransform1")
    }
    fn rig_foot_ankle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_ankle_left")
    }
    fn rig_foot_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_tip_left")
    }
    fn rig_footleft_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_footleft_pretransform")
    }
    fn rig_footright_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_footright_pretransform")
    }
    fn rig_head_space_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_head_space_blend")
    }
    fn rig_head_worldspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_head_worldspace")
    }
    fn rig_hip_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_hip_pretransform")
    }
    fn rig_hip_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_hip_pretransform_right")
    }
    fn rig_master_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform")
    }
    fn rig_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform1")
    }
    fn rig_mirror_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_leg_right")
    }
    fn rig_spine_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_spine_tip")
    }
    fn rig_wrist_space_cog_world1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_space_cog_world1")
    }
    fn rig_wrist_spaceblend_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_spaceblend_left")
    }
    fn righ_wrist_worldspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("righ_wrist_worldspace_left")
    }
    fn tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone1")
    }
    fn tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone2")
    }
    fn tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone3")
    }
    fn tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone4")
    }
    fn tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone5")
    }
    fn tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone6")
    }
    fn tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone7")
    }
    fn tongue_root1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_root1")
    }
    fn wrist_space_cog_world_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wrist_space_cog_world_right")
    }
    fn wrist_spaceblend_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wrist_spaceblend_right")
    }
    fn wrist_worldspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wrist_worldspace_right")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemalePlayback {
    Deform = 0,
    CachedDeform = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
    /// Subdivision Surface / Curves
    SubdivisionSurfaceCurves = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleDisplayTop {
    None = 0,
    TankTop = 1,
    /// T-Shirt
    TMinusShirt = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleDisplayBottom {
    None = 0,
    Shorts = 1,
    Pants = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleGeotype {
    Polygon = 0,
    Polysoup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleVportOnionskin {
    Off = 0,
    TransformOnly = 1,
    FullDeformation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemalePreXform {
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
pub enum ObjectSimplemaleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSimplemaleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectSimplemale {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectSimplemale {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_parent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn trigger_set_to_default(mut self) -> Self {
        self.params.insert(
            "set_to_default".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_set_initial_channels(mut self) -> Self {
        self.params.insert(
            "set_initial_channels".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_masteruniformscale(mut self, val: f32) -> Self {
        self.params.insert(
            "masteruniformscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_masteruniformscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masteruniformscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlmasterspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlmasterspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlmasterspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlmasterspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrllocalspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrllocalspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrllocalspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlheadspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlheadspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlheadspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlheadspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "eyelookatcustomspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyelookatcustomspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikleftarm(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikleftarm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikleftarm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikleftarm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristleftspace(mut self, val: f32) -> Self {
        self.params.insert(
            "ikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikwristleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikwristleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikwristleftspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikrightarm(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikrightarm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikrightarm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikrightarm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrightspace(mut self, val: f32) -> Self {
        self.params.insert(
            "ikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikwristrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikwristrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikwristrightspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftthumb2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftthumb3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftindex2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftindex2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftindex3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftindex3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftmiddle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftring2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftring2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftring3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftring3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftlittle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftlittle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftlittle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "leftlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_leftlittle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightthumb2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightthumb3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightindex2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightindex2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightindex2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightindex3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightindex3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightindex3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightmiddle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightring2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightring2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightring2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightring3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightring3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightring3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightlittle2rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightlittle2rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightlittle2rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightlittle3rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rightlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rightlittle3rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightlittle3rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikleftleg(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikleftleg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikleftleg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikleftleg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleglefttwistblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ikleglefttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ikleglefttwistblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleglefttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikfootleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikfootleftspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkikrightleg(mut self, val: f32) -> Self {
        self.params.insert(
            "fkikrightleg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fkikrightleg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkikrightleg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iklegrighttwistblend(mut self, val: f32) -> Self {
        self.params.insert(
            "iklegrighttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iklegrighttwistblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iklegrighttwistblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspaceblend(mut self, val: f32) -> Self {
        self.params.insert(
            "ctrlikfootrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ctrlikfootrightspaceblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspaceblend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_l_eye_open(mut self, val: f32) -> Self {
        self.params.insert(
            "l_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_l_eye_open_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "l_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_eye_open(mut self, val: f32) -> Self {
        self.params.insert(
            "r_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_eye_open_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_eye_open".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_jaw(mut self, val: f32) -> Self {
        self.params.insert(
            "jaw".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jaw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jaw".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_curl(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_curl".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_curl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_curl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_curl_fade(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_curl_fade".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_curl_fade_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_curl_fade".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_twist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_twist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tongue_extend(mut self, val: f32) -> Self {
        self.params.insert(
            "tongue_extend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tongue_extend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tongue_extend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend1(mut self, val: f32) -> Self {
        self.params.insert(
            "blend1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend2(mut self, val: f32) -> Self {
        self.params.insert(
            "blend2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend3(mut self, val: f32) -> Self {
        self.params.insert(
            "blend3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend4(mut self, val: f32) -> Self {
        self.params.insert(
            "blend4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend5(mut self, val: f32) -> Self {
        self.params.insert(
            "blend5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend6(mut self, val: f32) -> Self {
        self.params.insert(
            "blend6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend7(mut self, val: f32) -> Self {
        self.params.insert(
            "blend7".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend8(mut self, val: f32) -> Self {
        self.params.insert(
            "blend8".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend9(mut self, val: f32) -> Self {
        self.params.insert(
            "blend9".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend9".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend10(mut self, val: f32) -> Self {
        self.params.insert(
            "blend10".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend10".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend11(mut self, val: f32) -> Self {
        self.params.insert(
            "blend11".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend11".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend12(mut self, val: f32) -> Self {
        self.params.insert(
            "blend12".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend12".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend13(mut self, val: f32) -> Self {
        self.params.insert(
            "blend13".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend13".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend14(mut self, val: f32) -> Self {
        self.params.insert(
            "blend14".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend14".to_string(),
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
    pub fn with_mastertranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mastertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mastertranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mastertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_masterrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "masterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_masterrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localmastertraslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localmastertraslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localmastertraslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localmastertraslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localmasterrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "localmasterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_localmasterrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localmasterrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cogtranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cogtranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cogtranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cogtranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cogrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cogrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cogrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cogrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipstranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hipstranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hipstranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipstranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipsrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hipsrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hipsrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipsrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate1".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spinerotate3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spinerotate3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spinerotate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spinerotate3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_neckrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "neckrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_neckrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "neckrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "headrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_headrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeslookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeslookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeslookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeslookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeleftlookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyeleftlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyeleftlookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeleftlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyerightlookattranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eyerightlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eyerightlookattranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyerightlookattranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shoulderlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shoulderlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shoulderlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shoulderlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleftarmtwist(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikleftarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikleftarmtwist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleftarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperarmleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperarmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperarmleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperarmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forearmleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "forearmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_forearmleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forearmleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightshouldertranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightshouldertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightshouldertranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightshouldertranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristrighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikwristrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikwristrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikwristrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikwristrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikrightarmtwist(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikrightarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikrightarmtwist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikrightarmtwist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upperarmrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upperarmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upperarmrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upperarmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forearmrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "forearmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_forearmrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forearmrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftthumb1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftthumb1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftindex1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftindex1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftmiddle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftmiddle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftring1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftring1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftlittle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "leftlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_leftlittle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightthumb1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightthumb1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightthumb1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightindex1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightindex1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightindex1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightmiddle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightmiddle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightmiddle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightring1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightring1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightring1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightlittle1rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rightlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rightlittle1rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightlittle1rotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lefthiptranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lefthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lefthiptranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lefthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootlefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootlefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootlefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiplefttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiplefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiplefttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiplefttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetipleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetipleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetipleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetipleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootballleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootballleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootballleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootballleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoeleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoeleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikleglefttwisttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikleglefttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikleglefttwisttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikleglefttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkthighleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkthighleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkthighleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkthighleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkcalfleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkcalfleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkcalfleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkcalfleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkfootleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkfootleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkfootleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fktoeleftrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fktoeleftrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fktoeleftrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_righthiptranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "righthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_righthiptranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "righthiptranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootrighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootrighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootrighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiprighttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiprighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiprighttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiprighttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoetiprightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoetiprightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoetiprightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoetiprightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ikfootballrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ikfootballrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ikfootballrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ikfootballrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iktoerightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iktoerightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iklegrighttwisttranslate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "iklegrighttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iklegrighttwisttranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iklegrighttwisttranslate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkthighrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkthighrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkthighrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkthighrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkcalfrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkcalfrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkcalfrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkcalfrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fkfootrightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fkfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fkfootrightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fkfootrightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fktoerightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fktoerightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fktoerightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_jaw_ik(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "jaw_IK".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_jaw_ik_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jaw_IK".to_string(),
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
    pub fn with_playback(mut self, val: ObjectSimplemalePlayback) -> Self {
        self.params.insert(
            "playback".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_playback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "playback".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_top(mut self, val: ObjectSimplemaleDisplayTop) -> Self {
        self.params.insert(
            "display_top".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_display_top_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_top".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display_bottom(mut self, val: ObjectSimplemaleDisplayBottom) -> Self {
        self.params.insert(
            "display_bottom".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_display_bottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display_bottom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geotype(mut self, val: ObjectSimplemaleGeotype) -> Self {
        self.params.insert(
            "geoType".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_geotype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geoType".to_string(),
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
    pub fn with_viewportlod(mut self, val: ObjectSimplemaleViewportlod) -> Self {
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
    pub fn with_vport_onionskin(mut self, val: ObjectSimplemaleVportOnionskin) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectSimplemalePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectSimplemaleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectSimplemaleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectSimplemaleUparmtype) -> Self {
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
    pub fn with_ctrlmasterspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlmasterspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlmasterspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlmasterspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrllocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrllocalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrllocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspace(mut self, val: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyelookatcustomspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyelookatcustomspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikwristrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikwristrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootleftspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootleftspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspace(mut self, val: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ctrlikfootrightspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ctrlikfootrightspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_subdstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_subdstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_subdstyle".to_string(),
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
    pub fn with_enablesns(mut self, val: bool) -> Self {
        self.params.insert(
            "enableSnS".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableSnS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showbones(mut self, val: bool) -> Self {
        self.params.insert(
            "showbones".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showbones_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showbones".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showcontrols(mut self, val: bool) -> Self {
        self.params.insert(
            "showcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showcontrols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showdetailcontrols(mut self, val: bool) -> Self {
        self.params.insert(
            "showdetailcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showdetailcontrols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showdetailcontrols".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "showgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedeletamush(mut self, val: bool) -> Self {
        self.params.insert(
            "useDeletaMush".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedeletamush_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useDeletaMush".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ObjectSimplemale {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "simplemale"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectSimplemaleOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectSimplemaleOutputs for ObjectSimplemale {}
impl ObjectSimplemaleOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSimplemale> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectSimplemaleInnerExt {
    fn fkik_arm_right_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fk_arm_right_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fk_hand_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_chops(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_arm_twist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_wrist_left_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_wrist_right_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ik_wrist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn l_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lower_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn r_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn upper_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend_wrist_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bone_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bone_clavicle_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cog_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_calf_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_foot_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_toe_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_fk_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_arm_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_football_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_leg_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_leg_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_ik_wrist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_blend_fk_ik_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_blend_fk_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_calf_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_cog(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eye_lookat_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eye_lookat_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyes_lookat(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_eyes_lookat_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_foot_ankle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_foot_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hip_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hips_rotate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_hips_translate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_jaw_ik(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ctrl_rig_bone_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_customspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_eyes_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_foot_customspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_foot_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_localspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_wrist_custom_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fetch_rig_wrist_custom_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo_skin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn hair(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn local_master_space1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn master_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn materials(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mirror_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fkik_arm_left_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_fk_arm_left_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_foot_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_foot_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_left_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_ik_wrist_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_leg_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_fk_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_customspace_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_customspace_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_eyes_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_leg_twistspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_leg_twistspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_blend_wrist_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_clavicle_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_bone_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_calf_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_cog_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_ankle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_foot_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_footleft_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_footright_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_head_space_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_head_worldspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_hip_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_hip_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_mirror_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_spine_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_space_cog_world1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rig_wrist_spaceblend_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn righ_wrist_worldspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tongue_root1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wrist_space_cog_world_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wrist_spaceblend_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wrist_worldspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectSimplemaleInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectSimplemale> {
    fn fkik_arm_right_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("FKIK_arm_right_blend")
    }
    fn fk_arm_right_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("FK_arm_right_tip")
    }
    fn fk_hand_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("FK_hand_right_ptr")
    }
    fn ik_chops(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_CHOPS")
    }
    fn ik_arm_twist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_arm_twist_right_ptr")
    }
    fn ik_wrist_left_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_wrist_left_pretransform_right")
    }
    fn ik_wrist_right_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_wrist_right_orientation")
    }
    fn ik_wrist_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IK_wrist_right_ptr")
    }
    fn l_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye")
    }
    fn l_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_bone")
    }
    fn l_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_1")
    }
    fn l_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_2")
    }
    fn l_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_3")
    }
    fn l_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_lower_bone_4")
    }
    fn l_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_null")
    }
    fn l_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_offset")
    }
    fn l_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_1")
    }
    fn l_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_2")
    }
    fn l_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_3")
    }
    fn l_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_eye_upper_bone_4")
    }
    fn l_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone1")
    }
    fn l_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone2")
    }
    fn l_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone3")
    }
    fn l_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone4")
    }
    fn l_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone5")
    }
    fn l_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone6")
    }
    fn l_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone7")
    }
    fn l_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone8")
    }
    fn l_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("L_tongue_bone9")
    }
    fn lower_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Lower_Teeth_Gums")
    }
    fn r_eye(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye")
    }
    fn r_eye_bone(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_bone")
    }
    fn r_eye_lower_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_1")
    }
    fn r_eye_lower_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_2")
    }
    fn r_eye_lower_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_3")
    }
    fn r_eye_lower_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_lower_bone_4")
    }
    fn r_eye_null(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_null")
    }
    fn r_eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_offset")
    }
    fn r_eye_upper_bone_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_1")
    }
    fn r_eye_upper_bone_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_2")
    }
    fn r_eye_upper_bone_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_3")
    }
    fn r_eye_upper_bone_4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_eye_upper_bone_4")
    }
    fn r_null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null1")
    }
    fn r_null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null2")
    }
    fn r_null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null3")
    }
    fn r_null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null4")
    }
    fn r_null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null5")
    }
    fn r_null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null6")
    }
    fn r_null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null7")
    }
    fn r_null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_null8")
    }
    fn r_tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone1")
    }
    fn r_tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone2")
    }
    fn r_tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone3")
    }
    fn r_tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone4")
    }
    fn r_tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone5")
    }
    fn r_tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone6")
    }
    fn r_tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone7")
    }
    fn r_tongue_bone8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone8")
    }
    fn r_tongue_bone9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("R_tongue_bone9")
    }
    fn upper_teeth_gums(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("Upper_Teeth_Gums")
    }
    fn blend1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend1")
    }
    fn blend_wrist_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend_wrist_space_right")
    }
    fn bone_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bone_clavicle_right")
    }
    fn bone_clavicle_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bone_clavicle_right_ptr")
    }
    fn cog_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cog_pretransform_right")
    }
    fn ctrl_fk_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_left")
    }
    fn ctrl_fk_calf_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_left_ptr")
    }
    fn ctrl_fk_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_right")
    }
    fn ctrl_fk_calf_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_calf_right_ptr")
    }
    fn ctrl_fk_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_left")
    }
    fn ctrl_fk_foot_left_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_left_preT")
    }
    fn ctrl_fk_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_right")
    }
    fn ctrl_fk_foot_right_pret(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_foot_right_preT")
    }
    fn ctrl_fk_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_left")
    }
    fn ctrl_fk_forearm_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_left_ptr")
    }
    fn ctrl_fk_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_forearm_right")
    }
    fn ctrl_fk_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_left")
    }
    fn ctrl_fk_hand_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_left_ptr")
    }
    fn ctrl_fk_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_hand_right")
    }
    fn ctrl_fk_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_1_left")
    }
    fn ctrl_fk_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_1_right")
    }
    fn ctrl_fk_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_2_left")
    }
    fn ctrl_fk_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_2_right")
    }
    fn ctrl_fk_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_3_left")
    }
    fn ctrl_fk_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_index_3_right")
    }
    fn ctrl_fk_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_1_left")
    }
    fn ctrl_fk_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_1_right")
    }
    fn ctrl_fk_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_2_left")
    }
    fn ctrl_fk_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_2_right")
    }
    fn ctrl_fk_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_3_left")
    }
    fn ctrl_fk_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_little_3_right")
    }
    fn ctrl_fk_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_1_left")
    }
    fn ctrl_fk_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_1_right")
    }
    fn ctrl_fk_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_2_left")
    }
    fn ctrl_fk_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_2_right")
    }
    fn ctrl_fk_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_3_left")
    }
    fn ctrl_fk_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_middle_3_right")
    }
    fn ctrl_fk_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_1_left")
    }
    fn ctrl_fk_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_1_right")
    }
    fn ctrl_fk_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_2_left")
    }
    fn ctrl_fk_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_2_right")
    }
    fn ctrl_fk_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_3_left")
    }
    fn ctrl_fk_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_ring_3_right")
    }
    fn ctrl_fk_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_1")
    }
    fn ctrl_fk_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_2")
    }
    fn ctrl_fk_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_spine_3")
    }
    fn ctrl_fk_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thigh_left")
    }
    fn ctrl_fk_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thigh_right")
    }
    fn ctrl_fk_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_1_left")
    }
    fn ctrl_fk_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_1_right")
    }
    fn ctrl_fk_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_2_left")
    }
    fn ctrl_fk_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_2_right")
    }
    fn ctrl_fk_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_3_left")
    }
    fn ctrl_fk_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_thumb_3_right")
    }
    fn ctrl_fk_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_left")
    }
    fn ctrl_fk_toe_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_left_ptr")
    }
    fn ctrl_fk_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_right")
    }
    fn ctrl_fk_toe_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_toe_right_ptr")
    }
    fn ctrl_fk_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_upperarm_left")
    }
    fn ctrl_fk_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_FK_upperarm_right")
    }
    fn ctrl_ik_arm_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_left")
    }
    fn ctrl_ik_arm_twist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_left_ptr")
    }
    fn ctrl_ik_arm_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_arm_twist_right")
    }
    fn ctrl_ik_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_foot_left")
    }
    fn ctrl_ik_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_foot_right")
    }
    fn ctrl_ik_football_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_football_left")
    }
    fn ctrl_ik_football_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_football_right")
    }
    fn ctrl_ik_leg_twist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_leg_twist_left")
    }
    fn ctrl_ik_leg_twist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_leg_twist_right")
    }
    fn ctrl_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toe_left")
    }
    fn ctrl_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toe_right")
    }
    fn ctrl_ik_toetip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toetip_left")
    }
    fn ctrl_ik_toetip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_toetip_right")
    }
    fn ctrl_ik_wrist_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_left")
    }
    fn ctrl_ik_wrist_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_left_ptr")
    }
    fn ctrl_ik_wrist_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_IK_wrist_right")
    }
    fn ctrl_blend_fk_ik_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_blend_FK_IK_leg_right")
    }
    fn ctrl_blend_fk_ik_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_blend_FK_IK_toe_right")
    }
    fn ctrl_calf_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_calf_tip_right")
    }
    fn ctrl_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_clavicle_left")
    }
    fn ctrl_clavicle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_clavicle_right")
    }
    fn ctrl_cog(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_cog")
    }
    fn ctrl_eye_lookat_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eye_lookat_left")
    }
    fn ctrl_eye_lookat_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eye_lookat_right")
    }
    fn ctrl_eyes_lookat(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyes_lookat")
    }
    fn ctrl_eyes_lookat_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_eyes_lookat_ptr")
    }
    fn ctrl_foot_ankle_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_foot_ankle_right")
    }
    fn ctrl_foot_tip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_foot_tip_right")
    }
    fn ctrl_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_head")
    }
    fn ctrl_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_left")
    }
    fn ctrl_hip_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_left_ptr")
    }
    fn ctrl_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_right")
    }
    fn ctrl_hip_right_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hip_right_ptr")
    }
    fn ctrl_hips_rotate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hips_rotate")
    }
    fn ctrl_hips_translate(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_hips_translate")
    }
    fn ctrl_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_jaw")
    }
    fn ctrl_jaw_ik(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_jaw_IK")
    }
    fn ctrl_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_master")
    }
    fn ctrl_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_neck")
    }
    fn ctrl_rig_bone_forearm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_forearm_left")
    }
    fn ctrl_rig_bone_hand_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_hand_left")
    }
    fn ctrl_rig_bone_hand_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_hand_right")
    }
    fn ctrl_rig_bone_neck(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_neck")
    }
    fn ctrl_rig_bone_spine_1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_1")
    }
    fn ctrl_rig_bone_spine_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_2")
    }
    fn ctrl_rig_bone_spine_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_spine_3")
    }
    fn ctrl_rig_bone_upperarm_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ctrl_rig_bone_upperarm_left")
    }
    fn eye_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("eye_offset")
    }
    fn fetch_rig_customspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_customspace")
    }
    fn fetch_rig_eyes_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_eyes_custom_space")
    }
    fn fetch_rig_foot_customspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_foot_customspace_left")
    }
    fn fetch_rig_foot_customspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_foot_customspace_right")
    }
    fn fetch_rig_localspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_localspace")
    }
    fn fetch_rig_wrist_custom_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_wrist_custom_space_left")
    }
    fn fetch_rig_wrist_custom_space_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fetch_rig_wrist_custom_space_right")
    }
    fn geo_skin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("geo_skin")
    }
    fn hair(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("hair")
    }
    fn local_master(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master")
    }
    fn local_master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master_space")
    }
    fn local_master_space1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("local_master_space1")
    }
    fn master_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("master_pretransform_right")
    }
    fn master_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("master_space")
    }
    fn materials(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("materials")
    }
    fn mirror_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mirror_master_pretransform1")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null2")
    }
    fn null3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null3")
    }
    fn null4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null4")
    }
    fn null5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null5")
    }
    fn null6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null6")
    }
    fn null7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null7")
    }
    fn null8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null8")
    }
    fn null9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null9")
    }
    fn rig_fkik_arm_left_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FKIK_arm_left_blend")
    }
    fn rig_fk_arm_left_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_FK_arm_left_tip")
    }
    fn rig_ik_foot_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_foot_left_pretransform")
    }
    fn rig_ik_foot_right_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_foot_right_pretransform")
    }
    fn rig_ik_wrist_left_orientation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_left_orientation")
    }
    fn rig_ik_wrist_left_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_IK_wrist_left_pretransform")
    }
    fn rig_blend_fk_ik_leg_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_leg_left")
    }
    fn rig_blend_fk_ik_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_FK_IK_toe_left")
    }
    fn rig_blend_custom_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_custom_space")
    }
    fn rig_blend_customspace_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_customspace_foot_left")
    }
    fn rig_blend_customspace_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_customspace_foot_right")
    }
    fn rig_blend_eyes_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_eyes_space")
    }
    fn rig_blend_leg_twistspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_leg_twistspace_left")
    }
    fn rig_blend_leg_twistspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_leg_twistspace_right")
    }
    fn rig_blend_wrist_space_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_blend_wrist_space_left")
    }
    fn rig_bone_calf_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_calf_left")
    }
    fn rig_bone_calf_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_calf_right")
    }
    fn rig_bone_clavicle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_clavicle_left")
    }
    fn rig_bone_clavicle_left_ptr(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_clavicle_left_ptr")
    }
    fn rig_bone_foot_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_foot_left")
    }
    fn rig_bone_foot_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_foot_right")
    }
    fn rig_bone_forearm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_forearm_right")
    }
    fn rig_bone_head(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_head")
    }
    fn rig_bone_hip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_hip_left")
    }
    fn rig_bone_hip_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_hip_right")
    }
    fn rig_bone_index_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_1_left")
    }
    fn rig_bone_index_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_1_right")
    }
    fn rig_bone_index_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_2_left")
    }
    fn rig_bone_index_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_2_right")
    }
    fn rig_bone_index_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_3_left")
    }
    fn rig_bone_index_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_index_3_right")
    }
    fn rig_bone_jaw(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_jaw")
    }
    fn rig_bone_little_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_1_left")
    }
    fn rig_bone_little_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_1_right")
    }
    fn rig_bone_little_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_2_left")
    }
    fn rig_bone_little_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_2_right")
    }
    fn rig_bone_little_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_3_left")
    }
    fn rig_bone_little_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_little_3_right")
    }
    fn rig_bone_middle_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_1_left")
    }
    fn rig_bone_middle_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_1_right")
    }
    fn rig_bone_middle_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_2_left")
    }
    fn rig_bone_middle_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_2_right")
    }
    fn rig_bone_middle_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_3_left")
    }
    fn rig_bone_middle_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_middle_3_right")
    }
    fn rig_bone_pelvis(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_pelvis")
    }
    fn rig_bone_ring_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_1_left")
    }
    fn rig_bone_ring_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_1_right")
    }
    fn rig_bone_ring_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_2_left")
    }
    fn rig_bone_ring_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_2_right")
    }
    fn rig_bone_ring_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_3_left")
    }
    fn rig_bone_ring_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_ring_3_right")
    }
    fn rig_bone_thigh_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thigh_left")
    }
    fn rig_bone_thigh_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thigh_right")
    }
    fn rig_bone_thumb_1_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_1_left")
    }
    fn rig_bone_thumb_1_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_1_right")
    }
    fn rig_bone_thumb_2_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_2_left")
    }
    fn rig_bone_thumb_2_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_2_right")
    }
    fn rig_bone_thumb_3_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_3_left")
    }
    fn rig_bone_thumb_3_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_thumb_3_right")
    }
    fn rig_bone_toe_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_toe_left")
    }
    fn rig_bone_toe_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_toe_right")
    }
    fn rig_bone_upperarm_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_bone_upperarm_right")
    }
    fn rig_calf_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_calf_tip_left")
    }
    fn rig_cog_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_cog_pretransform1")
    }
    fn rig_foot_ankle_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_ankle_left")
    }
    fn rig_foot_tip_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_foot_tip_left")
    }
    fn rig_footleft_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_footleft_pretransform")
    }
    fn rig_footright_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_footright_pretransform")
    }
    fn rig_head_space_blend(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_head_space_blend")
    }
    fn rig_head_worldspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_head_worldspace")
    }
    fn rig_hip_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_hip_pretransform")
    }
    fn rig_hip_pretransform_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_hip_pretransform_right")
    }
    fn rig_master_pretransform(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform")
    }
    fn rig_master_pretransform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_master_pretransform1")
    }
    fn rig_mirror_leg_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_mirror_leg_right")
    }
    fn rig_spine_tip(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_spine_tip")
    }
    fn rig_wrist_space_cog_world1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_space_cog_world1")
    }
    fn rig_wrist_spaceblend_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rig_wrist_spaceblend_left")
    }
    fn righ_wrist_worldspace_left(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("righ_wrist_worldspace_left")
    }
    fn tongue_bone1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone1")
    }
    fn tongue_bone2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone2")
    }
    fn tongue_bone3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone3")
    }
    fn tongue_bone4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone4")
    }
    fn tongue_bone5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone5")
    }
    fn tongue_bone6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone6")
    }
    fn tongue_bone7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_bone7")
    }
    fn tongue_root1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tongue_root1")
    }
    fn wrist_space_cog_world_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wrist_space_cog_world_right")
    }
    fn wrist_spaceblend_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wrist_spaceblend_right")
    }
    fn wrist_worldspace_right(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wrist_worldspace_right")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSoundXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSoundRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSoundPreXform {
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
pub enum ObjectSoundUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSoundDropoff {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone)]
pub struct ObjectSound {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectSound {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_parent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
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
    pub fn with_volume(mut self, val: f32) -> Self {
        self.params.insert(
            "volume".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emitcone(mut self, val: f32) -> Self {
        self.params.insert(
            "emitcone".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emitcone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcone".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outcone(mut self, val: f32) -> Self {
        self.params.insert(
            "outcone".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outcone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outcone".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_droprate(mut self, val: f32) -> Self {
        self.params.insert(
            "droprate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_droprate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droprate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outvolume(mut self, val: f32) -> Self {
        self.params.insert(
            "outvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outvolume".to_string(),
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
    pub fn with_soundactive(mut self, val: i32) -> Self {
        self.params.insert(
            "soundactive".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_soundactive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soundactive".to_string(),
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
    pub fn with_xord(mut self, val: ObjectSoundXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectSoundRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectSoundPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectSoundUparmtype) -> Self {
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
    pub fn with_dropoff(mut self, val: ObjectSoundDropoff) -> Self {
        self.params.insert(
            "dropoff".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dropoff".to_string(),
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
    pub fn with_chop_sourcepath(mut self, val: &str) -> Self {
        self.params.insert(
            "chop_sourcepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_chop_sourcepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chop_sourcepath".to_string(),
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
    pub fn with_direct(mut self, val: bool) -> Self {
        self.params.insert(
            "direct".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_direct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for ObjectSound {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "sound"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectSoundOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectSoundOutputs for ObjectSound {}
impl ObjectSoundOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSound> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectSoundInnerExt {
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn file1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tube1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn tube2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectSoundInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectSound> {
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add1")
    }
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add2")
    }
    fn file1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("file1")
    }
    fn merge2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("merge2")
    }
    fn tube1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tube1")
    }
    fn tube2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("tube2")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStadiumcrowdsexamplePreXform {
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
pub enum ObjectStadiumcrowdsexampleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStadiumcrowdsexampleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStadiumcrowdsexampleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectStadiumcrowdsexample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectStadiumcrowdsexample {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
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
    pub fn with_pre_xform(mut self, val: ObjectStadiumcrowdsexamplePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectStadiumcrowdsexampleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectStadiumcrowdsexampleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectStadiumcrowdsexampleUparmtype) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for ObjectStadiumcrowdsexample {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "stadiumcrowdsexample"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectStadiumcrowdsexampleOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectStadiumcrowdsexampleOutputs for ObjectStadiumcrowdsexample {}
impl ObjectStadiumcrowdsexampleOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectStadiumcrowdsexample>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectStadiumcrowdsexampleInnerExt {
    fn agent_setup(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cam1_side(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cam2_center(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cam3_corner(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cheer_bbox(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn crowd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn crowd_sim(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn envlight(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lookat_pts(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn material_variation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn render(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rotation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sit_cheer_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sit_cheer_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sit_clap_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sit_idle_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sit_idle_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sit_stand_003(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stadium(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_boo_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_boo_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_boo_003(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_cheer_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_cheer_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_idle_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_idle_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_idle_004(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stand_sit_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectStadiumcrowdsexampleInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectStadiumcrowdsexample>
{
    fn agent_setup(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("agent_setup")
    }
    fn cam1_side(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cam1_side")
    }
    fn cam2_center(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cam2_center")
    }
    fn cam3_corner(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cam3_corner")
    }
    fn cheer_bbox(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cheer_bbox")
    }
    fn crowd(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("crowd")
    }
    fn crowd_sim(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("crowd_sim")
    }
    fn envlight(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("envlight")
    }
    fn lookat_pts(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lookat_pts")
    }
    fn material_variation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("material_variation")
    }
    fn render(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("render")
    }
    fn rotation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rotation")
    }
    fn sit_cheer_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sit_cheer_001")
    }
    fn sit_cheer_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sit_cheer_002")
    }
    fn sit_clap_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sit_clap_001")
    }
    fn sit_idle_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sit_idle_001")
    }
    fn sit_idle_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sit_idle_002")
    }
    fn sit_stand_003(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sit_stand_003")
    }
    fn stadium(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stadium")
    }
    fn stand_boo_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_boo_001")
    }
    fn stand_boo_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_boo_002")
    }
    fn stand_boo_003(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_boo_003")
    }
    fn stand_cheer_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_cheer_001")
    }
    fn stand_cheer_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_cheer_002")
    }
    fn stand_idle_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_idle_001")
    }
    fn stand_idle_002(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_idle_002")
    }
    fn stand_idle_004(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_idle_004")
    }
    fn stand_sit_001(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stand_sit_001")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamPreXform {
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
pub enum ObjectStereocamUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectStereocam {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectStereocam {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_parent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
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
    pub fn with_xord(mut self, val: ObjectStereocamXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectStereocamRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectStereocamPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectStereocamUparmtype) -> Self {
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
    pub fn with_leftcam(mut self, val: &str) -> Self {
        self.params.insert(
            "leftcam".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_leftcam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftcam".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightcam(mut self, val: &str) -> Self {
        self.params.insert(
            "rightcam".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rightcam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightcam".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for ObjectStereocam {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "stereocam"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectStereocamOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectStereocamOutputs for ObjectStereocam {}
impl ObjectStereocamOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectStereocam> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectStereocamInnerExt {
    fn camorigin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn file1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectStereocamInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectStereocam> {
    fn camorigin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("camOrigin")
    }
    fn file1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("file1")
    }
    fn xform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xform1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamrigXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamrigRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamrigPreXform {
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
pub enum ObjectStereocamrigUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStereocamrigViewFrustum {
    None = 0,
    Center = 1,
    Left = 2,
    Right = 3,
    LeftAndRight = 4,
    /// Left and Right (Intersection)
    LeftAndRightIntersection = 5,
}

#[derive(Debug, Clone)]
pub struct ObjectStereocamrig {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectStereocamrig {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_sub_network_input_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
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
    pub fn with_zps(mut self, val: f32) -> Self {
        self.params.insert(
            "ZPS".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ZPS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interaxial(mut self, val: f32) -> Self {
        self.params.insert(
            "interaxial".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interaxial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interaxial".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interaxial_left(mut self, val: f32) -> Self {
        self.params.insert(
            "interaxial_left".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interaxial_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interaxial_left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interaxial_right(mut self, val: f32) -> Self {
        self.params.insert(
            "interaxial_right".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interaxial_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interaxial_right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_left(mut self, val: f32) -> Self {
        self.params.insert(
            "r_left".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r_right(mut self, val: f32) -> Self {
        self.params.insert(
            "r_right".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_r_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r_right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_frustum_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "frustum_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frustum_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frustum_alpha".to_string(),
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
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
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
    pub fn with_cropl(mut self, val: f32) -> Self {
        self.params.insert(
            "cropl".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cropr(mut self, val: f32) -> Self {
        self.params.insert(
            "cropr".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cropb(mut self, val: f32) -> Self {
        self.params.insert(
            "cropb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cropt(mut self, val: f32) -> Self {
        self.params.insert(
            "cropt".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cropt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cropt".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shutter(mut self, val: f32) -> Self {
        self.params.insert(
            "shutter".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shutter".to_string(),
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
    pub fn with_fstop(mut self, val: f32) -> Self {
        self.params.insert(
            "fstop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fstop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fstop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pivot_left(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pivot_left".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pivot_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot_left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pivot_right(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pivot_right".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pivot_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot_right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_win(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "win".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_win_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "win".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_winsize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "winsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_winsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "winsize".to_string(),
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
    pub fn with_xord(mut self, val: ObjectStereocamrigXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectStereocamrigRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectStereocamrigPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectStereocamrigUparmtype) -> Self {
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
    pub fn with_view_frustum(mut self, val: ObjectStereocamrigViewFrustum) -> Self {
        self.params.insert(
            "view_frustum".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_view_frustum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "view_frustum".to_string(),
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
    pub fn with_focalunits(mut self, val: &str) -> Self {
        self.params.insert(
            "focalunits".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focalunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focalunits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_bokeh(mut self, val: &str) -> Self {
        self.params.insert(
            "vm_bokeh".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vm_bokeh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_bokeh".to_string(),
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
    pub fn with_infinite_zps(mut self, val: bool) -> Self {
        self.params.insert(
            "infinite_ZPS".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_infinite_zps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "infinite_ZPS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_show_axial(mut self, val: bool) -> Self {
        self.params.insert(
            "show_axial".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_axial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_axial".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewmenu_stereo(mut self, val: bool) -> Self {
        self.params.insert(
            "viewmenu_stereo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viewmenu_stereo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewmenu_stereo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewmenu_left(mut self, val: bool) -> Self {
        self.params.insert(
            "viewmenu_left".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viewmenu_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewmenu_left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewmenu_right(mut self, val: bool) -> Self {
        self.params.insert(
            "viewmenu_right".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viewmenu_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewmenu_right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewmenu_center(mut self, val: bool) -> Self {
        self.params.insert(
            "viewmenu_center".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viewmenu_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewmenu_center".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_toe_in(mut self, val: bool) -> Self {
        self.params.insert(
            "toe_in".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_toe_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toe_in".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_show_pivot_control(mut self, val: bool) -> Self {
        self.params.insert(
            "show_pivot_control".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_pivot_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_pivot_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_show_zps(mut self, val: bool) -> Self {
        self.params.insert(
            "show_ZPS".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_zps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_ZPS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_translucent_frustum(mut self, val: bool) -> Self {
        self.params.insert(
            "translucent_frustum".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translucent_frustum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translucent_frustum".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ObjectStereocamrig {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "stereocamrig"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectStereocamrigOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectStereocamrigOutputs for ObjectStereocamrig {}
impl ObjectStereocamrigOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectStereocamrig>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectStereocamrigInnerExt {
    fn hit_calculations(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn center_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn constraints(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn curved_double_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn double_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn invert_z_dir(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn joint_interaxial_a(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn joint_interaxial_b(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn left_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn left_camera_rotation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn left_pivot_control(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn left_pivot_control_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn offset_a(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn offset_b(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn quad_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn right_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn right_camera_rotation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn right_pivot_control(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn right_pivot_control_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn single_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stereo_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn view_center_frustum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn view_coords(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn view_left_frustum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn view_left_right_intersection(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn view_right_frustum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn visualization_root(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zero_parallax_setting(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectStereocamrigInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectStereocamrig>
{
    fn hit_calculations(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("HIT_calculations")
    }
    fn center_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("center_camera")
    }
    fn constraints(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("constraints")
    }
    fn curved_double_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("curved_double_arrow")
    }
    fn double_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("double_arrow")
    }
    fn invert_z_dir(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invert_z_dir")
    }
    fn joint_interaxial_a(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("joint_interaxial_A")
    }
    fn joint_interaxial_b(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("joint_interaxial_B")
    }
    fn left_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("left_camera")
    }
    fn left_camera_rotation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("left_camera_rotation")
    }
    fn left_pivot_control(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("left_pivot_control")
    }
    fn left_pivot_control_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("left_pivot_control_offset")
    }
    fn offset_a(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("offset_A")
    }
    fn offset_b(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("offset_B")
    }
    fn quad_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("quad_arrow")
    }
    fn right_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("right_camera")
    }
    fn right_camera_rotation(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("right_camera_rotation")
    }
    fn right_pivot_control(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("right_pivot_control")
    }
    fn right_pivot_control_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("right_pivot_control_offset")
    }
    fn single_arrow(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("single_arrow")
    }
    fn stereo_camera(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stereo_camera")
    }
    fn view_center_frustum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("view_center_frustum")
    }
    fn view_coords(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("view_coords")
    }
    fn view_left_frustum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("view_left_frustum")
    }
    fn view_left_right_intersection(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("view_left_right_intersection")
    }
    fn view_right_frustum(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("view_right_frustum")
    }
    fn visualization_root(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("visualization_root")
    }
    fn zero_parallax_setting(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zero_parallax_setting")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStickyDisplayicon {
    Icon = 0,
    Axis = 1,
    IconAndAxis = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStickyControltype {
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
pub enum ObjectStickyOrientation {
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
pub enum ObjectStickyXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStickyRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStickyPreXform {
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
pub enum ObjectStickyUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectSticky {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectSticky {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_parent_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn trigger_stickyclearuv(mut self) -> Self {
        self.params.insert(
            "stickyclearuv".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_stickyrot(mut self, val: f32) -> Self {
        self.params.insert(
            "stickyrot".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickyrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyrot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geoscale(mut self, val: f32) -> Self {
        self.params.insert(
            "geoscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_geoscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geoscale".to_string(),
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
    pub fn with_stickyuv(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "stickyuv".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stickyuv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyuv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyurange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "stickyurange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stickyurange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyurange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyvrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "stickyvrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stickyvrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyvrange".to_string(),
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
    pub fn with_geosize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "geosize".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_geosize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geocenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "geocenter".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_geocenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geocenter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_georotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "georotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_georotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "georotate".to_string(),
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
    pub fn with_displayicon(mut self, val: ObjectStickyDisplayicon) -> Self {
        self.params.insert(
            "displayicon".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_displayicon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayicon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controltype(mut self, val: ObjectStickyControltype) -> Self {
        self.params.insert(
            "controltype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_controltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "controltype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientation(mut self, val: ObjectStickyOrientation) -> Self {
        self.params.insert(
            "orientation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orientation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: ObjectStickyXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectStickyRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectStickyPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectStickyUparmtype) -> Self {
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
    pub fn with_stickysop(mut self, val: &str) -> Self {
        self.params.insert(
            "stickysop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickysop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickysop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stickyattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyattrib".to_string(),
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
    pub fn with_geocustom(mut self, val: &str) -> Self {
        self.params.insert(
            "geocustom".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geocustom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geocustom".to_string(),
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
    pub fn with_stickyorient(mut self, val: bool) -> Self {
        self.params.insert(
            "stickyorient".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickyorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fetchworld(mut self, val: bool) -> Self {
        self.params.insert(
            "fetchworld".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fetchworld_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fetchworld".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickywrapu(mut self, val: bool) -> Self {
        self.params.insert(
            "stickywrapu".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickywrapu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickywrapu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickywrapv(mut self, val: bool) -> Self {
        self.params.insert(
            "stickywrapv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickywrapv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickywrapv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyuvconstant(mut self, val: bool) -> Self {
        self.params.insert(
            "stickyuvconstant".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickyuvconstant_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyuvconstant".to_string(),
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
    pub fn with_renderspace(mut self, val: bool) -> Self {
        self.params.insert(
            "renderspace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderspace".to_string(),
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
    pub fn with_shadedmode(mut self, val: bool) -> Self {
        self.params.insert(
            "shadedmode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadedmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadedmode".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for ObjectSticky {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "sticky"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectStickyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectStickyOutputs for ObjectSticky {}
impl ObjectStickyOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSticky> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectStickyInnerExt {
    fn control1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn point1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectStickyInnerExt for houdini_ramen_core::graph::InnerGraph<'a, ObjectSticky> {
    fn control1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("control1")
    }
    fn point1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("point1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStreetcrowdsexamplePreXform {
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
pub enum ObjectStreetcrowdsexampleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStreetcrowdsexampleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectStreetcrowdsexampleUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectStreetcrowdsexample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectStreetcrowdsexample {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
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
    pub fn with_pre_xform(mut self, val: ObjectStreetcrowdsexamplePreXform) -> Self {
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
    pub fn with_xord(mut self, val: ObjectStreetcrowdsexampleXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectStreetcrowdsexampleRord) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectStreetcrowdsexampleUparmtype) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for ObjectStreetcrowdsexample {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "streetcrowdsexample"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectStreetcrowdsexampleOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectStreetcrowdsexampleOutputs for ObjectStreetcrowdsexample {}
impl ObjectStreetcrowdsexampleOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectStreetcrowdsexample>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ObjectStreetcrowdsexampleInnerExt {
    fn cam1_side(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cam2_front(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cam3_overhead(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn crowd_sim(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn crowdsource(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn environment(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living_rest_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living_run_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living_setup(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living_stand_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living_walkfast_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn living_walknormal_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn obstacle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn render(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn roadpath(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn shopnet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn skylight1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stoplight(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn sunlight1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zombie_rest_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zombie_setup(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zombie_stand_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zombie_walk_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zombies(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ObjectStreetcrowdsexampleInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, ObjectStreetcrowdsexample>
{
    fn cam1_side(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cam1_side")
    }
    fn cam2_front(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cam2_front")
    }
    fn cam3_overhead(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cam3_overhead")
    }
    fn crowd_sim(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("crowd_sim")
    }
    fn crowdsource(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("crowdsource")
    }
    fn environment(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("environment")
    }
    fn living(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living")
    }
    fn living_rest_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living_rest_cycle")
    }
    fn living_run_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living_run_cycle")
    }
    fn living_setup(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living_setup")
    }
    fn living_stand_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living_stand_cycle")
    }
    fn living_walkfast_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living_walkfast_cycle")
    }
    fn living_walknormal_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("living_walknormal_cycle")
    }
    fn obstacle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("obstacle")
    }
    fn render(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("render")
    }
    fn roadpath(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("roadPath")
    }
    fn shopnet1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("shopnet1")
    }
    fn skylight1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("skylight1")
    }
    fn stoplight(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stoplight")
    }
    fn sunlight1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sunlight1")
    }
    fn zombie_rest_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zombie_rest_cycle")
    }
    fn zombie_setup(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zombie_setup")
    }
    fn zombie_stand_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zombie_stand_cycle")
    }
    fn zombie_walk_cycle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zombie_walk_cycle")
    }
    fn zombies(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zombies")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSubnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSubnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSubnetPreXform {
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
pub enum ObjectSubnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectSubnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ObjectSubnet {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_sub_network_input_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_sub_network_input_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_sub_network_input_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_sub_network_input_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
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
    pub fn with_xord(mut self, val: ObjectSubnetXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectSubnetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectSubnetPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectSubnetUparmtype) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for ObjectSubnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "subnet"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectSubnetOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectSubnetOutputs for ObjectSubnet {}
impl ObjectSubnetOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSubnet> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSwitcherXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSwitcherRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSwitcherPreXform {
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
pub enum ObjectSwitcherUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ObjectSwitcher {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl ObjectSwitcher {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn add_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs
            .insert(self.next_input_index, (out.node_id, out.pin));
        self.next_input_index += 1;
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
    pub fn with_camswitch(mut self, val: i32) -> Self {
        self.params.insert(
            "camswitch".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_camswitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camswitch".to_string(),
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
    pub fn with_xord(mut self, val: ObjectSwitcherXord) -> Self {
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
    pub fn with_rord(mut self, val: ObjectSwitcherRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ObjectSwitcherPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ObjectSwitcherUparmtype) -> Self {
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
}

impl houdini_ramen_core::types::HoudiniNode for ObjectSwitcher {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "switcher"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait ObjectSwitcherOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ObjectSwitcherOutputs for ObjectSwitcher {}
impl ObjectSwitcherOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ObjectSwitcher> {}
