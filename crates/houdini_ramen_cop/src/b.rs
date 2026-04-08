#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesTracingmode {
    SurfaceNormal = 0,
    CageMesh = 1,
    /// Single Mesh ( Low Only )
    SingleMeshLowOnly = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesVisibilityculling {
    NoCulling = 0,
    Backface = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesNormals {
    ErrorIfLowResMeshMissingNormals = 0,
    WarningIfLowResMeshMissingNormals = 1,
    SilentlyCreateMissingNormals = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesTangents {
    AddMikktTangentsIfNeeded = 0,
    AddTextureUvTangentsIfNeeded = 1,
    ErrorIfNoTangentAttributesPresent = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesExterior {
    Unchanged = 0,
    Black = 1,
    DiffuseFill = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesFlipn {
    GreenDown = 0,
    GreenUp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesNormaltype {
    /// Signed (-1 to 1)
    SignedMinus1To1 = 0,
    /// Offset (0 to 1)
    Offset0To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesRoundednormaltype {
    /// Signed (-1 to 1)
    SignedMinus1To1 = 0,
    /// Offset (0 to 1)
    Offset0To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesRoundednormalmode {
    ConcaveAndConvex = 0,
    Concave = 1,
    Convex = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesWorldspace {
    /// Right-Handed ( Y-Up )
    RightMinusHandedYMinusUp = 0,
    /// Right-Handed ( Z-Up )
    RightMinusHandedZMinusUp = 1,
    /// Left-Handed ( Y-Up )
    LeftMinusHandedYMinusUp = 2,
    /// Left-Handed ( Z-Up )
    LeftMinusHandedZMinusUp = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesWorldnormaltype {
    /// Signed (-1 to 1)
    SignedMinus1To1 = 0,
    /// Offset (0 to 1)
    Offset0To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesPositiontype {
    /// Absolute ( Use Actual Position Values )
    AbsoluteUseActualPositionValues = 0,
    /// Relative ( Use Position Within Bounding Box)
    RelativeUsePositionWithinBoundingBox = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesBounds {
    FitToBoundingBox = 0,
    /// Fit to Uniform Box (No Stretching)
    FitToUniformBoxNoStretching = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesEdgemode {
    ConcaveAndConvex = 0,
    Concave = 1,
    Convex = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesHeighttype {
    TrueDisplacement = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesQuicksetup {
    /// Quick Setups ↓
    QuickSetups = 0,
    /// Vertex Color (Cd)
    VertexColorCd = 1,
    PieceMatchId = 2,
    Name = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBakegeometrytexturesType {
    Mono = 0,
    Uv = 1,
    Rgb = 2,
    Rgba = 3,
    Id = 4,
}

#[derive(Debug, Clone)]
pub struct CopBakegeometrytextures {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBakegeometrytextures {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_samplemultiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "samplemultiplier".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samplemultiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemultiplier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cageoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "cageoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cageoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cageoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "raybias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raybias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgepadding(mut self, val: f32) -> Self {
        self.params.insert(
            "edgepadding".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgepadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgepadding".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgeoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "edgeoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgeoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundednormalradius(mut self, val: f32) -> Self {
        self.params.insert(
            "roundednormalradius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roundednormalradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundednormalradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundednormalcuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "roundednormalcuspangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roundednormalcuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundednormalcuspangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_occlusionmaxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "occlusionmaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_occlusionmaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlusionmaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturescale(mut self, val: f32) -> Self {
        self.params.insert(
            "curvaturescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturemaxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "curvaturemaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturemaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturemaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturebias(mut self, val: f32) -> Self {
        self.params.insert(
            "curvaturebias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturebias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "edgeradius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgeradius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgecuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "edgecuspangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgecuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgecuspangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cavitymaxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "cavitymaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cavitymaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cavitymaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cavitybias(mut self, val: f32) -> Self {
        self.params.insert(
            "cavitybias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cavitybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cavitybias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_heightinnerdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "heightinnerdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_heightinnerdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightinnerdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_heightouterdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "heightouterdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_heightouterdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightouterdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_udim(mut self, val: i32) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundednormalsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "roundednormalsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_roundednormalsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundednormalsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_occlusionsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "occlusionsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_occlusionsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlusionsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturesamples(mut self, val: i32) -> Self {
        self.params.insert(
            "curvaturesamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_curvaturesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturesamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgesamples(mut self, val: i32) -> Self {
        self.params.insert(
            "edgesamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_edgesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgesamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cavitysamples(mut self, val: i32) -> Self {
        self.params.insert(
            "cavitysamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cavitysamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cavitysamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknesssamples(mut self, val: i32) -> Self {
        self.params.insert(
            "thicknesssamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_thicknesssamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknesssamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_tracingmode(mut self, val: CopBakegeometrytexturesTracingmode) -> Self {
        self.params.insert(
            "tracingmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tracingmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tracingmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibilityculling(mut self, val: CopBakegeometrytexturesVisibilityculling) -> Self {
        self.params.insert(
            "visibilityculling".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_visibilityculling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibilityculling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normals(mut self, val: CopBakegeometrytexturesNormals) -> Self {
        self.params.insert(
            "normals".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_normals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normals".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tangents(mut self, val: CopBakegeometrytexturesTangents) -> Self {
        self.params.insert(
            "tangents".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tangents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exterior(mut self, val: CopBakegeometrytexturesExterior) -> Self {
        self.params.insert(
            "exterior".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exterior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exterior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flipn(mut self, val: CopBakegeometrytexturesFlipn) -> Self {
        self.params.insert(
            "flipN".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_flipn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flipN".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normaltype(mut self, val: CopBakegeometrytexturesNormaltype) -> Self {
        self.params.insert(
            "normaltype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_normaltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaltype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundednormaltype(mut self, val: CopBakegeometrytexturesRoundednormaltype) -> Self {
        self.params.insert(
            "roundednormaltype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_roundednormaltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundednormaltype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roundednormalmode(mut self, val: CopBakegeometrytexturesRoundednormalmode) -> Self {
        self.params.insert(
            "roundednormalmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_roundednormalmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundednormalmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldspace(mut self, val: CopBakegeometrytexturesWorldspace) -> Self {
        self.params.insert(
            "worldspace".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_worldspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldnormaltype(mut self, val: CopBakegeometrytexturesWorldnormaltype) -> Self {
        self.params.insert(
            "worldnormaltype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_worldnormaltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldnormaltype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positiontype(mut self, val: CopBakegeometrytexturesPositiontype) -> Self {
        self.params.insert(
            "positiontype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_positiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positiontype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bounds(mut self, val: CopBakegeometrytexturesBounds) -> Self {
        self.params.insert(
            "bounds".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_edgemode(mut self, val: CopBakegeometrytexturesEdgemode) -> Self {
        self.params.insert(
            "edgemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_edgemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_heighttype(mut self, val: CopBakegeometrytexturesHeighttype) -> Self {
        self.params.insert(
            "heighttype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_heighttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heighttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quicksetup(mut self, val: CopBakegeometrytexturesQuicksetup) -> Self {
        self.params.insert(
            "quicksetup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quicksetup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quicksetup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: CopBakegeometrytexturesType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_tracer(mut self, val: &str) -> Self {
        self.params.insert(
            "tracer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tracer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tracer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "uvattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_includeset(mut self, val: &str) -> Self {
        self.params.insert(
            "includeset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_includeset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includeset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludeset(mut self, val: &str) -> Self {
        self.params.insert(
            "excludeset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludeset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attrib{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attrib{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_overrideudim(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideudim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideudim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createcage(mut self, val: bool) -> Self {
        self.params.insert(
            "createcage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cagesoften(mut self, val: bool) -> Self {
        self.params.insert(
            "cagesoften".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cagesoften_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cagesoften".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablematchpieces(mut self, val: bool) -> Self {
        self.params.insert(
            "enablematchpieces".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablematchpieces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablematchpieces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableuvfilling(mut self, val: bool) -> Self {
        self.params.insert(
            "enableuvfilling".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableuvfilling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableuvfilling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludealphafromuvfill(mut self, val: bool) -> Self {
        self.params.insert(
            "excludealphafromuvfill".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_excludealphafromuvfill_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludealphafromuvfill".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakenormal(mut self, val: bool) -> Self {
        self.params.insert(
            "bakenormal".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakenormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakeroundednormal(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeroundednormal".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeroundednormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeroundednormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakeworldnormal(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeworldnormal".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeworldnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeworldnormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakeposition(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeposition".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeposition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakeocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useocclusionmaxdist(mut self, val: bool) -> Self {
        self.params.insert(
            "useocclusionmaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useocclusionmaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useocclusionmaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakecurvature(mut self, val: bool) -> Self {
        self.params.insert(
            "bakecurvature".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakecurvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakecurvature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakeedge(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeedge".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeedge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakecavity(mut self, val: bool) -> Self {
        self.params.insert(
            "bakecavity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakecavity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakecavity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usecavitymaxdist(mut self, val: bool) -> Self {
        self.params.insert(
            "usecavitymaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecavitymaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecavitymaxdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakethickness(mut self, val: bool) -> Self {
        self.params.insert(
            "bakethickness".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakethickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakethickness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thicknessinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "thicknessinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_thicknessinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakeheight(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeheight".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeheight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bakealpha(mut self, val: bool) -> Self {
        self.params.insert(
            "bakealpha".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakealpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakealpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maskhighres(mut self, val: bool) -> Self {
        self.params.insert(
            "maskhighres".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskhighres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskhighres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doattrib_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("doattrib{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("doattrib{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBakegeometrytextures {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bakegeometrytextures"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBendMode {
    Corners = 0,
    Sides = 1,
    Angle = 2,
}

#[derive(Debug, Clone)]
pub struct CopBend {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBend {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bendangle(mut self, val: f32) -> Self {
        self.params.insert(
            "bendangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_capturedirection(mut self, val: f32) -> Self {
        self.params.insert(
            "capturedirection".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_capturedirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturedirection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_capturelength(mut self, val: f32) -> Self {
        self.params.insert(
            "capturelength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_capturelength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "capturelength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_bottomleft(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottomleft".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottomleft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottomleft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bottomright(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottomright".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottomright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottomright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topleft(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "topleft".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_topleft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topleft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topright(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "topright".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_topright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bottomleft_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottomleft_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottomleft_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottomleft_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bottomleft_v(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottomleft_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottomleft_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottomleft_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bottomright_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottomright_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottomright_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottomright_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bottomright_v(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottomright_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottomright_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottomright_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topleft_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "topleft_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_topleft_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topleft_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topleft_v(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "topleft_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_topleft_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topleft_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topright_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "topright_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_topright_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topright_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topright_v(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "topright_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_topright_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topright_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bottom(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bottom".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bottom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_top(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "top".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_top_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "top".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_left(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "left".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_right(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "right".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_captureorigin(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "captureorigin".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_captureorigin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureorigin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: CopBendMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_deforminbothdirections(mut self, val: bool) -> Self {
        self.params.insert(
            "deforminbothdirections".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deforminbothdirections_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deforminbothdirections".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bend"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlendBorder {
    Auto = 0,
    Constant = 1,
    Clamp = 2,
    Mirror = 3,
    Wrap = 4,
    Clip = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlendFilter {
    Point = 0,
    Bilinear = 1,
    Box = 2,
    Bartlett = 3,
    /// Catmull-Rom
    CatmullMinusRom = 4,
    Mitchell = 5,
    /// B-spline
    BMinusSpline = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlendMode {
    /// ![BUTTONS_blendmode_blend]Blend
    Blend = 0,
    /// ![BUTTONS_blendmode_over]Over
    Over = 1,
    /// ![BUTTONS_blendmode_under]Under
    Under = 2,
    /// ![BUTTONS_blendmode_add]Add
    Add = 3,
    /// ![BUTTONS_blendmode_subtract]Subtract
    Subtract = 4,
    /// ![BUTTONS_blendmode_multiply]Multiply
    Multiply = 5,
    /// ![BUTTONS_blendmode_divide]Divide
    Divide = 6,
    /// ![BUTTONS_blendmode_screen]Screen
    Screen = 7,
    /// ![BUTTONS_blendmode_hypot]Hypot
    Hypot = 8,
    /// ![BUTTONS_blendmode_difference]Difference
    Difference = 9,
    /// ![BUTTONS_blendmode_exclusion]Exclusion
    Exclusion = 10,
    /// ![BUTTONS_blendmode_sharpen]Sharpen
    Sharpen = 11,
    /// ![BUTTONS_blendmode_maximum]Maximum
    Maximum = 12,
    /// ![BUTTONS_blendmode_minimum]Minimum
    Minimum = 13,
    /// ![BUTTONS_blendmode_overlay]Overlay
    Overlay = 14,
    /// ![BUTTONS_blendmode_softlight]SoftLight
    Softlight = 15,
    /// ![BUTTONS_blendmode_hardlight]HardLight
    Hardlight = 16,
    /// ![BUTTONS_blendmode_dodge]Dodge
    Dodge = 17,
    /// ![BUTTONS_blendmode_burn]Burn
    Burn = 18,
    /// ![BUTTONS_blendmode_hue]Hue
    Hue = 19,
    /// ![BUTTONS_blendmode_saturation]Saturation
    Saturation = 20,
    /// ![BUTTONS_blendmode_luminosity]Luminosity
    Luminosity = 21,
    /// ![BUTTONS_blendmode_color]Color
    Color = 22,
}

#[derive(Debug, Clone)]
pub struct CopBlend {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBlend {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_border(mut self, val: CopBlendBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filter(mut self, val: CopBlendFilter) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: CopBlendMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_alpha(mut self, val: bool) -> Self {
        self.params.insert(
            "alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_swap(mut self, val: bool) -> Self {
        self.params.insert(
            "swap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uselength(mut self, val: bool) -> Self {
        self.params.insert(
            "uselength".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBlend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "blend"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlockBeginType {
    Mono = 0,
    Uv = 1,
    Rgb = 2,
    Rgba = 3,
    Id = 4,
    Geometry = 5,
    IntegerVdb = 6,
    FloatVdb = 7,
    VectorVdb = 8,
    Cable = 9,
}

#[derive(Debug, Clone)]
pub struct CopBlockBegin {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBlockBegin {
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

    // --- Menu parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: CopBlockBeginType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_blockpath(mut self, val: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blockpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBlockBegin {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "block_begin"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlockEndType {
    Mono = 0,
    Uv = 1,
    Rgb = 2,
    Rgba = 3,
    Id = 4,
    Geometry = 5,
    IntegerVdb = 6,
    FloatVdb = 7,
    VectorVdb = 8,
    Cable = 9,
}

#[derive(Debug, Clone)]
pub struct CopBlockEnd {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBlockEnd {
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

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_continuouscook_toggle(mut self) -> Self {
        self.params.insert(
            "continuouscook_toggle".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_slapcompaddaovs(mut self) -> Self {
        self.params.insert(
            "slapcompaddaovs".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_continuouscook_tick(mut self, val: i32) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_continuouscook_tick_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert(
            "startframe".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachedframes(mut self, val: i32) -> Self {
        self.params.insert(
            "cachedframes".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachedframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachedframes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointframes(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpointframes".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointframes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: CopBlockEndType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_simulate(mut self, val: bool) -> Self {
        self.params.insert(
            "simulate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_simulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "simulate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_continuouscook(mut self, val: bool) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuouscook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_docompile(mut self, val: bool) -> Self {
        self.params.insert(
            "docompile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docompile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docompile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slapcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "slapcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_slapcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slapcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slapcompcameraspace(mut self, val: bool) -> Self {
        self.params.insert(
            "slapcompcameraspace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_slapcompcameraspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slapcompcameraspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBlockEnd {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "block_end"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlocktogeoBindstates {
    AllConnected = 0,
    Network = 1,
}

#[derive(Debug, Clone)]
pub struct CopBlocktogeo {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBlocktogeo {
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

    // --- Menu parameters ---
    pub fn with_bindstates(mut self, val: CopBlocktogeoBindstates) -> Self {
        self.params.insert(
            "bindstates".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindstates_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindstates".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_blockpath(mut self, val: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blockpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBlocktogeo {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "blocktogeo"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlurFilter {
    Box = 0,
    Gaussian = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBlurUnits {
    Image = 0,
    Pixels = 1,
}

#[derive(Debug, Clone)]
pub struct CopBlur {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBlur {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_size_pixel(mut self, val: f32) -> Self {
        self.params.insert(
            "size_pixel".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_size_pixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size_pixel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_scales(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "scales".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_scales_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scales".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_filter(mut self, val: CopBlurFilter) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: CopBlurUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_readoutside(mut self, val: bool) -> Self {
        self.params.insert(
            "readoutside".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_readoutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "readoutside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scalebytimeinc(mut self, val: bool) -> Self {
        self.params.insert(
            "scalebytimeinc".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scalebytimeinc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalebytimeinc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBlur {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "blur"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBokehFilter {
    Point = 0,
    Bilinear = 1,
    Box = 2,
    Bartlett = 3,
    /// Catmull-Rom
    CatmullMinusRom = 4,
    Mitchell = 5,
    /// B-spline
    BMinusSpline = 6,
}

#[derive(Debug, Clone)]
pub struct CopBokeh {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBokeh {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
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
    pub fn with_bokehgain(mut self, val: f32) -> Self {
        self.params.insert(
            "bokehgain".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bokehgain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokehgain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergb(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergb".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bokehres(mut self, val: i32) -> Self {
        self.params.insert(
            "bokehres".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bokehres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bokehres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_filter(mut self, val: CopBokehFilter) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_filterbokeh(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbokeh".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbokeh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbokeh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normalize(mut self, val: bool) -> Self {
        self.params.insert(
            "normalize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBokeh {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bokeh"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBoundrectSide {
    BelowThreshold = 0,
    AboveThreshold = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBoundrectGeounits {
    Image = 0,
    Texture = 1,
    Pixel = 2,
}

#[derive(Debug, Clone)]
pub struct CopBoundrect {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBoundrect {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fg(mut self, val: f32) -> Self {
        self.params.insert(
            "fg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bg(mut self, val: f32) -> Self {
        self.params.insert(
            "bg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_side(mut self, val: CopBoundrectSide) -> Self {
        self.params.insert(
            "side".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_side_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "side".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geounits(mut self, val: CopBoundrectGeounits) -> Self {
        self.params.insert(
            "geounits".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geounits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geounits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBoundrect {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "boundrect"
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

#[derive(Debug, Clone)]
pub struct CopBright {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBright {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bright(mut self, val: f32) -> Self {
        self.params.insert(
            "bright".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "shift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_brighttint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "brighttint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_brighttint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "brighttint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shifttint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shifttint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shifttint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shifttint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_scalebytimeinc(mut self, val: bool) -> Self {
        self.params.insert(
            "scalebytimeinc".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scalebytimeinc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalebytimeinc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBright {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bright"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopBubblenoiseElementsizetype {
    /// Per-Component Control
    PerMinusComponentControl = 0,
}

#[derive(Debug, Clone)]
pub struct CopBubblenoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl CopBubblenoise {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_center(mut self, val: f32) -> Self {
        self.params.insert(
            "center".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lac(mut self, val: f32) -> Self {
        self.params.insert(
            "lac".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lac".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distort(mut self, val: f32) -> Self {
        self.params.insert(
            "distort".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distort".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_droop(mut self, val: f32) -> Self {
        self.params.insert(
            "droop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_droop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "post_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_gain(mut self, val: f32) -> Self {
        self.params.insert(
            "post_gain".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_gain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "post_gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_gamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "post_contrast".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_contrast".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_minimum(mut self, val: f32) -> Self {
        self.params.insert(
            "post_minimum".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_minimum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_minimum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_maximum(mut self, val: f32) -> Self {
        self.params.insert(
            "post_maximum".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_post_maximum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_maximum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_elementscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "elementscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_elementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tilesize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "tilesize".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_tilesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tilesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stretch(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "stretch".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stretch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_droopdir(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "droopdir".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_droopdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droopdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_phase(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "phase".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "phase".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_max_octaves(mut self, val: i32) -> Self {
        self.params.insert(
            "max_octaves".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_max_octaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_octaves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_elementsizetype(mut self, val: CopBubblenoiseElementsizetype) -> Self {
        self.params.insert(
            "elementsizetype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_elementsizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsizetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dodroop(mut self, val: bool) -> Self {
        self.params.insert(
            "dodroop".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodroop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodroop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dofold(mut self, val: bool) -> Self {
        self.params.insert(
            "dofold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dofold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dofold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_docomplement(mut self, val: bool) -> Self {
        self.params.insert(
            "post_docomplement".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_docomplement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_docomplement".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_dobias(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dobias".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dobias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dobias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_dogain(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dogain".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dogain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dogain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_dogamma(mut self, val: bool) -> Self {
        self.params.insert(
            "post_dogamma".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_dogamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_dogamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_docontrast(mut self, val: bool) -> Self {
        self.params.insert(
            "post_docontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_docontrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_docontrast".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_doclampmin(mut self, val: bool) -> Self {
        self.params.insert(
            "post_doclampmin".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_doclampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_doclampmin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_post_doclampmax(mut self, val: bool) -> Self {
        self.params.insert(
            "post_doclampmax".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_post_doclampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "post_doclampmax".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for CopBubblenoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "bubblenoise"
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
