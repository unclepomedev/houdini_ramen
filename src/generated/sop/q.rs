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
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopQuadremesh {
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

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Geometry to Remesh"
    pub fn set_input_geometry_to_remesh<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Geometry to Remesh" and specifies the output index of the target node.
    pub fn set_input_geometry_to_remesh_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_geometry_to_remesh_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Reference For Edge Flows"
    pub fn set_input_reference_for_edge_flows<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Reference For Edge Flows" and specifies the output index of the target node.
    pub fn set_input_reference_for_edge_flows_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_reference_for_edge_flows_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_targetquadarea(mut self, val: f32) -> Self {
        self.params.insert(
            "targetquadarea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetquadarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetquadarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targettolerance(mut self, val: f32) -> Self {
        self.params.insert(
            "targettolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targettolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trianglearea(mut self, val: f32) -> Self {
        self.params.insert(
            "trianglearea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trianglearea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trianglearea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivityweight(mut self, val: f32) -> Self {
        self.params.insert(
            "adaptivityweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adaptivityweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivityweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivitysizingweight(mut self, val: f32) -> Self {
        self.params.insert(
            "adaptivitysizingweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adaptivitysizingweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitysizingweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalweight(mut self, val: f32) -> Self {
        self.params.insert(
            "globalweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localcurvatureweight(mut self, val: f32) -> Self {
        self.params.insert(
            "localcurvatureweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localcurvatureweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localcurvatureweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvaturerotation(mut self, val: f32) -> Self {
        self.params.insert(
            "curvaturerotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturerotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturerotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localboundaryweight(mut self, val: f32) -> Self {
        self.params.insert(
            "localboundaryweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localboundaryweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localboundaryweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundaryrotation(mut self, val: f32) -> Self {
        self.params.insert(
            "boundaryrotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_boundaryrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundaryrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localguideweight(mut self, val: f32) -> Self {
        self.params.insert(
            "localguideweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localguideweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localguideweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_symmetrycenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "symmetrycenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_symmetrycenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "symmetrycenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_decimationlevel(mut self, val: i32) -> Self {
        self.params.insert(
            "decimationlevel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_decimationlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "decimationlevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetquadcount(mut self, val: i32) -> Self {
        self.params.insert(
            "targetquadcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_targetquadcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetquadcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolutionscale(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolutionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: SopQuadremeshOutput) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolution(mut self, val: SopQuadremeshResolution) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolutionsource(mut self, val: SopQuadremeshResolutionsource) -> Self {
        self.params.insert(
            "resolutionsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolutionsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xdirection(mut self, val: SopQuadremeshXdirection) -> Self {
        self.params.insert(
            "xdirection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xdirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ydirection(mut self, val: SopQuadremeshYdirection) -> Self {
        self.params.insert(
            "ydirection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ydirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ydirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zdirection(mut self, val: SopQuadremeshZdirection) -> Self {
        self.params.insert(
            "zdirection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_zdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zdirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_field(mut self, val: SopQuadremeshField) -> Self {
        self.params.insert(
            "field".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_field_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "field".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundarymode(mut self, val: SopQuadremeshBoundarymode) -> Self {
        self.params.insert(
            "boundarymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundarymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundarymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidemode(mut self, val: SopQuadremeshGuidemode) -> Self {
        self.params.insert(
            "guidemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivitymaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "adaptivitymaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_adaptivitymaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitymaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivitysizingattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "adaptivitysizingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_adaptivitysizingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitysizingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalmask(mut self, val: &str) -> Self {
        self.params.insert(
            "globalmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_globalmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvaturemaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "curvaturemaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curvaturemaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturemaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundarymaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "boundarymaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundarymaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundarymaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidemaskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "guidemaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidemaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidemaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "guideattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_decimation(mut self, val: bool) -> Self {
        self.params.insert(
            "decimation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_decimation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "decimation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_yaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_zaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mirroroutput(mut self, val: bool) -> Self {
        self.params.insert(
            "mirroroutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirroroutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirroroutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableadaptivity(mut self, val: bool) -> Self {
        self.params.insert(
            "enableadaptivity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableadaptivity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableadaptivity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivitysizing(mut self, val: bool) -> Self {
        self.params.insert(
            "adaptivitysizing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptivitysizing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivitysizing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_featureboundaries(mut self, val: bool) -> Self {
        self.params.insert(
            "featureboundaries".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_featureboundaries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "featureboundaries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvature(mut self, val: bool) -> Self {
        self.params.insert(
            "curvature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_curvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary(mut self, val: bool) -> Self {
        self.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boundary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide(mut self, val: bool) -> Self {
        self.params.insert(
            "guide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopQuadremesh {
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
