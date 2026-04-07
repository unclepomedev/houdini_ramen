#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternPatterntype {
    StackBond = 0,
    HeaderBond = 1,
    StretcherBond = 2,
    FlemishBond = 3,
    AmericanBond = 4,
    MonkBond = 5,
    EnglishBond = 6,
    DoubleEnglishBond = 7,
    Windmill = 8,
    CrossHatch = 9,
    Basketwave = 10,
    Hopscotch = 11,
    /// Herringbone N:1
    HerringboneN1 = 12,
    /// Herringbone 3:2
    Herringbone32 = 13,
    FrenchPattern = 14,
    CustomPattern = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternPatternmode {
    Seamless = 0,
    TileCount = 1,
    TileSize = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternShapeclass {
    Basic = 0,
    Marker = 1,
    Compound = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternDraw {
    Filled = 0,
    Distance = 1,
    Gradient = 2,
    Outline = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternGradienttype {
    Horizontal = 0,
    Vertical = 1,
    Radial = 2,
    Concentric = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternFilter {
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
pub enum CopTilepatternBlend {
    /// Depth-Sorted Over
    DepthMinusSortedOver = 0,
    /// Unsorted Depth-Weighted Over
    UnsortedDepthMinusWeightedOver = 1,
    Add = 2,
    Subtract = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternValueMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternRowslideoffsetMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternRotateMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternScaleMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternHscaleMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternVscaleMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternInsetMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternTileorder {
    Default = 0,
    Reverse = 1,
    Random = 2,
    Shift = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternNsplitsMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTilepatternSplitjitterMode {
    Uniform = 0,
    SetVarying = 1,
}

#[derive(Debug, Clone)]
pub struct CopTilepattern {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTilepattern {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_stackbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "stackbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stackbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stackbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stackbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "stackbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stackbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stackbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headerbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "headerbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headerbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headerbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headerbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "headerbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headerbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headerbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretcherbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "stretcherbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretcherbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretcherbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretcherbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "stretcherbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretcherbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretcherbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flemishbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "flemishbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flemishbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flemishbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flemishbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "flemishbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flemishbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flemishbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_americanbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "americanbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_americanbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "americanbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_americanbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "americanbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_americanbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "americanbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_monkbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "monkbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_monkbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "monkbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_monkbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "monkbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_monkbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "monkbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_englishbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "englishbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_englishbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_englishbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "englishbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_englishbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denglishbond_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "denglishbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_denglishbond_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denglishbond_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denglishbond_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "denglishbond_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_denglishbond_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denglishbond_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windmill_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "windmill_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windmill_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windmill_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windmill_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "windmill_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windmill_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windmill_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windmill_iris(mut self, val: f32) -> Self {
        self.params.insert(
            "windmill_iris".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windmill_iris_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windmill_iris".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "crosshatch_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_crosshatch_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "crosshatch_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_crosshatch_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "basketwave_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basketwave_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "basketwave_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basketwave_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_rowthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "basketwave_rowthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basketwave_rowthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_rowthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_colthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "basketwave_colthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basketwave_colthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_colthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hopscotch_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "hopscotch_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hopscotch_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hopscotch_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hopscotch_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "hopscotch_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hopscotch_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hopscotch_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbonen1_tilesize(mut self, val: f32) -> Self {
        self.params.insert(
            "herringbonen1_tilesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_herringbonen1_tilesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbonen1_tilesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbone32_tilesize(mut self, val: f32) -> Self {
        self.params.insert(
            "herringbone32_tilesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_herringbone32_tilesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbone32_tilesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frenchpattern_tilewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "frenchpattern_tilewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frenchpattern_tilewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frenchpattern_tilewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frenchpattern_tileheight(mut self, val: f32) -> Self {
        self.params.insert(
            "frenchpattern_tileheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frenchpattern_tileheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frenchpattern_tileheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowoffset_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("rowoffset{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowoffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rowoffset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowslideoffset_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("rowslideoffset{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowslideoffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rowslideoffset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowheight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("rowheight{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowheight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rowheight{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colwidth_inst(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.params.insert(
            format!("colwidth{}_{}", index1, index2),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colwidth_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colwidth{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colheight_inst(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.params.insert(
            format!("colheight{}_{}", index1, index2),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colheight_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colheight{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coloffset_inst(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.params.insert(
            format!("coloffset{}_{}", index1, index2),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coloffset_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("coloffset{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colrot_inst(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.params.insert(
            format!("colrot{}_{}", index1, index2),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colrot_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colrot{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "circle_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_circle_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_arc(mut self, val: f32) -> Self {
        self.params.insert(
            "circle_arc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_circle_arc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_arc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "circle_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_circle_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_length(mut self, val: f32) -> Self {
        self.params.insert(
            "line_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_line_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_sthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "line_sthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_line_sthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_sthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_ethickness(mut self, val: f32) -> Self {
        self.params.insert(
            "line_ethickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_line_ethickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_ethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_bulge(mut self, val: f32) -> Self {
        self.params.insert(
            "line_bulge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_line_bulge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_bulge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_roundness(mut self, val: f32) -> Self {
        self.params.insert(
            "rect_roundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rect_roundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_roundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_tlroundness(mut self, val: f32) -> Self {
        self.params.insert(
            "rect_tlroundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rect_tlroundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_tlroundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_trroundness(mut self, val: f32) -> Self {
        self.params.insert(
            "rect_trroundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rect_trroundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_trroundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_blroundness(mut self, val: f32) -> Self {
        self.params.insert(
            "rect_blroundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rect_blroundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_blroundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_brroundness(mut self, val: f32) -> Self {
        self.params.insert(
            "rect_brroundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rect_brroundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_brroundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regpolygon_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "regpolygon_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_regpolygon_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regpolygon_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spiral_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "spiral_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spiral_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spiral_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spiral_arclen(mut self, val: f32) -> Self {
        self.params.insert(
            "spiral_arclen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spiral_arclen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spiral_arclen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squircle_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "squircle_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_squircle_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "squircle_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squircle_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "squircle_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_squircle_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "squircle_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_star_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "star_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_star_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "star_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_star_anglefactor(mut self, val: f32) -> Self {
        self.params.insert(
            "star_anglefactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_star_anglefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "star_anglefactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trapezoid_height(mut self, val: f32) -> Self {
        self.params.insert(
            "trapezoid_height".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trapezoid_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trapezoid_height".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trapezoid_blength(mut self, val: f32) -> Self {
        self.params.insert(
            "trapezoid_blength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trapezoid_blength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trapezoid_blength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trapezoid_tlength(mut self, val: f32) -> Self {
        self.params.insert(
            "trapezoid_tlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trapezoid_tlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trapezoid_tlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangle_width(mut self, val: f32) -> Self {
        self.params.insert(
            "triangle_width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_triangle_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangle_width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangle_height(mut self, val: f32) -> Self {
        self.params.insert(
            "triangle_height".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_triangle_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangle_height".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circlewave_arc(mut self, val: f32) -> Self {
        self.params.insert(
            "circlewave_arc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_circlewave_arc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circlewave_arc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circlewave_freq(mut self, val: f32) -> Self {
        self.params.insert(
            "circlewave_freq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_circlewave_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circlewave_freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_asterisk_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_asterisk_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_asterisk_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_asterisk_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_check_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_check_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_check_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_check_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_chevron_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_chevron_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_chevron_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_chevron_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_clobber_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_clobber_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_clobber_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_clobber_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_cross_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_cross_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_cross_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_cross_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_infinity_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_infinity_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_infinity_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_infinity_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_ring_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_ring_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_ring_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_ring_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_tag_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_tag_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_tag_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_tag_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_marker_t_size(mut self, val: f32) -> Self {
        self.params.insert(
            "marker_t_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_marker_t_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "marker_t_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_archway_width(mut self, val: f32) -> Self {
        self.params.insert(
            "archway_width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_archway_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "archway_width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_archway_height(mut self, val: f32) -> Self {
        self.params.insert(
            "archway_height".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_archway_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "archway_height".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_length(mut self, val: f32) -> Self {
        self.params.insert(
            "arrow_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_arrow_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_shaft_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "arrow_shaft_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_arrow_shaft_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_shaft_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_head_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "arrow_head_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_arrow_head_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_head_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_head_ratio(mut self, val: f32) -> Self {
        self.params.insert(
            "arrow_head_ratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_arrow_head_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_head_ratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cross_size(mut self, val: f32) -> Self {
        self.params.insert(
            "cross_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cross_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cross_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cross_inset(mut self, val: f32) -> Self {
        self.params.insert(
            "cross_inset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cross_inset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cross_inset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cross_bevel(mut self, val: f32) -> Self {
        self.params.insert(
            "cross_bevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cross_bevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cross_bevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_egg_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "egg_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_egg_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "egg_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_egg_roundness(mut self, val: f32) -> Self {
        self.params.insert(
            "egg_roundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_egg_roundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "egg_roundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fishscale_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "fishscale_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fishscale_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fishscale_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horseshoe_size(mut self, val: f32) -> Self {
        self.params.insert(
            "horseshoe_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horseshoe_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horseshoe_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horseshoe_separation(mut self, val: f32) -> Self {
        self.params.insert(
            "horseshoe_separation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horseshoe_separation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horseshoe_separation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horseshoe_length(mut self, val: f32) -> Self {
        self.params.insert(
            "horseshoe_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horseshoe_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horseshoe_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horseshoe_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "horseshoe_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horseshoe_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horseshoe_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_moon_outter_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "moon_outter_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_moon_outter_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "moon_outter_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_moon_inner_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "moon_inner_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_moon_inner_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "moon_inner_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_moon_inner_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "moon_inner_offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_moon_inner_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "moon_inner_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_octagondot_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "octagondot_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_octagondot_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "octagondot_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roundedx_size(mut self, val: f32) -> Self {
        self.params.insert(
            "roundedx_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roundedx_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundedx_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roundedx_width(mut self, val: f32) -> Self {
        self.params.insert(
            "roundedx_width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roundedx_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundedx_width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vesica_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "vesica_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vesica_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vesica_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vesica_roundness(mut self, val: f32) -> Self {
        self.params.insert(
            "vesica_roundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vesica_roundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vesica_roundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fadedist(mut self, val: f32) -> Self {
        self.params.insert(
            "fadedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fadedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fadedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradientrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "gradientrotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradientrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gradientrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlinewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "outlinewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outlinewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlinewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutout_edgefilter(mut self, val: f32) -> Self {
        self.params.insert(
            "cutout_edgefilter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cutout_edgefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutout_edgefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randstamporderseed(mut self, val: f32) -> Self {
        self.params.insert(
            "randstamporderseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randstamporderseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randstamporderseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value(mut self, val: f32) -> Self {
        self.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_var(mut self, val: f32) -> Self {
        self.params.insert(
            "value_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "value_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgvalue(mut self, val: f32) -> Self {
        self.params.insert(
            "bgvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bgvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bgvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "rowoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rowoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coloffset(mut self, val: f32) -> Self {
        self.params.insert(
            "coloffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coloffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coloffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowslideoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "rowslideoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowslideoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rowslideoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowslideoffset_var(mut self, val: f32) -> Self {
        self.params.insert(
            "rowslideoffset_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowslideoffset_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rowslideoffset_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowslideoffset_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "rowslideoffset_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rowslideoffset_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rowslideoffset_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posjitter(mut self, val: f32) -> Self {
        self.params.insert(
            "posjitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_posjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hposjitter(mut self, val: f32) -> Self {
        self.params.insert(
            "hposjitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hposjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hposjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vposjitter(mut self, val: f32) -> Self {
        self.params.insert(
            "vposjitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vposjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vposjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posjitterseed(mut self, val: f32) -> Self {
        self.params.insert(
            "posjitterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_posjitterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posjitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grotate(mut self, val: f32) -> Self {
        self.params.insert(
            "grotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate_var(mut self, val: f32) -> Self {
        self.params.insert(
            "rotate_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotate_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "rotate_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotate_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_scale_var(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hscale(mut self, val: f32) -> Self {
        self.params.insert(
            "hscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hscale_var(mut self, val: f32) -> Self {
        self.params.insert(
            "hscale_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hscale_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hscale_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hscale_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "hscale_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hscale_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hscale_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vscale_var(mut self, val: f32) -> Self {
        self.params.insert(
            "vscale_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vscale_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vscale_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "vscale_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vscale_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inset(mut self, val: f32) -> Self {
        self.params.insert(
            "inset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inset_var(mut self, val: f32) -> Self {
        self.params.insert(
            "inset_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inset_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inset_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inset_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "inset_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inset_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inset_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tilethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "tilethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tilethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tilethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thresholdrandseed(mut self, val: f32) -> Self {
        self.params.insert(
            "thresholdrandseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thresholdrandseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thresholdrandseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areathreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "areathreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_areathreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "areathreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splittreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "splittreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splittreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splittreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nsplits_var(mut self, val: f32) -> Self {
        self.params.insert(
            "nsplits_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nsplits_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nsplits_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nsplits_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "nsplits_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nsplits_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nsplits_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitjitter(mut self, val: f32) -> Self {
        self.params.insert(
            "splitjitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitjitter_var(mut self, val: f32) -> Self {
        self.params.insert(
            "splitjitter_var".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitjitter_var_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitjitter_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitjitter_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "splitjitter_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitjitter_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitjitter_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_split_orientratio(mut self, val: f32) -> Self {
        self.params.insert(
            "split_orientratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_split_orientratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "split_orientratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splittresholdseed(mut self, val: f32) -> Self {
        self.params.insert(
            "splittresholdseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splittresholdseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splittresholdseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_tiledsize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "tiledsize".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_tiledsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiledsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diamond_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diamond_size".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diamond_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diamond_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_pt0(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "line_pt0".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_line_pt0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_pt0".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_pt1(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "line_pt1".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_line_pt1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_pt1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "rect_size".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_rect_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangle_pt0(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "triangle_pt0".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_triangle_pt0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangle_pt0".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangle_pt1(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "triangle_pt1".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_triangle_pt1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangle_pt1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangle_pt2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "triangle_pt2".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_triangle_pt2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangle_pt2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_pt0(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "arrow_pt0".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_arrow_pt0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_pt0".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_pt1(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "arrow_pt1".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_arrow_pt1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_pt1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_stackbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "stackbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stackbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stackbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stackbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "stackbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stackbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stackbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stackbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "stackbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stackbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stackbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headerbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "headerbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_headerbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headerbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headerbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "headerbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_headerbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headerbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headerbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "headerbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_headerbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headerbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretcherbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "stretcherbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stretcherbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretcherbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretcherbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "stretcherbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stretcherbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretcherbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretcherbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "stretcherbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stretcherbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretcherbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flemishbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "flemishbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flemishbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flemishbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flemishbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "flemishbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flemishbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flemishbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flemishbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "flemishbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flemishbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flemishbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_americanbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "americanbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_americanbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "americanbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_americanbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "americanbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_americanbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "americanbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_americanbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "americanbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_americanbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "americanbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_monkbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "monkbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_monkbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "monkbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_monkbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "monkbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_monkbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "monkbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_monkbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "monkbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_monkbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "monkbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_englishbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "englishbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_englishbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_englishbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "englishbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_englishbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_englishbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "englishbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_englishbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denglishbond_div(mut self, val: i32) -> Self {
        self.params.insert(
            "denglishbond_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_denglishbond_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denglishbond_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denglishbond_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "denglishbond_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_denglishbond_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denglishbond_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denglishbond_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "denglishbond_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_denglishbond_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denglishbond_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windmill_div(mut self, val: i32) -> Self {
        self.params.insert(
            "windmill_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_windmill_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windmill_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windmill_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "windmill_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_windmill_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windmill_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windmill_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "windmill_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_windmill_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windmill_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_div(mut self, val: i32) -> Self {
        self.params.insert(
            "crosshatch_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_crosshatch_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "crosshatch_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_crosshatch_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "crosshatch_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_crosshatch_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_rowdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "crosshatch_rowdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_crosshatch_rowdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_rowdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_crosshatch_coldiv(mut self, val: i32) -> Self {
        self.params.insert(
            "crosshatch_coldiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_crosshatch_coldiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "crosshatch_coldiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_div(mut self, val: i32) -> Self {
        self.params.insert(
            "basketwave_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_basketwave_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "basketwave_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_basketwave_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "basketwave_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_basketwave_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hopscotch_div(mut self, val: i32) -> Self {
        self.params.insert(
            "hopscotch_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hopscotch_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hopscotch_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hopscotch_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "hopscotch_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hopscotch_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hopscotch_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hopscotch_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "hopscotch_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hopscotch_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hopscotch_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbonen1_div(mut self, val: i32) -> Self {
        self.params.insert(
            "herringbonen1_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_herringbonen1_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbonen1_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbonen1_tilecount(mut self, val: i32) -> Self {
        self.params.insert(
            "herringbonen1_tilecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_herringbonen1_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbonen1_tilecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbonen1_ration(mut self, val: i32) -> Self {
        self.params.insert(
            "herringbonen1_ration".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_herringbonen1_ration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbonen1_ration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbone32_div(mut self, val: i32) -> Self {
        self.params.insert(
            "herringbone32_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_herringbone32_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbone32_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_herringbone32_tilecount(mut self, val: i32) -> Self {
        self.params.insert(
            "herringbone32_tilecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_herringbone32_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "herringbone32_tilecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frenchpattern_div(mut self, val: i32) -> Self {
        self.params.insert(
            "frenchpattern_div".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_frenchpattern_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frenchpattern_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frenchpattern_rows(mut self, val: i32) -> Self {
        self.params.insert(
            "frenchpattern_rows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_frenchpattern_rows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frenchpattern_rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frenchpattern_cols(mut self, val: i32) -> Self {
        self.params.insert(
            "frenchpattern_cols".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_frenchpattern_cols_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frenchpattern_cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regpolygon_sides(mut self, val: i32) -> Self {
        self.params.insert(
            "regpolygon_sides".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_regpolygon_sides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regpolygon_sides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spiral_arcnum(mut self, val: i32) -> Self {
        self.params.insert(
            "spiral_arcnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spiral_arcnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spiral_arcnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_star_sides(mut self, val: i32) -> Self {
        self.params.insert(
            "star_sides".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_star_sides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "star_sides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thresholdshiftoffset(mut self, val: i32) -> Self {
        self.params.insert(
            "thresholdshiftoffset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_thresholdshiftoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thresholdshiftoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idselection(mut self, val: i32) -> Self {
        self.params.insert(
            "idselection".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_idselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowidselection(mut self, val: i32) -> Self {
        self.params.insert(
            "rowidselection".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rowidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rowidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_columnidselection(mut self, val: i32) -> Self {
        self.params.insert(
            "columnidselection".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_columnidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "columnidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nsplititers(mut self, val: i32) -> Self {
        self.params.insert(
            "nsplititers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_nsplititers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nsplititers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nsplits(mut self, val: i32) -> Self {
        self.params.insert(
            "nsplits".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_nsplits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nsplits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_patterntype(mut self, val: CopTilepatternPatterntype) -> Self {
        self.params.insert(
            "patterntype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_patterntype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "patterntype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_patternmode(mut self, val: CopTilepatternPatternmode) -> Self {
        self.params.insert(
            "patternmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_patternmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "patternmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shapeclass(mut self, val: CopTilepatternShapeclass) -> Self {
        self.params.insert(
            "shapeclass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shapeclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shapeclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_draw(mut self, val: CopTilepatternDraw) -> Self {
        self.params.insert(
            "draw".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_draw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "draw".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradienttype(mut self, val: CopTilepatternGradienttype) -> Self {
        self.params.insert(
            "gradienttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_gradienttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gradienttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: CopTilepatternFilter) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blend(mut self, val: CopTilepatternBlend) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_mode(mut self, val: CopTilepatternValueMode) -> Self {
        self.params.insert(
            "value_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_value_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowslideoffset_mode(mut self, val: CopTilepatternRowslideoffsetMode) -> Self {
        self.params.insert(
            "rowslideoffset_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rowslideoffset_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rowslideoffset_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate_mode(mut self, val: CopTilepatternRotateMode) -> Self {
        self.params.insert(
            "rotate_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rotate_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_mode(mut self, val: CopTilepatternScaleMode) -> Self {
        self.params.insert(
            "scale_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scale_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hscale_mode(mut self, val: CopTilepatternHscaleMode) -> Self {
        self.params.insert(
            "hscale_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hscale_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hscale_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vscale_mode(mut self, val: CopTilepatternVscaleMode) -> Self {
        self.params.insert(
            "vscale_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vscale_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vscale_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inset_mode(mut self, val: CopTilepatternInsetMode) -> Self {
        self.params.insert(
            "inset_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inset_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inset_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tileorder(mut self, val: CopTilepatternTileorder) -> Self {
        self.params.insert(
            "tileorder".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tileorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tileorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nsplits_mode(mut self, val: CopTilepatternNsplitsMode) -> Self {
        self.params.insert(
            "nsplits_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nsplits_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nsplits_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitjitter_mode(mut self, val: CopTilepatternSplitjitterMode) -> Self {
        self.params.insert(
            "splitjitter_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitjitter_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitjitter_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_gradientramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "gradientramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_gradientramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gradientramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_expr(mut self, val: &str) -> Self {
        self.params.insert(
            "expr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basictype(mut self, val: &str) -> Self {
        self.params.insert(
            "basictype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basictype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basictype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_markertype(mut self, val: &str) -> Self {
        self.params.insert(
            "markertype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_markertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "markertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compoundtype(mut self, val: &str) -> Self {
        self.params.insert(
            "compoundtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_compoundtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compoundtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_cliptosize(mut self, val: bool) -> Self {
        self.params.insert(
            "cliptosize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cliptosize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliptosize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basketwave_drawsquarebonds(mut self, val: bool) -> Self {
        self.params.insert(
            "basketwave_drawsquarebonds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basketwave_drawsquarebonds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basketwave_drawsquarebonds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flippattern(mut self, val: bool) -> Self {
        self.params.insert(
            "flippattern".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flippattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flippattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexpr(mut self, val: bool) -> Self {
        self.params.insert(
            "useexpr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowvisibility_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("rowvisibility{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rowvisibility_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rowvisibility{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rowreflect_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("rowreflect{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rowreflect_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rowreflect{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colvisibility_inst(mut self, index1: usize, index2: usize, val: bool) -> Self {
        self.params.insert(
            format!("colvisibility{}_{}", index1, index2),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_colvisibility_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("colvisibility{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docolheight_inst(mut self, index1: usize, index2: usize, val: bool) -> Self {
        self.params.insert(
            format!("docolheight{}_{}", index1, index2),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docolheight_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("docolheight{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docoloffset_inst(mut self, index1: usize, index2: usize, val: bool) -> Self {
        self.params.insert(
            format!("docoloffset{}_{}", index1, index2),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docoloffset_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("docoloffset{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docolrot_inst(mut self, index1: usize, index2: usize, val: bool) -> Self {
        self.params.insert(
            format!("docolrot{}_{}", index1, index2),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docolrot_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("docolrot{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaspectratio(mut self, val: bool) -> Self {
        self.params.insert(
            "useaspectratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useaspectratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_doarc(mut self, val: bool) -> Self {
        self.params.insert(
            "circle_doarc".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_circle_doarc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_doarc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_dothickness(mut self, val: bool) -> Self {
        self.params.insert(
            "circle_dothickness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_circle_dothickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_dothickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_roundends(mut self, val: bool) -> Self {
        self.params.insert(
            "circle_roundends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_circle_roundends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_roundends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_circle_quadratic(mut self, val: bool) -> Self {
        self.params.insert(
            "circle_quadratic".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_circle_quadratic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "circle_quadratic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_usepoints(mut self, val: bool) -> Self {
        self.params.insert(
            "line_usepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_line_usepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_usepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_addthickness(mut self, val: bool) -> Self {
        self.params.insert(
            "line_addthickness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_line_addthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_addthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_line_dobulge(mut self, val: bool) -> Self {
        self.params.insert(
            "line_dobulge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_line_dobulge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "line_dobulge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_round(mut self, val: bool) -> Self {
        self.params.insert(
            "rect_round".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rect_round_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_round".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rect_doseproundness(mut self, val: bool) -> Self {
        self.params.insert(
            "rect_doseproundness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rect_doseproundness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rect_doseproundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangle_usepoints(mut self, val: bool) -> Self {
        self.params.insert(
            "triangle_usepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_triangle_usepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangle_usepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrow_usepoints(mut self, val: bool) -> Self {
        self.params.insert(
            "arrow_usepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_arrow_usepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arrow_usepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doaa(mut self, val: bool) -> Self {
        self.params.insert(
            "doaa".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doaa_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doaa".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randstamporder(mut self, val: bool) -> Self {
        self.params.insert(
            "randstamporder".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randstamporder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randstamporder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doposjitter(mut self, val: bool) -> Self {
        self.params.insert(
            "doposjitter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doposjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doposjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dohposjitter(mut self, val: bool) -> Self {
        self.params.insert(
            "dohposjitter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dohposjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dohposjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovposjitter(mut self, val: bool) -> Self {
        self.params.insert(
            "dovposjitter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovposjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovposjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate_chb(mut self, val: bool) -> Self {
        self.params.insert(
            "rotate_chb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rotate_chb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate_chb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orienttiles(mut self, val: bool) -> Self {
        self.params.insert(
            "orienttiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orienttiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orienttiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doinset(mut self, val: bool) -> Self {
        self.params.insert(
            "doinset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doinset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doinset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dothresholdselection(mut self, val: bool) -> Self {
        self.params.insert(
            "dothresholdselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dothresholdselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dothresholdselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversethresholdselection(mut self, val: bool) -> Self {
        self.params.insert(
            "reversethresholdselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversethresholdselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversethresholdselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doidselection(mut self, val: bool) -> Self {
        self.params.insert(
            "doidselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reverseidselection(mut self, val: bool) -> Self {
        self.params.insert(
            "reverseidselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reverseidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverseidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorowidselection(mut self, val: bool) -> Self {
        self.params.insert(
            "dorowidselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorowidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorowidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reverserowidselection(mut self, val: bool) -> Self {
        self.params.insert(
            "reverserowidselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reverserowidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverserowidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docolumnidselection(mut self, val: bool) -> Self {
        self.params.insert(
            "docolumnidselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docolumnidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docolumnidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversecolumnidselection(mut self, val: bool) -> Self {
        self.params.insert(
            "reversecolumnidselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversecolumnidselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversecolumnidselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docheckerboard(mut self, val: bool) -> Self {
        self.params.insert(
            "docheckerboard".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docheckerboard_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docheckerboard".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversecheckerboard(mut self, val: bool) -> Self {
        self.params.insert(
            "reversecheckerboard".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversecheckerboard_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversecheckerboard".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_negate(mut self, val: bool) -> Self {
        self.params.insert(
            "negate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_negate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splittiles(mut self, val: bool) -> Self {
        self.params.insert(
            "splittiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splittiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splittiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosplititer(mut self, val: bool) -> Self {
        self.params.insert(
            "dosplititer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosplititer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosplititer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doareathreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "doareathreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doareathreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doareathreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopTilepattern {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "tilepattern"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTonemapOperator {
    Reinhard = 0,
    Ward = 1,
    AcesFilmic = 2,
    HableFilmic = 3,
    HableFilmicUpdated = 4,
    Unreal = 5,
}

#[derive(Debug, Clone)]
pub struct CopTonemap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTonemap {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "exposure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exposure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hable_shoulderstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "hable_shoulderstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hable_shoulderstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hable_shoulderstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hable_linearstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "hable_linearstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hable_linearstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hable_linearstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hable_linearangle(mut self, val: f32) -> Self {
        self.params.insert(
            "hable_linearangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hable_linearangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hable_linearangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hable_toestrength(mut self, val: f32) -> Self {
        self.params.insert(
            "hable_toestrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hable_toestrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hable_toestrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hable_whitepoint(mut self, val: f32) -> Self {
        self.params.insert(
            "hable_whitepoint".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hable_whitepoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hable_whitepoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hablefilmicupdated_toestrength(mut self, val: f32) -> Self {
        self.params.insert(
            "hablefilmicupdated_toestrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hablefilmicupdated_toestrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hablefilmicupdated_toestrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hablefilmicupdated_toelength(mut self, val: f32) -> Self {
        self.params.insert(
            "hablefilmicupdated_toelength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hablefilmicupdated_toelength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hablefilmicupdated_toelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hablefilmicupdated_shoulderstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "hablefilmicupdated_shoulderstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hablefilmicupdated_shoulderstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hablefilmicupdated_shoulderstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hablefilmicupdated_shoulderlength(mut self, val: f32) -> Self {
        self.params.insert(
            "hablefilmicupdated_shoulderlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hablefilmicupdated_shoulderlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hablefilmicupdated_shoulderlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hablefilmicupdated_shoulderangle(mut self, val: f32) -> Self {
        self.params.insert(
            "hablefilmicupdated_shoulderangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hablefilmicupdated_shoulderangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hablefilmicupdated_shoulderangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unreal_toestrength(mut self, val: f32) -> Self {
        self.params.insert(
            "unreal_toestrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unreal_toestrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unreal_toestrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unreal_shoulderstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "unreal_shoulderstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unreal_shoulderstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unreal_shoulderstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unreal_slope(mut self, val: f32) -> Self {
        self.params.insert(
            "unreal_slope".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unreal_slope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unreal_slope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unreal_g(mut self, val: f32) -> Self {
        self.params.insert(
            "unreal_g".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unreal_g_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unreal_g".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_operator(mut self, val: CopTonemapOperator) -> Self {
        self.params.insert(
            "operator".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operator_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operator".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_ispremult(mut self, val: bool) -> Self {
        self.params.insert(
            "ispremult".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ispremult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ispremult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopTonemap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "tonemap"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTopnetCheckpointformat {
    /// Python (Deprecated)
    PythonDeprecated = 0,
    Json = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTopnetCheckpointload {
    Never = 0,
    OnSceneLoad = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTopnetRegenerationtype {
    UpdateWorkItemsAndInvalidateCaches = 0,
    UpdateWorkItemsOnly = 1,
    IgnoreChanges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTopnetEvaluationtime {
    NetworkCookTime = 0,
    GlobalStartTime = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct CopTopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTopnet {
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
    pub fn with_checkpointformat(mut self, val: CopTopnetCheckpointformat) -> Self {
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
    pub fn with_checkpointload(mut self, val: CopTopnetCheckpointload) -> Self {
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
    pub fn with_regenerationtype(mut self, val: CopTopnetRegenerationtype) -> Self {
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
    pub fn with_evaluationtime(mut self, val: CopTopnetEvaluationtime) -> Self {
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

impl crate::core::types::HoudiniNode for CopTopnet {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTriplanarTexturetype {
    UniformTexture = 0,
    TexturePerAxis = 1,
}

#[derive(Debug, Clone)]
pub struct CopTriplanar {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTriplanar {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            4,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_4_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            5,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            5,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_5_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            5,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_globalscale(mut self, val: f32) -> Self {
        self.params.insert(
            "globalscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "globalrotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xscale(mut self, val: f32) -> Self {
        self.params.insert(
            "xscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "xrotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yscale(mut self, val: f32) -> Self {
        self.params.insert(
            "yscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "yrotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zscale(mut self, val: f32) -> Self {
        self.params.insert(
            "zscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zrotate(mut self, val: f32) -> Self {
        self.params.insert(
            "zrotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triplanarblend(mut self, val: f32) -> Self {
        self.params.insert(
            "triplanarblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_triplanarblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triplanarblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_globaloffset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "globaloffset".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_globaloffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globaloffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xoffset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "xoffset".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_xoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yoffset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "yoffset".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_yoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zoffset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "zoffset".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_zoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_texturetype(mut self, val: CopTriplanarTexturetype) -> Self {
        self.params.insert(
            "texturetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_texturetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texturetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_xzramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "xzramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_xzramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xzramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "yramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_yramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopTriplanar {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "triplanar"
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

#[derive(Debug, Clone)]
pub struct CopTriplanarhextile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTriplanarhextile {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_3_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            4,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_4_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            5,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            5,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_5_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            5,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            6,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            6,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_6_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            6,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            7,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            7,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_7_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            7,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            8,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            8,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_8_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            8,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            9,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            9,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_9_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            9,
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
    pub fn with_scalerand(mut self, val: f32) -> Self {
        self.params.insert(
            "scalerand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalerand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalerand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rot(mut self, val: f32) -> Self {
        self.params.insert(
            "rot".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotrand(mut self, val: f32) -> Self {
        self.params.insert(
            "rotrand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotrand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotrand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contrast_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "contrast_falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrast_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrast_falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightexp(mut self, val: f32) -> Self {
        self.params.insert(
            "weightexp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weightexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalex(mut self, val: f32) -> Self {
        self.params.insert(
            "scalex".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotx(mut self, val: f32) -> Self {
        self.params.insert(
            "rotx".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaley(mut self, val: f32) -> Self {
        self.params.insert(
            "scaley".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scaley_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaley".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roty(mut self, val: f32) -> Self {
        self.params.insert(
            "roty".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalez(mut self, val: f32) -> Self {
        self.params.insert(
            "scalez".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalez_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalez".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotz(mut self, val: f32) -> Self {
        self.params.insert(
            "rotz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offsetx(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offsetx".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offsetx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offsetx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offsety(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offsety".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offsety_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offsety".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offsetz(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offsetz".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offsetz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offsetz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_xzblend(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "xzblend".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_xzblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xzblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yblend(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "yblend".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_yblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopTriplanarhextile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "triplanarhextile"
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

#[derive(Debug, Clone)]
pub struct CopTriplanaruv {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTriplanaruv {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopTriplanaruv {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "triplanaruv"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopTwowayswitchifwiredRule {
    FirstInputIfWired = 0,
    SecondInputIfWired = 1,
}

#[derive(Debug, Clone)]
pub struct CopTwowayswitchifwired {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopTwowayswitchifwired {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_0_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_2_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Menu parameters ---
    pub fn with_rule(mut self, val: CopTwowayswitchifwiredRule) -> Self {
        self.params.insert(
            "rule".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rule".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopTwowayswitchifwired {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "twowayswitchifwired"
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
