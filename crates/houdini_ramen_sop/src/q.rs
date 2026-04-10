#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshOutput {
    ExtractedMesh = 0,
    GlobalParameterization = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshResolution {
    QuadCount = 0,
    QuadArea = 1,
    Tolerance = 2,
    RelativeScale = 3,
    AbsoluteScale = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshResolutionsource {
    Auto = 0,
    TriangleArea = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshXdirection {
    /// Positive X-axis
    PositiveXMinusAxis = 0,
    /// Negative X-axis
    NegativeXMinusAxis = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshYdirection {
    /// Positive Y-axis
    PositiveYMinusAxis = 0,
    /// Negative Y-axis
    NegativeYMinusAxis = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshZdirection {
    /// Positive Z-axis
    PositiveZMinusAxis = 0,
    /// Negative Z-axis
    NegativeZMinusAxis = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshField {
    Face = 0,
    Edge = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshBoundarymode {
    Add = 0,
    Over = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopQuadremeshGuidemode {
    Add = 0,
    Over = 1,
}

#[derive(Debug, Clone)]
pub struct SopQuadremesh {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl SopQuadremesh {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_geometry_to_remesh_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input1".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_reference_for_edge_flows_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input2".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_targetquadarea(mut self, val: f32) -> Self {
        self.params.insert(
            "targetquadarea".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetquadarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetquadarea".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettolerance(mut self, val: f32) -> Self {
        self.params.insert(
            "targettolerance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targettolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettolerance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trianglearea(mut self, val: f32) -> Self {
        self.params.insert(
            "trianglearea".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trianglearea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trianglearea".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivityweight(mut self, val: f32) -> Self {
        self.params.insert(
            "adaptivityweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adaptivityweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivityweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivitysizingweight(mut self, val: f32) -> Self {
        self.params.insert(
            "adaptivitysizingweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adaptivitysizingweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitysizingweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_globalweight(mut self, val: f32) -> Self {
        self.params.insert(
            "globalweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localcurvatureweight(mut self, val: f32) -> Self {
        self.params.insert(
            "localcurvatureweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localcurvatureweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localcurvatureweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturerotation(mut self, val: f32) -> Self {
        self.params.insert(
            "curvaturerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturerotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localboundaryweight(mut self, val: f32) -> Self {
        self.params.insert(
            "localboundaryweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localboundaryweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localboundaryweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_boundaryrotation(mut self, val: f32) -> Self {
        self.params.insert(
            "boundaryrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_boundaryrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundaryrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localguideweight(mut self, val: f32) -> Self {
        self.params.insert(
            "localguideweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localguideweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localguideweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_symmetrycenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "symmetrycenter".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_symmetrycenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symmetrycenter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_decimationlevel(mut self, val: i32) -> Self {
        self.params.insert(
            "decimationlevel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_decimationlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "decimationlevel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetquadcount(mut self, val: i32) -> Self {
        self.params.insert(
            "targetquadcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_targetquadcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetquadcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolutionscale(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolutionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_output(mut self, val: SopQuadremeshOutput) -> Self {
        self.params.insert(
            "output".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolution(mut self, val: SopQuadremeshResolution) -> Self {
        self.params.insert(
            "resolution".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolutionsource(mut self, val: SopQuadremeshResolutionsource) -> Self {
        self.params.insert(
            "resolutionsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolutionsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xdirection(mut self, val: SopQuadremeshXdirection) -> Self {
        self.params.insert(
            "xdirection".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xdirection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ydirection(mut self, val: SopQuadremeshYdirection) -> Self {
        self.params.insert(
            "ydirection".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ydirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ydirection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zdirection(mut self, val: SopQuadremeshZdirection) -> Self {
        self.params.insert(
            "zdirection".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_zdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zdirection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_field(mut self, val: SopQuadremeshField) -> Self {
        self.params.insert(
            "field".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_field_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "field".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_boundarymode(mut self, val: SopQuadremeshBoundarymode) -> Self {
        self.params.insert(
            "boundarymode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundarymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundarymode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidemode(mut self, val: SopQuadremeshGuidemode) -> Self {
        self.params.insert(
            "guidemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivitymaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "adaptivitymaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivitymaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitymaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivitysizingattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "adaptivitysizingattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivitysizingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitysizingattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_globalmask(mut self, val: &str) -> Self {
        self.params.insert(
            "globalmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_globalmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturemaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "curvaturemaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvaturemaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturemaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_boundarymaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "boundarymaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_boundarymaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundarymaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidemaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "guidemaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidemaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidemaskattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "guideattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_decimation(mut self, val: bool) -> Self {
        self.params.insert(
            "decimation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_decimation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "decimation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "xaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "yaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_yaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "zaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_zaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirroroutput(mut self, val: bool) -> Self {
        self.params.insert(
            "mirroroutput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirroroutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirroroutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableadaptivity(mut self, val: bool) -> Self {
        self.params.insert(
            "enableadaptivity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableadaptivity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableadaptivity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adaptivitysizing(mut self, val: bool) -> Self {
        self.params.insert(
            "adaptivitysizing".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptivitysizing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitysizing".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_featureboundaries(mut self, val: bool) -> Self {
        self.params.insert(
            "featureboundaries".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_featureboundaries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "featureboundaries".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_curvature(mut self, val: bool) -> Self {
        self.params.insert(
            "curvature".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_curvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_boundary(mut self, val: bool) -> Self {
        self.params.insert(
            "boundary".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boundary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guide(mut self, val: bool) -> Self {
        self.params.insert(
            "guide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for SopQuadremesh {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "quadremesh"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait SopQuadremeshOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl SopQuadremeshOutputs for SopQuadremesh {}
impl SopQuadremeshOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<SopQuadremesh> {}

pub trait SopQuadremeshWiringExt {
    fn set_geometry_to_remesh_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_reference_for_edge_flows_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
    fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self;
}

impl<'a, 'g, C> SopQuadremeshWiringExt
    for houdini_ramen_core::graph::NodeWiring<'a, 'g, C, SopQuadremesh>
{
    fn set_geometry_to_remesh_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(0, output)
    }
    fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input1", output)
    }
    fn set_reference_for_edge_flows_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_at(1, output)
    }
    fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        self,
        output: O,
    ) -> Self {
        self.set_input_name("input2", output)
    }
}
