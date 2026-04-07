#[derive(Debug, Clone)]
pub struct SopTableimport {
    pub base: crate::core::types::NodeBase,
}

impl SopTableimport {
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
    pub fn trigger_autofillattribs(mut self) -> Self {
        self.base.params.insert(
            "autofillattribs".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxrows(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxrows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxrows_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxrows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skipfirst(mut self, val: i32) -> Self {
        self.base.params.insert(
            "skipfirst".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_skipfirst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skipfirst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxissues(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxissues".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxissues_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxissues".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_column_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("column{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_column_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("column{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attriblen_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("attriblen{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_attriblen_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attriblen{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_encoding(mut self, val: &str) -> Self {
        self.base.params.insert(
            "encoding".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_encoding_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "encoding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parseissues(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parseissues".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parseissues_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parseissues".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customdelimiter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "customdelimiter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customdelimiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customdelimiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translator_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("translator{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_translator_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("translator{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translateformat_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("translateformat{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_translateformat_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("translateformat{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("attribname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribtype_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("attribtype{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribtype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usemaxrows(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxrows".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxrows_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxrows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useencoding(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useencoding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useencoding_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useencoding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecustomdelimiter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecustomdelimiter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomdelimiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecustomdelimiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTableimport {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tableimport"
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
pub enum SopTangentfieldCarrier {
    Points = 0,
    Faces = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTangentfieldBoundarymode {
    Add = 0,
    Over = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTangentfieldGuidemode {
    Add = 0,
    Over = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTangentfieldOutputmode {
    AllDirections = 0,
    OneDirectionPerSymmetricPair = 1,
    OneDirectionOnly = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTangentfieldFieldscalemode {
    PrescaledForViewing = 0,
    Normalized = 1,
    None = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTangentfieldGuidescalemode {
    PrescaledForViewing = 0,
    Normalized = 1,
    None = 2,
}

#[derive(Debug, Clone)]
pub struct SopTangentfield {
    pub base: crate::core::types::NodeBase,
}

impl SopTangentfield {
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

    /// Connects to input 0: "Geometry to Generate Over"
    pub fn set_input_geometry_to_generate_over(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Generate Over" and specifies the output index of the target node.
    pub fn set_input_geometry_to_generate_over_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rotation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "globalweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "globalweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localcurvatureweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "localcurvatureweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localcurvatureweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localcurvatureweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvaturerotation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "curvaturerotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvaturerotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "curvaturerotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localboundaryweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "localboundaryweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localboundaryweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localboundaryweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundaryrotation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "boundaryrotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_boundaryrotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundaryrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localguideweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "localguideweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_localguideweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localguideweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropyweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "anisotropyweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisotropyweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropyweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropycurvatureweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "anisotropycurvatureweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisotropycurvatureweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropycurvatureweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropyguideweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "anisotropyguideweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisotropyguideweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropyguideweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropysizingweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "anisotropysizingweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisotropysizingweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropysizingweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fieldscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fieldscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fieldscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fieldscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropyscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "anisotropyscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisotropyscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropyscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_directions(mut self, val: i32) -> Self {
        self.base.params.insert(
            "directions".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_directions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "directions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_carrier(mut self, val: SopTangentfieldCarrier) -> Self {
        self.base.params.insert(
            "carrier".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_carrier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "carrier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundarymode(mut self, val: SopTangentfieldBoundarymode) -> Self {
        self.base.params.insert(
            "boundarymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundarymode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundarymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidemode(mut self, val: SopTangentfieldGuidemode) -> Self {
        self.base.params.insert(
            "guidemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputmode(mut self, val: SopTangentfieldOutputmode) -> Self {
        self.base.params.insert(
            "outputmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fieldscalemode(mut self, val: SopTangentfieldFieldscalemode) -> Self {
        self.base.params.insert(
            "fieldscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fieldscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fieldscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidescalemode(mut self, val: SopTangentfieldGuidescalemode) -> Self {
        self.base.params.insert(
            "guidescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidescalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidescalemode".to_string(),
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
    pub fn with_globalmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "globalmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_globalmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "globalmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvaturemaskattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "curvaturemaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curvaturemaskattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "curvaturemaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundarymaskattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "boundarymaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundarymaskattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundarymaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidemaskattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidemaskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidemaskattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidemaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guideattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropymask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "anisotropymask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisotropymask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropymask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropycurvaturemask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "anisotropycurvaturemask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisotropycurvaturemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropycurvaturemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropyguidemask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "anisotropyguidemask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisotropyguidemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropyguidemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropyguideattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "anisotropyguideattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisotropyguideattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropyguideattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropysizingmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "anisotropysizingmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisotropysizingmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropysizingmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropysizingattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "anisotropysizingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisotropysizingattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropysizingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fieldattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fieldattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fieldattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fieldattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singulargroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "singulargroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_singulargroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singulargroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positivesingulargroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "positivesingulargroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positivesingulargroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "positivesingulargroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_negativesingulargroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "negativesingulargroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_negativesingulargroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "negativesingulargroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_discontinuities(mut self, val: &str) -> Self {
        self.base.params.insert(
            "discontinuities".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_discontinuities_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "discontinuities".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_curvature(mut self, val: bool) -> Self {
        self.base.params.insert(
            "curvature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_curvature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "curvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary(mut self, val: bool) -> Self {
        self.base.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boundary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropycurvature(mut self, val: bool) -> Self {
        self.base.params.insert(
            "anisotropycurvature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_anisotropycurvature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropycurvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropyguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "anisotropyguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_anisotropyguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropyguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisotropysizing(mut self, val: bool) -> Self {
        self.base.params.insert(
            "anisotropysizing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_anisotropysizing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anisotropysizing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalizefield(mut self, val: bool) -> Self {
        self.base.params.insert(
            "normalizefield".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalizefield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normalizefield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesingulargroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesingulargroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesingulargroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesingulargroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepositivesingulargroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepositivesingulargroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepositivesingulargroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepositivesingulargroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenegativesingulargroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usenegativesingulargroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenegativesingulargroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usenegativesingulargroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usediscontinuitiesgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usediscontinuitiesgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usediscontinuitiesgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usediscontinuitiesgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showfield(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showfield".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showfield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showsingularities(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showsingularities".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showsingularities_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showsingularities".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguides(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguides".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguides_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showanisotropy(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showanisotropy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showanisotropy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showanisotropy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTangentfield {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tangentfield"
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
pub enum SopTestgeometryCapybaraOutput {
    SkinSurface = 0,
    MuscleSurfaces = 1,
    BoneSurfaces = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestgeometryCapybaraClip {
    Walk = 0,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryCapybara {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryCapybara {
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
    pub fn with_frame(mut self, val: f32) -> Self {
        self.base.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frame".to_string(),
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

    // --- Int parameters ---
    pub fn with_numloops(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numloops".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numloops_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numloops".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: SopTestgeometryCapybaraOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clip(mut self, val: SopTestgeometryCapybaraClip) -> Self {
        self.base.params.insert(
            "clip".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addshader(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addnormalmap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addnormalmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addnormalmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addnormalmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss_enable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sss_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_enable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sss_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputclip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputclip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputclip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputclip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applylocomotion(mut self, val: bool) -> Self {
        self.base.params.insert(
            "applylocomotion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applylocomotion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "applylocomotion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryCapybara {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_capybara"
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
pub struct SopTestgeometryCrag {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryCrag {
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
    pub fn with_frame(mut self, val: f32) -> Self {
        self.base.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frame".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_hammer(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hammer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hammer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hammer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restpose(mut self, val: bool) -> Self {
        self.base.params.insert(
            "restpose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restpose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addshader(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryCrag {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_crag"
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
pub enum SopTestgeometryElectraTexture {
    Wooden = 0,
    Metallic = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestgeometryElectraOutput {
    SkinSurface = 0,
    ApexScene = 1,
    ApexCharacter = 2,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryElectra {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryElectra {
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

    // --- Menu parameters ---
    pub fn with_texture(mut self, val: SopTestgeometryElectraTexture) -> Self {
        self.base.params.insert(
            "texture".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_texture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output(mut self, val: SopTestgeometryElectraOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_ragdollcharname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ragdollcharname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdollcharname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdollcharname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addshader(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_configureragdoll(mut self, val: bool) -> Self {
        self.base.params.insert(
            "configureragdoll".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_configureragdoll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "configureragdoll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemotionmixerconfig(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemotionmixerconfig".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemotionmixerconfig_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemotionmixerconfig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_configurefullbodyik(mut self, val: bool) -> Self {
        self.base.params.insert(
            "configurefullbodyik".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_configurefullbodyik_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "configurefullbodyik".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useskincontrols(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useskincontrols".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useskincontrols_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useskincontrols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryElectra {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_electra"
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
pub enum SopTestgeometryOttoOutput {
    SkinSurface = 0,
    AllGeometry = 1,
    ApexScene = 2,
    ApexCharacter = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestgeometryOttoMeshlod {
    Render = 0,
    Proxy = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestgeometryOttoApexMeshlod {
    Render = 0,
    Proxy = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestgeometryOttoAnimation {
    BodyBuilder = 0,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryOtto {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryOtto {
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
    pub fn with_frame(mut self, val: f32) -> Self {
        self.base.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- Menu parameters ---
    pub fn with_output(mut self, val: SopTestgeometryOttoOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_meshlod(mut self, val: SopTestgeometryOttoMeshlod) -> Self {
        self.base.params.insert(
            "meshlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_meshlod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "meshlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apex_meshlod(mut self, val: SopTestgeometryOttoApexMeshlod) -> Self {
        self.base.params.insert(
            "apex_meshlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_apex_meshlod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apex_meshlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animation(mut self, val: SopTestgeometryOttoAnimation) -> Self {
        self.base.params.insert(
            "animation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_animation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_charactername(mut self, val: &str) -> Self {
        self.base.params.insert(
            "charactername".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_charactername_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "charactername".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdollcharname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ragdollcharname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdollcharname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdollcharname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdolllimitsattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ragdolllimitsattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdolllimitsattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdolllimitsattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fbikjointconfigattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fbikjointconfigattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fbikjointconfigattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fbikjointconfigattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fbiktargetconfigattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fbiktargetconfigattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fbiktargetconfigattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fbiktargetconfigattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addmat(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addmat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addmat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addmat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addshorts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshorts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshorts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshorts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addbonecapture(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addbonecapture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addbonecapture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addbonecapture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apex_skinmesh(mut self, val: bool) -> Self {
        self.base.params.insert(
            "apex_skinmesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apex_skinmesh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apex_skinmesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apex_shortsmesh(mut self, val: bool) -> Self {
        self.base.params.insert(
            "apex_shortsmesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apex_shortsmesh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apex_shortsmesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apex_skeletonmesh(mut self, val: bool) -> Self {
        self.base.params.insert(
            "apex_skeletonmesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apex_skeletonmesh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apex_skeletonmesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apex_organmesh(mut self, val: bool) -> Self {
        self.base.params.insert(
            "apex_organmesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apex_organmesh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apex_organmesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apex_addmuscleprim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "apex_addmuscleprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apex_addmuscleprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apex_addmuscleprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allgeo_unpack(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allgeo_unpack".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allgeo_unpack_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allgeo_unpack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allgeo_addbonecapture(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allgeo_addbonecapture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allgeo_addbonecapture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allgeo_addbonecapture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addmldeform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addmldeform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addmldeform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addmldeform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doconfigureragdoll(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doconfigureragdoll".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doconfigureragdoll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doconfigureragdoll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doconfigurefbik(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doconfigurefbik".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doconfigurefbik_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doconfigurefbik".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_skin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_skin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_skin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_skin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_fingernails(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_fingernails".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_fingernails_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_fingernails".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_eyes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_eyes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_eyes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_eyes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_mouth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_mouth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_mouth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_mouth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_skeleton(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_skeleton".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_skeleton_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_skeleton".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_organs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_organs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_organs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_organs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_georender_shorts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "georender_shorts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_georender_shorts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "georender_shorts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geosim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geosim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geosim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosim_skin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geosim_skin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geosim_skin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geosim_skin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosim_skeleton(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geosim_skeleton".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geosim_skeleton_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geosim_skeleton".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosim_organs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geosim_organs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geosim_organs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geosim_organs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosim_muscles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geosim_muscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geosim_muscles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geosim_muscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_skin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_skin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_skin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_skin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_fingernails(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_fingernails".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_fingernails_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_fingernails".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_eyes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_eyes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_eyes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_eyes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_mouth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_mouth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_mouth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_mouth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_skeleton(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_skeleton".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_skeleton_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_skeleton".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_organs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_organs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_organs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_organs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoproxy_shorts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "geoproxy_shorts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geoproxy_shorts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geoproxy_shorts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addanimation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addanimation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addanimation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addanimation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputclip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputclip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputclip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputclip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryOtto {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_otto"
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
pub enum SopTestgeometryPigheadDifficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryPighead {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryPighead {
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
    pub fn with_uniformscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uniformscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformscale".to_string(),
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

    // --- Menu parameters ---
    pub fn with_difficulty(mut self, val: SopTestgeometryPigheadDifficulty) -> Self {
        self.base.params.insert(
            "difficulty".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_difficulty_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "difficulty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addshader(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryPighead {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_pighead"
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
pub enum SopTestgeometryRubbertoyDifficulty {
    Hard = 0,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryRubbertoy {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryRubbertoy {
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
    pub fn with_uniformscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uniformscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformscale".to_string(),
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

    // --- Menu parameters ---
    pub fn with_difficulty(mut self, val: SopTestgeometryRubbertoyDifficulty) -> Self {
        self.base.params.insert(
            "difficulty".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_difficulty_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "difficulty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addshader(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryRubbertoy {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_rubbertoy"
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
pub enum SopTestgeometryShaderballModel {
    Default = 0,
    VmantraClassic = 1,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryShaderball {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryShaderball {
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

    // --- Int parameters ---
    pub fn with_model(mut self, val: SopTestgeometryShaderballModel) -> Self {
        self.base.params.insert(
            "model".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_model_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "model".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shop_materialpath_shell(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath_shell".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_shell_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath_shell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_pedestal(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath_pedestal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_pedestal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shop_materialpath_pedestal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_seam(mut self, val: bool) -> Self {
        self.base.params.insert(
            "seam".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seam_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seam".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_screws(mut self, val: bool) -> Self {
        self.base.params.insert(
            "screws".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_screws_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "screws".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_pedestal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_pedestal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_pedestal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_pedestal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryShaderball {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_shaderball"
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
pub enum SopTestgeometrySquabDifficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometrySquab {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometrySquab {
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
    pub fn with_uniformscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uniformscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformscale".to_string(),
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

    // --- Menu parameters ---
    pub fn with_difficulty(mut self, val: SopTestgeometrySquabDifficulty) -> Self {
        self.base.params.insert(
            "difficulty".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_difficulty_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "difficulty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addshader(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometrySquab {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_squab"
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
pub struct SopTestgeometryTemplatebody {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryTemplatebody {
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
}

impl crate::core::types::HoudiniNode for SopTestgeometryTemplatebody {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_templatebody"
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
pub struct SopTestgeometryTemplatehead {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryTemplatehead {
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
}

impl crate::core::types::HoudiniNode for SopTestgeometryTemplatehead {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_templatehead"
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
pub enum SopTestgeometryTommyRes {
    HiRes = 0,
    LowRes = 1,
}

#[derive(Debug, Clone)]
pub struct SopTestgeometryTommy {
    pub base: crate::core::types::NodeBase,
}

impl SopTestgeometryTommy {
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

    // --- Int parameters ---
    pub fn with_res(mut self, val: SopTestgeometryTommyRes) -> Self {
        self.base.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_switch_clothing(mut self, val: bool) -> Self {
        self.base.params.insert(
            "switch_clothing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_switch_clothing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "switch_clothing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_switch_textures(mut self, val: bool) -> Self {
        self.base.params.insert(
            "switch_textures".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_switch_textures_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "switch_textures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestgeometryTommy {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testgeometry_tommy"
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
pub enum SopTestsimCrowdtransitionTransitionmode {
    CustomSequence = 0,
    Random = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestsimCrowdtransitionMode {
    DirectionVector = 0,
    UpAttribute = 1,
}

#[derive(Debug, Clone)]
pub struct SopTestsimCrowdtransition {
    pub base: crate::core::types::NodeBase,
}

impl SopTestsimCrowdtransition {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Clip Transition Graph"
    pub fn set_input_clip_transition_graph(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Clip Transition Graph" and specifies the output index of the target node.
    pub fn set_input_clip_transition_graph_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Terrain"
    pub fn set_input_terrain(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Clip Properties"
    pub fn set_input_clip_properties(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Clip Properties" and specifies the output index of the target node.
    pub fn set_input_clip_properties_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.base.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_triggerframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "triggerframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_triggerframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "triggerframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomstartoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "randomstartoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomstartoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomstartoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomstartseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "randomstartseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomstartseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomstartseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggerframe_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("triggerframe{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_triggerframe_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("triggerframe{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomtransitionseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "randomtransitionseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomtransitionseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomtransitionseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framesbetweentriggers(mut self, val: f32) -> Self {
        self.base.params.insert(
            "framesbetweentriggers".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_framesbetweentriggers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "framesbetweentriggers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainguidescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "terrainguidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_terrainguidescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainguidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hipoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hipoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hipoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hipshiftperframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hipshiftperframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipshiftperframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hipshiftperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kneedampingthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "kneedampingthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kneedampingthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kneedampingthreshold".to_string(),
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
    pub fn with_tiltangleperframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tiltangleperframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltangleperframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tiltangleperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintilt(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mintilt".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintilt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mintilt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtilt(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxtilt".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtilt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxtilt".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_projectiondir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "projectiondir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_projectiondir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projectiondir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_before_transition_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "before_transition_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_before_transition_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "before_transition_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pending_transition_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pending_transition_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pending_transition_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pending_transition_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_during_transition_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "during_transition_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_during_transition_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "during_transition_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_after_transition_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "after_transition_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_after_transition_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "after_transition_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refup(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_substep(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_transitionmode(mut self, val: SopTestsimCrowdtransitionTransitionmode) -> Self {
        self.base.params.insert(
            "transitionmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_transitionmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transitionmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: SopTestsimCrowdtransitionMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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

    // --- Ramp parameters ---
    pub fn with_terrainguidecolor(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "terrainguidecolor".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_terrainguidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_clipname_b(mut self, val: &str) -> Self {
        self.base.params.insert(
            "clipname_b".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_b_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clipname_b".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname_a(mut self, val: &str) -> Self {
        self.base.params.insert(
            "clipname_a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_a_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clipname_a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("clipname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clipname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_randomstart(mut self, val: bool) -> Self {
        self.base.params.insert(
            "randomstart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomstart_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showterrainguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showterrainguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showterrainguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showterrainguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefootlocking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablefootlocking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefootlocking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablefootlocking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjusthips(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adjusthips".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjusthips_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adjusthips".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limithipshift(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limithipshift".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limithipshift_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limithipshift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablekneedamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablekneedamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablekneedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablekneedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableterrainprojection(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableterrainprojection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableterrainprojection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableterrainprojection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_terrain_adaptation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enable_terrain_adaptation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_terrain_adaptation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enable_terrain_adaptation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestsimCrowdtransition {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testsim_crowdtransition"
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
pub enum SopTestsimRagdollRagdollStiffnessvalue {
    Constant = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTestsimRagdollConstraintsolvertype {
    /// Parallel Gauss-Seidel (Islands)
    ParallelGaussMinusSeidelIslands = 0,
    /// Parallel Gauss-Seidel (Graph Coloring)
    ParallelGaussMinusSeidelGraphColoring = 1,
}

#[derive(Debug, Clone)]
pub struct SopTestsimRagdoll {
    pub base: crate::core::types::NodeBase,
}

impl SopTestsimRagdoll {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.base.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_softness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintforcemixing(mut self, val: f32) -> Self {
        self.base.params.insert(
            "constraintforcemixing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constraintforcemixing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintforcemixing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_biasfactor(mut self, val: f32) -> Self {
        self.base.params.insert(
            "biasfactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_biasfactor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "biasfactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relaxationfactor(mut self, val: f32) -> Self {
        self.base.params.insert(
            "relaxationfactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relaxationfactor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relaxationfactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positioncfm(mut self, val: f32) -> Self {
        self.base.params.insert(
            "positioncfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positioncfm_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "positioncfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionerp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "positionerp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionerp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "positionerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ragdoll_stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_stiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnessscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnessscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnesscfm(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnesscfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnesscfm_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnesscfm".to_string(),
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
    pub fn with_sleepingtime(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sleepingtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sleepingtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sleepingtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contactbreakingthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "contactbreakingthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contactbreakingthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "contactbreakingthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintsolvertolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "constraintsolvertolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constraintsolvertolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintsolvertolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalcfm(mut self, val: f32) -> Self {
        self.base.params.insert(
            "globalcfm".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalcfm_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "globalcfm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalerp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "globalerp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalerp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "globalerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_penetrationthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "penetrationthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_penetrationthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "penetrationthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitimpulseerp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "splitimpulseerp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitimpulseerp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitimpulseerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_ragdoll_stiffnessramprange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessramprange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnessramprange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessramprange".to_string(),
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
    pub fn with_force(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "force".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_force_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "force".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ground_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ground_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_r(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ground_r".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ground_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numiteration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numiteration".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiteration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numiteration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substep(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_ragdoll_stiffnessvalue(
        mut self,
        val: SopTestsimRagdollRagdollStiffnessvalue,
    ) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessvalue".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ragdoll_stiffnessvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintsolvertype(mut self, val: SopTestsimRagdollConstraintsolvertype) -> Self {
        self.base.params.insert(
            "constraintsolvertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_constraintsolvertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintsolvertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_ragdoll_stiffnessramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ragdoll_stiffnessramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_ragdoll_stiffnessgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ragdoll_stiffnessgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_stiffnessgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showcollisionlayer(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showcollisionlayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showcollisionlayer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showcollisionlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_gravity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enable_gravity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_gravity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enable_gravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinrootshapes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pinrootshapes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pinrootshapes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pinrootshapes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinunconfiguredshapes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pinunconfiguredshapes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pinunconfiguredshapes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pinunconfiguredshapes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeinitialerror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computeinitialerror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeinitialerror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computeinitialerror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ragdoll_enablestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ragdoll_enablestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ragdoll_enablestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ragdoll_enablestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_implicitdrag(mut self, val: bool) -> Self {
        self.base.params.insert(
            "implicitdrag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_implicitdrag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "implicitdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomize_order(mut self, val: bool) -> Self {
        self.base.params.insert(
            "randomize_order".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomize_order_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomize_order".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ensureindependentislands(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ensureindependentislands".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ensureindependentislands_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ensureindependentislands".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitimpulse(mut self, val: bool) -> Self {
        self.base.params.insert(
            "splitimpulse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splitimpulse_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitimpulse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTestsimRagdoll {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "testsim_ragdoll"
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
pub enum SopTetconformLocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone)]
pub struct SopTetconform {
    pub base: crate::core::types::NodeBase,
}

impl SopTetconform {
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

    /// Connects to input 0: "Input Polygon Mesh"
    pub fn set_input_input_polygon_mesh(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Polygon Mesh" and specifies the output index of the target node.
    pub fn set_input_input_polygon_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Additional Points/Curves"
    pub fn set_input_additional_points_curves(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Additional Points/Curves" and specifies the output index of the target node.
    pub fn set_input_additional_points_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_basesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "basesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxtetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxtetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaleconst(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scaleconst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scaleconst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scaleconst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalelocalfeature(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scalelocalfeature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalelocalfeature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scalelocalfeature".to_string(),
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

    // --- Menu parameters ---
    pub fn with_localscaling(mut self, val: SopTetconformLocalscaling) -> Self {
        self.base.params.insert(
            "localscaling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_localscaling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localscaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insertedpointgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "insertedpointgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_insertedpointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "insertedpointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usebasesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxtetscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxtetscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxtetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveinputgeometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "preserveinputgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveinputgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preserveinputgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onefacepertet(mut self, val: bool) -> Self {
        self.base.params.insert(
            "onefacepertet".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onefacepertet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onefacepertet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputsurftri(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputsurftri".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputsurftri_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputsurftri".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowsurfacemods(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allowsurfacemods".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowsurfacemods_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allowsurfacemods".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hiquality(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hiquality".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hiquality_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hiquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesdf(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generateinsertedpointgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "generateinsertedpointgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generateinsertedpointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generateinsertedpointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetconform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetconform"
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
pub enum SopTetcraftDirection {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone)]
pub struct SopTetcraft {
    pub base: crate::core::types::NodeBase,
}

impl SopTetcraft {
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

    // --- Menu parameters ---
    pub fn with_direction(mut self, val: SopTetcraftDirection) -> Self {
        self.base.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_direction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "direction".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_createtets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createtets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createtets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createtets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetcraft {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetcraft"
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
pub enum SopTetembedLocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone)]
pub struct SopTetembed {
    pub base: crate::core::types::NodeBase,
}

impl SopTetembed {
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

    /// Connects to input 0: "Input Polygon Mesh"
    pub fn set_input_input_polygon_mesh(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Polygon Mesh" and specifies the output index of the target node.
    pub fn set_input_input_polygon_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Additional Points/Curves"
    pub fn set_input_additional_points_curves(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Additional Points/Curves" and specifies the output index of the target node.
    pub fn set_input_additional_points_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_basesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "basesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxtetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxtetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surftriscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surftriscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surftriscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surftriscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaleconst(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scaleconst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scaleconst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scaleconst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalelocalfeature(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scalelocalfeature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalelocalfeature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scalelocalfeature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voxeloffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "voxeloffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxeloffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "voxeloffset".to_string(),
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

    // --- Int parameters ---
    pub fn with_maxres(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxres".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_localscaling(mut self, val: SopTetembedLocalscaling) -> Self {
        self.base.params.insert(
            "localscaling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_localscaling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localscaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usebasesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputsurftri(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputsurftri".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputsurftri_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputsurftri".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coverinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "coverinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coverinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coverinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usevoxelsize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usevoxelsize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usevoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxres(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetembed {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetembed"
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
pub struct SopTetfracture {
    pub base: crate::core::types::NodeBase,
}

impl SopTetfracture {
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

    /// Connects to input 0: "Geometry To Fracture"
    pub fn set_input_geometry_to_fracture(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry To Fracture" and specifies the output index of the target node.
    pub fn set_input_geometry_to_fracture_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_basesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "basesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "partscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_patternoffset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "patternoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_patternoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "patternoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_origptsgrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "origptsgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_origptsgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "origptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newptsgrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "newptsgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newptsgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "newptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usebasesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voronoi(mut self, val: bool) -> Self {
        self.base.params.insert(
            "voronoi".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_voronoi_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "voronoi".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualizepieces(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visualizepieces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualizepieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualizepieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitprims(mut self, val: bool) -> Self {
        self.base.params.insert(
            "splitprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splitprims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doorigptsgrp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doorigptsgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorigptsgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doorigptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_donewptsgrp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "donewptsgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donewptsgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "donewptsgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetfracture {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetfracture"
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
pub enum SopTetlayerDirection {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone)]
pub struct SopTetlayer {
    pub base: crate::core::types::NodeBase,
}

impl SopTetlayer {
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

    // --- Float parameters ---
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_direction(mut self, val: SopTetlayerDirection) -> Self {
        self.base.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_direction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "direction".to_string(),
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
    pub fn with_thicknessmultiplierattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknessmultiplierattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessmultiplierattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessmultiplierattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablethicknessmultiplierattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablethicknessmultiplierattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablethicknessmultiplierattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablethicknessmultiplierattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createboundarytriangles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createboundarytriangles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createboundarytriangles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createboundarytriangles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createtets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createtets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createtets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createtets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetlayer {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetlayer"
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
pub struct SopTetpartition {
    pub base: crate::core::types::NodeBase,
}

impl SopTetpartition {
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

    /// Connects to input 0: "Input Tetrahedron Mesh"
    pub fn set_input_input_tetrahedron_mesh(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Tetrahedron Mesh" and specifies the output index of the target node.
    pub fn set_input_input_tetrahedron_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input Region Boundaries"
    pub fn set_input_input_region_boundaries(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input Region Boundaries" and specifies the output index of the target node.
    pub fn set_input_input_region_boundaries_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_tetgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tetgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tetgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tetgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polygroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "polygroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_polygroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "polygroup".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopTetpartition {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetpartition"
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
pub enum SopTetrahedralizeBatch {
    Entire = 0,
    ConnectedComponents = 1,
    Attribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTetrahedralizeMode {
    ConformToSurface = 0,
    RefineTets = 1,
    ConvexHull = 2,
    DetectIntersections = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTetrahedralizeOutput {
    ConnectedPolylines = 0,
    Tetrahedra = 1,
    Polygons = 2,
    PolygonsAndTetrahedra = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTetrahedralizeFailures {
    RemoveFailedComponents = 0,
    KeepFailedComponents = 1,
    FailOnError = 2,
}

#[derive(Debug, Clone)]
pub struct SopTetrahedralize {
    pub base: crate::core::types::NodeBase,
}

impl SopTetrahedralize {
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

    /// Connects to input 0: "Boundary Constraints or Tet Mesh for Refinement"
    pub fn set_input_boundary_constraints_or_tet_mesh_for_ref(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Boundary Constraints or Tet Mesh for Refinement" and specifies the output index of the target node.
    pub fn set_input_boundary_constraints_or_tet_mesh_for_ref_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Additional Points For Delaunay Tetrahedralization"
    pub fn set_input_additional_points_for_delaunay_tetrahedr(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Additional Points For Delaunay Tetrahedralization" and specifies the output index of the target node.
    pub fn set_input_additional_points_for_delaunay_tetrahedr_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_radedgetol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radedgetol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radedgetol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radedgetol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mindihedralang(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mindihedralang".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mindihedralang_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mindihedralang".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformmaxsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uniformmaxsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformmaxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformmaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precisiontol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "precisiontol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_precisiontol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "precisiontol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dihedralangtol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dihedralangtol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dihedralangtol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dihedralangtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_isectpolyclr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "isectpolyclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_isectpolyclr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isectpolyclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invalidprimclr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "invalidprimclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_invalidprimclr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invalidprimclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxiter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsteiner(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxsteiner".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsteiner_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsteiner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "optiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_optiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "randomseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_randomseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxattempts(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxattempts".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxattempts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxattempts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_batch(mut self, val: SopTetrahedralizeBatch) -> Self {
        self.base.params.insert(
            "batch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "batch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: SopTetrahedralizeMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_output(mut self, val: SopTetrahedralizeOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_failures(mut self, val: SopTetrahedralizeFailures) -> Self {
        self.base.params.insert(
            "failures".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_failures_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "failures".to_string(),
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
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsizeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetsizeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetsizeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetsizeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsizeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maxsizeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxsizeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsizeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isectpolygrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "isectpolygrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_isectpolygrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isectpolygrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invalidprimgrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "invalidprimgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_invalidprimgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invalidprimgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_remove(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remove".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remove_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepprims(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepprims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noboundmod(mut self, val: bool) -> Self {
        self.base.params.insert(
            "noboundmod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_noboundmod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "noboundmod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onefacepertet(mut self, val: bool) -> Self {
        self.base.params.insert(
            "onefacepertet".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onefacepertet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onefacepertet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_propnormal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "propnormal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_propnormal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "propnormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_internattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "internattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_internattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "internattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usequality(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usequality".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usequality_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetargetsizeattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetargetsizeattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetsizeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetargetsizeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useuniformmaxsize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useuniformmaxsize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useuniformmaxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useuniformmaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxsizeattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxsizeattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxsizeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxsizeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxiter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxiter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxsteiner(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxsteiner".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxsteiner_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxsteiner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optedgeface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "optedgeface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optedgeface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optedgeface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optvtxsmooth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "optvtxsmooth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optvtxsmooth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optvtxsmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optvtxmod(mut self, val: bool) -> Self {
        self.base.params.insert(
            "optvtxmod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optvtxmod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optvtxmod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useisectcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useisectcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useisectcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useisectcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useisectgrp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useisectgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useisectgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useisectgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useinvalidcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useinvalidcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useinvalidcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useinvalidcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useinvalidgrp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useinvalidgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useinvalidgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useinvalidgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetrahedralize {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetrahedralize"
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
pub struct SopTetrasurface {
    pub base: crate::core::types::NodeBase,
}

impl SopTetrasurface {
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

    /// Connects to input 0: "Input Geometry"
    pub fn set_input_input_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepprimitives(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepprimitives".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepprimitives_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepprimitives".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_buildpolysoup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "buildpolysoup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_buildpolysoup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "buildpolysoup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetrasurface {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetrasurface"
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
pub struct SopTetstrata {
    pub base: crate::core::types::NodeBase,
}

impl SopTetstrata {
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

    /// Connects to input 0: "Input Polygon Mesh"
    pub fn set_input_input_polygon_mesh(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Polygon Mesh" and specifies the output index of the target node.
    pub fn set_input_input_polygon_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Inner Boundary Surface"
    pub fn set_input_inner_boundary_surface(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Inner Boundary Surface" and specifies the output index of the target node.
    pub fn set_input_inner_boundary_surface_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_exteriorthickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "exteriorthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exteriorthickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorthickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "interiorthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interiorthickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regularinteriorinitialfraction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "regularinteriorinitialfraction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_regularinteriorinitialfraction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regularinteriorinitialfraction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivethicknessmultiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "adaptivethicknessmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adaptivethicknessmultiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adaptivethicknessmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformmaxsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uniformmaxsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformmaxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformmaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_exteriortetlayers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "exteriortetlayers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_exteriortetlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriortetlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiortetlayers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "interiortetlayers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_interiortetlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiortetlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_exteriorlayertetsname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "exteriorlayertetsname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exteriorlayertetsname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorlayertetsname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorsurfacename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "exteriorsurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exteriorsurfacename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorsurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiplierthicknessri(mut self, val: &str) -> Self {
        self.base.params.insert(
            "multiplierthicknessri".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_multiplierthicknessri_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multiplierthicknessri".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorlayertetsname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "interiorlayertetsname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorlayertetsname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorlayertetsname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorsurfacename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "interiorsurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorsurfacename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorsurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiplierthicknessai(mut self, val: &str) -> Self {
        self.base.params.insert(
            "multiplierthicknessai".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_multiplierthicknessai_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multiplierthicknessai".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorfillertetsname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "interiorfillertetsname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorfillertetsname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorfillertetsname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorboundaryname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "interiorboundaryname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorboundaryname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorboundaryname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputprimitivesname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputprimitivesname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputprimitivesname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputprimitivesname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createexteriorlayers(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createexteriorlayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createexteriorlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createexteriorlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorcreatetets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "exteriorcreatetets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exteriorcreatetets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorcreatetets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createexteriorsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createexteriorsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createexteriorsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createexteriorsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createinteriorlayers(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createinteriorlayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createinteriorlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createinteriorlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemultiplierthicknessri(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemultiplierthicknessri".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemultiplierthicknessri_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemultiplierthicknessri".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorcreatetets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "interiorcreatetets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_interiorcreatetets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorcreatetets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createinteriorsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createinteriorsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createinteriorsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createinteriorsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createinteriorfillertets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createinteriorfillertets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createinteriorfillertets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createinteriorfillertets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemultiplierthicknessai(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemultiplierthicknessai".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemultiplierthicknessai_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemultiplierthicknessai".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createinteriorboundary(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createinteriorboundary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createinteriorboundary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createinteriorboundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveinputgeometry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "preserveinputgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveinputgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preserveinputgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTetstrata {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tetstrata"
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
pub enum SopTextureType {
    Orthographic = 0,
    Polar = 1,
    Cylindrical = 2,
    /// Rows & Columns
    RowsColumns = 3,
    Face = 4,
    ModifySource = 5,
    UniformSpline = 6,
    AverageSpline = 7,
    ArcLengthSpline = 8,
    PerspectiveFromCamera = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTextureAxis {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTextureCoord {
    AutoSelect = 0,
    Point = 1,
    Vertex = 2,
}

#[derive(Debug, Clone)]
pub struct SopTexture {
    pub base: crate::core::types::NodeBase,
}

impl SopTexture {
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

    /// Connects to input 0: "Texture source"
    pub fn set_input_texture_source(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Texture source" and specifies the output index of the target node.
    pub fn set_input_texture_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_angle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopTextureType) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_axis(mut self, val: SopTextureAxis) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_coord(mut self, val: SopTextureCoord) -> Self {
        self.base.params.insert(
            "coord".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_coord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coord".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "uvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvattrib".to_string(),
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
    pub fn with_campath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "campath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_campath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "campath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_fixseams(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fixseams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixseams_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fixseams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTexture {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "texture"
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
pub enum SopTexturefeatureMethod {
    CornerDetection = 0,
    MinimumEigenValue = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturefeatureOutput {
    Geometry = 0,
    Volume = 1,
}

#[derive(Debug, Clone)]
pub struct SopTexturefeature {
    pub base: crate::core::types::NodeBase,
}

impl SopTexturefeature {
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

    /// Connects to input 0: "Volume Image"
    pub fn set_input_volume_image(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Volume Image" and specifies the output index of the target node.
    pub fn set_input_volume_image_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Volume Mask"
    pub fn set_input_volume_mask(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Volume Mask" and specifies the output index of the target node.
    pub fn set_input_volume_mask_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_qualitytolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "qualitytolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_qualitytolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "qualitytolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cornerweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cornerweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cornerweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cornerweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxfeaturesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxfeaturesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxfeaturesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxfeaturesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimumspacing(mut self, val: i32) -> Self {
        self.base.params.insert(
            "minimumspacing".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumspacing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minimumspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blurringwindowradius(mut self, val: i32) -> Self {
        self.base.params.insert(
            "blurringwindowradius".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blurringwindowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blurringwindowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradientwindowradius(mut self, val: i32) -> Self {
        self.base.params.insert(
            "gradientwindowradius".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_gradientwindowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradientwindowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopTexturefeatureMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output(mut self, val: SopTexturefeatureOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
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
    pub fn with_outputname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTexturefeature {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "texturefeature"
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
pub enum SopTexturemaskpaintResmenu {
    /// 256
    N256 = 0,
    /// 512
    N512 = 1,
    /// 1024
    N1024 = 2,
    /// 2048
    N2048 = 3,
    /// 4096
    N4096 = 4,
    /// 8192
    N8192 = 5,
    /// 16384
    N16384 = 6,
    /// 32768
    N32768 = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintUpdatemode {
    Continuous = 0,
    OnMouseUp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintBrushspace {
    World = 0,
    Screen = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintMirroraxis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintVistype {
    Layered = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintFgcolormode {
    Color = 0,
    Texture = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintBgcolormode {
    Color = 0,
    Texture = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTexturemaskpaintCanvascolormode {
    Color = 0,
    Texture = 1,
}

#[derive(Debug, Clone)]
pub struct SopTexturemaskpaint {
    pub base: crate::core::types::NodeBase,
}

impl SopTexturemaskpaint {
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

    /// Connects to input 0: "Geometry"
    pub fn set_input_geometry(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry" and specifies the output index of the target node.
    pub fn set_input_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Volumes"
    pub fn set_input_mask_volumes(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Volumes" and specifies the output index of the target node.
    pub fn set_input_mask_volumes_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_clear(mut self) -> Self {
        self.base
            .params
            .insert("clear".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_resetallchanges(mut self) -> Self {
        self.base.params.insert(
            "resetallchanges".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_flattenwithinput(mut self) -> Self {
        self.base.params.insert(
            "flattenwithinput".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_brushreset(mut self) -> Self {
        self.base.params.insert(
            "brushreset".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_movestashtofile(mut self) -> Self {
        self.base.params.insert(
            "movestashtofile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadstashfromfile(mut self) -> Self {
        self.base.params.insert(
            "loadstashfromfile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_stash(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stash".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_stash_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "brushes".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_brushes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha(mut self, val: &str) -> Self {
        self.base.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_clearvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clearvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clearvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clearvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushfgvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushfgvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushfgvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushfgvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushbgvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushbgvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushbgvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushbgvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushspacing(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushspacing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushspacing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushopacity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushopacity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushopacity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushopacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushsoftedge(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushsoftedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushsoftedge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushsoftedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushalphascale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushalphascale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushalphascale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushalphascale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushalpharotate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "brushalpharotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brushalpharotate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushalpharotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_fgcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fgcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fgcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fgcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "bgcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bgcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bgcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_canvascolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "canvascolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_canvascolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "canvascolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_resmenu(mut self, val: SopTexturemaskpaintResmenu) -> Self {
        self.base.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatemode(mut self, val: SopTexturemaskpaintUpdatemode) -> Self {
        self.base.params.insert(
            "updatemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_updatemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "updatemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushname(mut self, val: i32) -> Self {
        self.base.params.insert(
            "brushname".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_brushname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushop(mut self, val: i32) -> Self {
        self.base.params.insert(
            "brushop".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_brushop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushspace(mut self, val: SopTexturemaskpaintBrushspace) -> Self {
        self.base.params.insert(
            "brushspace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_brushspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mirroraxis(mut self, val: SopTexturemaskpaintMirroraxis) -> Self {
        self.base.params.insert(
            "mirroraxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mirroraxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirroraxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vistype(mut self, val: SopTexturemaskpaintVistype) -> Self {
        self.base.params.insert(
            "vistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vistype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fgcolormode(mut self, val: SopTexturemaskpaintFgcolormode) -> Self {
        self.base.params.insert(
            "fgcolormode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fgcolormode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fgcolormode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgcolormode(mut self, val: SopTexturemaskpaintBgcolormode) -> Self {
        self.base.params.insert(
            "bgcolormode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bgcolormode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bgcolormode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_canvascolormode(mut self, val: SopTexturemaskpaintCanvascolormode) -> Self {
        self.base.params.insert(
            "canvascolormode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_canvascolormode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "canvascolormode".to_string(),
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
    pub fn with_uvattr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "uvattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvattr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maskname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alphaname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "alphaname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_alphaname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alphaname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushalphaimagefile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "brushalphaimagefile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_brushalphaimagefile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushalphaimagefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushcustomfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "brushcustomfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_brushcustomfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushcustomfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fgtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fgtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fgtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fgtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bgtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bgtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bgtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_canvastexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "canvastexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_canvastexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "canvastexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stashfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stashfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stashfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stashfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usealpha(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usealpha".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usealpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usealpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useudimattr(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useudimattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useudimattr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useudimattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_paint2d(mut self, val: bool) -> Self {
        self.base.params.insert(
            "paint2d".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_paint2d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "paint2d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushopon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushopon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushopon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushopon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushspaceon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushspaceon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushspaceon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushspaceon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushradiuson(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushradiuson".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushradiuson_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushradiuson".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushfgvalueon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushfgvalueon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushfgvalueon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushfgvalueon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushbgvalueon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushbgvalueon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushbgvalueon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushbgvalueon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushspacingon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushspacingon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushspacingon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushspacingon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushopacityon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushopacityon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushopacityon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushopacityon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushsoftedgeon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushsoftedgeon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushsoftedgeon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushsoftedgeon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushalphascaleon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushalphascaleon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushalphascaleon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushalphascaleon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushalpharotateon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushalpharotateon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushalpharotateon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushalpharotateon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brushalphaimagefileon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "brushalphaimagefileon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_brushalphaimagefileon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "brushalphaimagefileon".to_string(),
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
    pub fn with_showgeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTexturemaskpaint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "texturemaskpaint"
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
pub trait SopTexturemaskpaintInnerExt {
    fn alpha(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_cloud(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_image(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_rough(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_standard(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_texture(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn brushes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn cur_brush(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn draw_geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn flatten_with_input(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn geo(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_alpha(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn masks(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn stash(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn updated_brushes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_new(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_new1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_orig(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribadjustdict1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcast1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcast2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcast3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcopy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcopy2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribcopy3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribdelete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribdelete2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribdelete3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribdelete4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribdelete5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribdelete7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribfromparm1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribpromote1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribpromote2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle11(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle12(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle13(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle14(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle15(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle16(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle19(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle20(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle21(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle24(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle25(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle26(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle27(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle29(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn attribwrangle9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blast6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn canvas_2d(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compile_begin9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn convert1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copnet1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copnet2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copnet3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copytopoints1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn copytopoints2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn current_brush_parms(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn divide1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin10(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin11(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin1_metadata1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin1_metadata2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn grid1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn group1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn groupdelete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn invokegraph1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn invokegraph2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn mask_new(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn mask_new1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn mask_orig(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn name1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn normal2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn polyextrude1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn process_paint_vols(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn processed_input_mesh(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn remove_non_polygon_prims(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename_uv_attr_to_uv(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn split1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn split2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn stash1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switchif9(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn transformbyattrib1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn validate_input(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vertexsplit1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumeresample1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumeresample2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumeresample3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumevop1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumevop3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumevop4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumevop5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volumevop6(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopTexturemaskpaintInnerExt for crate::core::graph::InnerGraph<'a, SopTexturemaskpaint> {
    fn alpha(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ALPHA")
    }
    fn alpha_cloud(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ALPHA_CLOUD")
    }
    fn alpha_image(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ALPHA_IMAGE")
    }
    fn alpha_rough(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ALPHA_ROUGH")
    }
    fn alpha_standard(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ALPHA_STANDARD")
    }
    fn alpha_texture(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("ALPHA_TEXTURE")
    }
    fn brushes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("BRUSHES")
    }
    fn cur_brush(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("CUR_BRUSH")
    }
    fn draw_geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("DRAW_GEO")
    }
    fn flatten_with_input(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("FLATTEN_WITH_INPUT")
    }
    fn geo(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("GEO")
    }
    fn input(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("INPUT")
    }
    fn input_alpha(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("INPUT_ALPHA")
    }
    fn input_masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("INPUT_MASKS")
    }
    fn masks(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("MASKS")
    }
    fn stash(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("STASH")
    }
    fn updated_brushes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("UPDATED_BRUSHES")
    }
    fn alpha_new(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("alpha_new")
    }
    fn alpha_new1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("alpha_new1")
    }
    fn alpha_orig(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("alpha_orig")
    }
    fn attribadjustdict1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribadjustdict1")
    }
    fn attribcast1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcast1")
    }
    fn attribcast2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcast2")
    }
    fn attribcast3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcast3")
    }
    fn attribcopy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcopy1")
    }
    fn attribcopy2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcopy2")
    }
    fn attribcopy3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribcopy3")
    }
    fn attribdelete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribdelete1")
    }
    fn attribdelete2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribdelete2")
    }
    fn attribdelete3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribdelete3")
    }
    fn attribdelete4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribdelete4")
    }
    fn attribdelete5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribdelete5")
    }
    fn attribdelete7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribdelete7")
    }
    fn attribfromparm1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribfromparm1")
    }
    fn attribpromote1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribpromote1")
    }
    fn attribpromote2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribpromote2")
    }
    fn attribwrangle1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle1")
    }
    fn attribwrangle10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle10")
    }
    fn attribwrangle11(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle11")
    }
    fn attribwrangle12(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle12")
    }
    fn attribwrangle13(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle13")
    }
    fn attribwrangle14(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle14")
    }
    fn attribwrangle15(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle15")
    }
    fn attribwrangle16(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle16")
    }
    fn attribwrangle19(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle19")
    }
    fn attribwrangle2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle2")
    }
    fn attribwrangle20(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle20")
    }
    fn attribwrangle21(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle21")
    }
    fn attribwrangle24(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle24")
    }
    fn attribwrangle25(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle25")
    }
    fn attribwrangle26(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle26")
    }
    fn attribwrangle27(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle27")
    }
    fn attribwrangle29(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle29")
    }
    fn attribwrangle3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle3")
    }
    fn attribwrangle4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle4")
    }
    fn attribwrangle5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle5")
    }
    fn attribwrangle6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle6")
    }
    fn attribwrangle7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle7")
    }
    fn attribwrangle8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle8")
    }
    fn attribwrangle9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("attribwrangle9")
    }
    fn blast1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blast1")
    }
    fn blast3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blast3")
    }
    fn blast5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blast5")
    }
    fn blast6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blast6")
    }
    fn canvas_2d(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("canvas_2d")
    }
    fn compile_begin1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin1")
    }
    fn compile_begin10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin10")
    }
    fn compile_begin2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin2")
    }
    fn compile_begin3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin3")
    }
    fn compile_begin4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin4")
    }
    fn compile_begin5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin5")
    }
    fn compile_begin6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin6")
    }
    fn compile_begin7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin7")
    }
    fn compile_begin8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin8")
    }
    fn compile_begin9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("compile_begin9")
    }
    fn convert1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("convert1")
    }
    fn copnet1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copnet1")
    }
    fn copnet2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copnet2")
    }
    fn copnet3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copnet3")
    }
    fn copytopoints1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copytopoints1")
    }
    fn copytopoints2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("copytopoints2")
    }
    fn current_brush_parms(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("current_brush_parms")
    }
    fn divide1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("divide1")
    }
    fn foreach_begin1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin1")
    }
    fn foreach_begin10(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin10")
    }
    fn foreach_begin11(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin11")
    }
    fn foreach_begin1_metadata1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin1_metadata1")
    }
    fn foreach_begin1_metadata2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin1_metadata2")
    }
    fn foreach_begin2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin2")
    }
    fn foreach_begin3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin3")
    }
    fn foreach_begin4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin4")
    }
    fn foreach_begin5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin5")
    }
    fn foreach_begin6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin6")
    }
    fn foreach_begin9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin9")
    }
    fn foreach_end1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_end1")
    }
    fn foreach_end2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_end2")
    }
    fn foreach_end3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_end3")
    }
    fn grid1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("grid1")
    }
    fn group1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("group1")
    }
    fn groupdelete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("groupdelete1")
    }
    fn invokegraph1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("invokegraph1")
    }
    fn invokegraph2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("invokegraph2")
    }
    fn mask_new(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("mask_new")
    }
    fn mask_new1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("mask_new1")
    }
    fn mask_orig(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("mask_orig")
    }
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge1")
    }
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge2")
    }
    fn merge6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge6")
    }
    fn merge7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge7")
    }
    fn merge8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("merge8")
    }
    fn name1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("name1")
    }
    fn normal2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("normal2")
    }
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn output1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output1")
    }
    fn output2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output2")
    }
    fn polyextrude1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("polyextrude1")
    }
    fn process_paint_vols(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("process_paint_vols")
    }
    fn processed_input_mesh(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("processed_input_mesh")
    }
    fn remove_non_polygon_prims(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("remove_non_polygon_prims")
    }
    fn rename_uv_attr_to_uv(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rename_uv_attr_to_uv")
    }
    fn split1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("split1")
    }
    fn split2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("split2")
    }
    fn stash1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("stash1")
    }
    fn switchif1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif1")
    }
    fn switchif2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif2")
    }
    fn switchif3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif3")
    }
    fn switchif4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif4")
    }
    fn switchif5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif5")
    }
    fn switchif6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif6")
    }
    fn switchif8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif8")
    }
    fn switchif9(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switchif9")
    }
    fn transformbyattrib1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("transformbyattrib1")
    }
    fn validate_input(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("validate_input")
    }
    fn vertexsplit1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vertexsplit1")
    }
    fn volume1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volume1")
    }
    fn volume2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volume2")
    }
    fn volume5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volume5")
    }
    fn volume6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volume6")
    }
    fn volumeresample1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumeresample1")
    }
    fn volumeresample2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumeresample2")
    }
    fn volumeresample3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumeresample3")
    }
    fn volumevop1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumevop1")
    }
    fn volumevop3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumevop3")
    }
    fn volumevop4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumevop4")
    }
    fn volumevop5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumevop5")
    }
    fn volumevop6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("volumevop6")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTextureopticalflowMethod {
    Farneback = 0,
    Dis = 1,
}

#[derive(Debug, Clone)]
pub struct SopTextureopticalflow {
    pub base: crate::core::types::NodeBase,
}

impl SopTextureopticalflow {
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

    /// Connects to input 0: "Source Image"
    pub fn set_input_source_image(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source Image" and specifies the output index of the target node.
    pub fn set_input_source_image_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Goal Image"
    pub fn set_input_goal_image(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Goal Image" and specifies the output index of the target node.
    pub fn set_input_goal_image_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_pyramidscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pyramidscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pyramidscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pyramidscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_approximationwindowradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "approximationwindowradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_approximationwindowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "approximationwindowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothnessweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "smoothnessweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smoothnessweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothnessweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorconstancyweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "colorconstancyweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorconstancyweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "colorconstancyweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradientconstancyweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gradientconstancyweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradientconstancyweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradientconstancyweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_blurringwindowradius(mut self, val: i32) -> Self {
        self.base.params.insert(
            "blurringwindowradius".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blurringwindowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blurringwindowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pyramidlevels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pyramidlevels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pyramidlevels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pyramidlevels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothnessdegree(mut self, val: i32) -> Self {
        self.base.params.insert(
            "smoothnessdegree".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_smoothnessdegree_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothnessdegree".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_patchsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "patchsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_patchsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "patchsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_patchstride(mut self, val: i32) -> Self {
        self.base.params.insert(
            "patchstride".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_patchstride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "patchstride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finestscale(mut self, val: i32) -> Self {
        self.base.params.insert(
            "finestscale".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_finestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "finestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradientdescentiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "gradientdescentiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_gradientdescentiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradientdescentiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variationalrefinementiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "variationalrefinementiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_variationalrefinementiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "variationalrefinementiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopTextureopticalflowMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegroup".to_string(),
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
    pub fn with_outputname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegaussianfilter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegaussianfilter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegaussianfilter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegaussianfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemeannormalization(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemeannormalization".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemeannormalization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemeannormalization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usespatialpropagation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usespatialpropagation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usespatialpropagation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usespatialpropagation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTextureopticalflow {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "textureopticalflow"
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
pub enum SopTimeblendVoxelblend {
    None = 0,
    ByGridIndex = 1,
}

#[derive(Debug, Clone)]
pub struct SopTimeblend {
    pub base: crate::core::types::NodeBase,
}

impl SopTimeblend {
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

    /// Connects to input 0: "Geometry to Evaluate at Another Time"
    pub fn set_input_geometry_to_evaluate_at_another_time(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Evaluate at Another Time" and specifies the output index of the target node.
    pub fn set_input_geometry_to_evaluate_at_another_time_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_firstframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "firstframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_firstframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "firstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lastframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "lastframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lastframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lastframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_voxelblend(mut self, val: SopTimeblendVoxelblend) -> Self {
        self.base.params.insert(
            "voxelblend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "voxelblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptidattr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ptidattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptidattr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptidattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidattr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primidattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidattr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primidattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_holdfirst(mut self, val: bool) -> Self {
        self.base.params.insert(
            "holdfirst".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_holdfirst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "holdfirst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_holdlast(mut self, val: bool) -> Self {
        self.base.params.insert(
            "holdlast".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_holdlast_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "holdlast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doslerp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doslerp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doslerp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doslerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usevforpinterp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usevforpinterp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevforpinterp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usevforpinterp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTimeblend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "timeblend"
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
pub enum SopTimeshiftMethod {
    ByFrame = 0,
    ByTime = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTimeshiftRangeclamp {
    None = 0,
    ClampToFirst = 1,
    ClampToLast = 2,
    ClampToBoth = 3,
}

#[derive(Debug, Clone)]
pub struct SopTimeshift {
    pub base: crate::core::types::NodeBase,
}

impl SopTimeshift {
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

    // --- Float parameters ---
    pub fn with_frame(mut self, val: f32) -> Self {
        self.base.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_time(mut self, val: f32) -> Self {
        self.base.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopTimeshiftMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangeclamp(mut self, val: SopTimeshiftRangeclamp) -> Self {
        self.base.params.insert(
            "rangeclamp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rangeclamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rangeclamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_integerframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "integerframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_integerframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "integerframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTimeshift {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "timeshift"
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
pub enum SopTimewarpPreextend {
    Hold = 0,
    Extend = 1,
    Cycle = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTimewarpPostextend {
    Hold = 0,
    Extend = 1,
    Cycle = 2,
}

#[derive(Debug, Clone)]
pub struct SopTimewarp {
    pub base: crate::core::types::NodeBase,
}

impl SopTimewarp {
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

    /// Connects to input 0: "Geometry to Skew Evaluation Times of"
    pub fn set_input_geometry_to_skew_evaluation_times_of(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Skew Evaluation Times of" and specifies the output index of the target node.
    pub fn set_input_geometry_to_skew_evaluation_times_of_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float2 parameters ---
    pub fn with_inrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "inrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_inrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "outrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_outrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_preextend(mut self, val: SopTimewarpPreextend) -> Self {
        self.base.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postextend(mut self, val: SopTimewarpPostextend) -> Self {
        self.base.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTimewarp {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "timewarp"
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
pub enum SopTissuepropertiesGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesMode {
    AssignValues = 0,
    BlendWithMask = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesPresetproperties {
    None = 0,
    Weak = 1,
    Blubber = 2,
    Sponge = 3,
    Firm = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfacestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfacedampingmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfacebendstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfacebenddampingmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfacemassdensitymode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfaceattachstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfaceattachcompressstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfaceattachdampingmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfaceattachrestscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesCollisionscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSolidshapestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSolidvolumestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSoliddampingratiomode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSolidmassdensitymode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesTissueattachstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesTissueattachdampingmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSlidingratemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSlidinglimitmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSlidinglimitstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSlidingdampingratiomode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesRestscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesCorerestscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesSurfacerestscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone)]
pub struct SopTissueproperties {
    pub base: crate::core::types::NodeBase,
}

impl SopTissueproperties {
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

    /// Connects to input 0: "Tissue"
    pub fn set_input_tissue(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Tissue" and specifies the output index of the target node.
    pub fn set_input_tissue_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Blend Source"
    pub fn set_input_blend_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Blend Source" and specifies the output index of the target node.
    pub fn set_input_blend_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_maskblend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maskblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maskblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacedamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacedamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacedampingrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacedampingrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacedampingrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacedampingrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacebendstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacebendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacebendstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacebendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacebendstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacebendstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacebendstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacebendstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacebenddamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacebenddamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacebenddamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacebenddamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacebenddampingrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacebenddampingrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacebenddampingrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacebenddampingrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacemassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacemassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacemassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacemassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacemassdensityrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacemassdensityrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacemassdensityrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacemassdensityrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachcompressstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachcompressstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachcompressstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachcompressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachcompressstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachcompressstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachcompressstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachcompressstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachdamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachdamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachdampingrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachdampingrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachdampingrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachdampingrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachrestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachrestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachrestscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceattachrestscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceattachrestscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachrestscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "collisionscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "collisionscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidshapestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratiorel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soliddampingratiorel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratiorel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratiorel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensityrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidmassdensityrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensityrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensityrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissueattachstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissueattachstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissueattachstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissueattachstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachdamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissueattachdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissueattachdamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachdampingrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissueattachdampingrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissueattachdampingrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachdampingrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidingrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingraterel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidingraterel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidingraterel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingraterel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidinglimit(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidinglimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidinglimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidinglimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidinglimitrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidinglimitrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidinglimitrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidinglimitrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidinglimitstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidinglimitstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidinglimitstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidinglimitstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidinglimitstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidinglimitstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidinglimitstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidinglimitstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingdampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidingdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidingdampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingdampingratiorel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slidingdampingratiorel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidingdampingratiorel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingdampingratiorel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "restscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "restscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corerestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corerestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corerestscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corerestscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corerestscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corerestscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacerestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacerestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacerestscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacerestscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacerestscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacerestscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_mode(mut self, val: SopTissuepropertiesMode) -> Self {
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
    pub fn with_surfacestiffnessmode(
        mut self,
        val: SopTissuepropertiesSurfacestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "surfacestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfacestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacedampingmode(mut self, val: SopTissuepropertiesSurfacedampingmode) -> Self {
        self.base.params.insert(
            "surfacedampingmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfacedampingmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacedampingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacebendstiffnessmode(
        mut self,
        val: SopTissuepropertiesSurfacebendstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "surfacebendstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfacebendstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacebendstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacebenddampingmode(
        mut self,
        val: SopTissuepropertiesSurfacebenddampingmode,
    ) -> Self {
        self.base.params.insert(
            "surfacebenddampingmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfacebenddampingmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacebenddampingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacemassdensitymode(
        mut self,
        val: SopTissuepropertiesSurfacemassdensitymode,
    ) -> Self {
        self.base.params.insert(
            "surfacemassdensitymode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfacemassdensitymode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacemassdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachstiffnessmode(
        mut self,
        val: SopTissuepropertiesSurfaceattachstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "surfaceattachstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfaceattachstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachcompressstiffnessmode(
        mut self,
        val: SopTissuepropertiesSurfaceattachcompressstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "surfaceattachcompressstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfaceattachcompressstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachcompressstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachdampingmode(
        mut self,
        val: SopTissuepropertiesSurfaceattachdampingmode,
    ) -> Self {
        self.base.params.insert(
            "surfaceattachdampingmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfaceattachdampingmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachdampingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachrestscalemode(
        mut self,
        val: SopTissuepropertiesSurfaceattachrestscalemode,
    ) -> Self {
        self.base.params.insert(
            "surfaceattachrestscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfaceattachrestscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachrestscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionscalemode(mut self, val: SopTissuepropertiesCollisionscalemode) -> Self {
        self.base.params.insert(
            "collisionscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_collisionscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffnessmode(
        mut self,
        val: SopTissuepropertiesSolidshapestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "solidshapestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidshapestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffnessmode(
        mut self,
        val: SopTissuepropertiesSolidvolumestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidvolumestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratiomode(
        mut self,
        val: SopTissuepropertiesSoliddampingratiomode,
    ) -> Self {
        self.base.params.insert(
            "soliddampingratiomode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_soliddampingratiomode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratiomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensitymode(
        mut self,
        val: SopTissuepropertiesSolidmassdensitymode,
    ) -> Self {
        self.base.params.insert(
            "solidmassdensitymode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidmassdensitymode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachstiffnessmode(
        mut self,
        val: SopTissuepropertiesTissueattachstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "tissueattachstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissueattachstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachdampingmode(
        mut self,
        val: SopTissuepropertiesTissueattachdampingmode,
    ) -> Self {
        self.base.params.insert(
            "tissueattachdampingmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissueattachdampingmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachdampingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingratemode(mut self, val: SopTissuepropertiesSlidingratemode) -> Self {
        self.base.params.insert(
            "slidingratemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_slidingratemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingratemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidinglimitmode(mut self, val: SopTissuepropertiesSlidinglimitmode) -> Self {
        self.base.params.insert(
            "slidinglimitmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_slidinglimitmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidinglimitmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidinglimitstiffnessmode(
        mut self,
        val: SopTissuepropertiesSlidinglimitstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "slidinglimitstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_slidinglimitstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidinglimitstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingdampingratiomode(
        mut self,
        val: SopTissuepropertiesSlidingdampingratiomode,
    ) -> Self {
        self.base.params.insert(
            "slidingdampingratiomode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_slidingdampingratiomode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingdampingratiomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscalemode(mut self, val: SopTissuepropertiesRestscalemode) -> Self {
        self.base.params.insert(
            "restscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_restscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corerestscalemode(mut self, val: SopTissuepropertiesCorerestscalemode) -> Self {
        self.base.params.insert(
            "corerestscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_corerestscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corerestscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacerestscalemode(
        mut self,
        val: SopTissuepropertiesSurfacerestscalemode,
    ) -> Self {
        self.base.params.insert(
            "surfacerestscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_surfacerestscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacerestscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopTissuepropertiesGrouptype) -> Self {
        self.base.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_presetproperties(mut self, val: SopTissuepropertiesPresetproperties) -> Self {
        self.base.params.insert(
            "presetproperties".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_presetproperties_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "presetproperties".to_string(),
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
    pub fn with_mask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invertmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invertmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfacestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfacestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfacestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfacestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfacedamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfacedamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfacedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfacedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfacebendstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfacebendstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfacebendstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfacebendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfacebenddamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfacebenddamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfacebenddamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfacebenddamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfacemassdensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfacemassdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfacemassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfacemassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfaceattachstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfaceattachstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfaceattachstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfaceattachstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfaceattachcompressstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfaceattachcompressstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfaceattachcompressstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfaceattachcompressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfaceattachdamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfaceattachdamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfaceattachdamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfaceattachdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfaceattachrestscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfaceattachrestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfaceattachrestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfaceattachrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecollisionscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecollisionscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecollisionscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecollisionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidshapestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidvolumestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesoliddampingratio(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesoliddampingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesoliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesoliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidmassdensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidmassdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissueattachstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissueattachstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissueattachstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissueattachstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissueattachdamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissueattachdamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissueattachdamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissueattachdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslidingrate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableslidingrate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslidingrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableslidingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslidinglimit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableslidinglimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslidinglimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableslidinglimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslidinglimitstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableslidinglimitstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslidinglimitstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableslidinglimitstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslidingdampingratio(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableslidingdampingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslidingdampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableslidingdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerestscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecorerestscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecorerestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecorerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecorerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesurfacerestscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesurfacerestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesurfacerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesurfacerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTissueproperties {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tissueproperties"
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
pub enum SopTissuepropertiesotisGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisMode {
    AssignValues = 0,
    BlendWithMask = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisPresetproperties {
    None = 0,
    Weak = 1,
    Blubber = 2,
    Sponge = 3,
    Firm = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCoresolidshapestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCoresolidvolumestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCoresoliddampingratioexp {
    /// 1
    N1 = 0,
    /// 0.1
    N01 = 1,
    /// 0.01
    N001 = 2,
    /// 0.001
    N0001 = 3,
    /// 0.000 1
    N00001 = 4,
    /// 0.000 01
    N000001 = 5,
    /// 0.000 001
    N0000001 = 6,
    /// 1e-7
    N1eMinus7 = 7,
    /// 1e-8
    N1eMinus8 = 8,
    /// 1e-9
    N1eMinus9 = 9,
    /// 1e-10
    N1eMinus10 = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCoresoliddampingratiomode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCoresolidmassdensitymode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCoresolidfiberstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisSolidshapestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisSolidvolumestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisSoliddampingratioexp {
    /// 1
    N1 = 0,
    /// 0.1
    N01 = 1,
    /// 0.01
    N001 = 2,
    /// 0.001
    N0001 = 3,
    /// 0.000 1
    N00001 = 4,
    /// 0.000 01
    N000001 = 5,
    /// 0.000 001
    N0000001 = 6,
    /// 1e-7
    N1eMinus7 = 7,
    /// 1e-8
    N1eMinus8 = 8,
    /// 1e-9
    N1eMinus9 = 9,
    /// 1e-10
    N1eMinus10 = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisSoliddampingratiomode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisSolidmassdensitymode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisSolidfiberstiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetomusclestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetomuscledampingexp {
    /// 1
    N1 = 0,
    /// 0.1
    N01 = 1,
    /// 0.01
    N001 = 2,
    /// 0.001
    N0001 = 3,
    /// 0.000 1
    N00001 = 4,
    /// 0.000 01
    N000001 = 5,
    /// 0.000 001
    N0000001 = 6,
    /// 1e-7
    N1eMinus7 = 7,
    /// 1e-8
    N1eMinus8 = 8,
    /// 1e-9
    N1eMinus9 = 9,
    /// 1e-10
    N1eMinus10 = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetomuscledampingmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetomuscledistancemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetobonestiffnessmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetobonedampingexp {
    /// 1
    N1 = 0,
    /// 0.1
    N01 = 1,
    /// 0.01
    N001 = 2,
    /// 0.001
    N0001 = 3,
    /// 0.000 1
    N00001 = 4,
    /// 0.000 01
    N000001 = 5,
    /// 0.000 001
    N0000001 = 6,
    /// 1e-7
    N1eMinus7 = 7,
    /// 1e-8
    N1eMinus8 = 8,
    /// 1e-9
    N1eMinus9 = 9,
    /// 1e-10
    N1eMinus10 = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetobonedampingmode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisTissuetobonedistancemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisRestscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuepropertiesotisCorerestscalemode {
    Absolute = 0,
    Relative = 1,
}

#[derive(Debug, Clone)]
pub struct SopTissuepropertiesotis {
    pub base: crate::core::types::NodeBase,
}

impl SopTissuepropertiesotis {
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

    /// Connects to input 0: "Tissue"
    pub fn set_input_tissue(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Tissue" and specifies the output index of the target node.
    pub fn set_input_tissue_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Blend Source"
    pub fn set_input_blend_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Blend Source" and specifies the output index of the target node.
    pub fn set_input_blend_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_maskblend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maskblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maskblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidshapestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidshapestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidshapestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidshapestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidshapestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidvolumestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidvolumestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidvolumestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidvolumestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidvolumestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresoliddampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresoliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresoliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresoliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresoliddampingratiorel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresoliddampingratiorel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresoliddampingratiorel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresoliddampingratiorel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidmassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidmassdensityrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidmassdensityrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidmassdensityrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidmassdensityrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidfiberstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidfiberstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidfiberstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coresolidfiberstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coresolidfiberstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidfiberstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidshapestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratiorel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soliddampingratiorel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratiorel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratiorel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensityrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidmassdensityrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensityrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensityrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidfiberstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidfiberstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidfiberstiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidfiberstiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidfiberstiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidfiberstiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomusclestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetomusclestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetomusclestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomusclestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomusclestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetomusclestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetomusclestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomusclestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetomuscledamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetomuscledamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledampingrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetomuscledampingrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetomuscledampingrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledampingrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetomuscledistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetomuscledistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledistancerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetomuscledistancerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetomuscledistancerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledistancerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetobonestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetobonestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonestiffnessrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetobonestiffnessrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetobonestiffnessrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonestiffnessrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetobonedamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetobonedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedampingrel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetobonedampingrel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetobonedampingrel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedampingrel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetobonedistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetobonedistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedistancerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuetobonedistancerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuetobonedistancerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedistancerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "restscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "restscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corerestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corerestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corerestscalerel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corerestscalerel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corerestscalerel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corerestscalerel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_mode(mut self, val: SopTissuepropertiesotisMode) -> Self {
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
    pub fn with_coresolidshapestiffnessmode(
        mut self,
        val: SopTissuepropertiesotisCoresolidshapestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "coresolidshapestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coresolidshapestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidshapestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidvolumestiffnessmode(
        mut self,
        val: SopTissuepropertiesotisCoresolidvolumestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "coresolidvolumestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coresolidvolumestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidvolumestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresoliddampingratiomode(
        mut self,
        val: SopTissuepropertiesotisCoresoliddampingratiomode,
    ) -> Self {
        self.base.params.insert(
            "coresoliddampingratiomode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coresoliddampingratiomode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresoliddampingratiomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidmassdensitymode(
        mut self,
        val: SopTissuepropertiesotisCoresolidmassdensitymode,
    ) -> Self {
        self.base.params.insert(
            "coresolidmassdensitymode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coresolidmassdensitymode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidmassdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresolidfiberstiffnessmode(
        mut self,
        val: SopTissuepropertiesotisCoresolidfiberstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "coresolidfiberstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coresolidfiberstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresolidfiberstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffnessmode(
        mut self,
        val: SopTissuepropertiesotisSolidshapestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "solidshapestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidshapestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffnessmode(
        mut self,
        val: SopTissuepropertiesotisSolidvolumestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidvolumestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratiomode(
        mut self,
        val: SopTissuepropertiesotisSoliddampingratiomode,
    ) -> Self {
        self.base.params.insert(
            "soliddampingratiomode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_soliddampingratiomode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratiomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensitymode(
        mut self,
        val: SopTissuepropertiesotisSolidmassdensitymode,
    ) -> Self {
        self.base.params.insert(
            "solidmassdensitymode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidmassdensitymode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidfiberstiffnessmode(
        mut self,
        val: SopTissuepropertiesotisSolidfiberstiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "solidfiberstiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_solidfiberstiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidfiberstiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomusclestiffnessmode(
        mut self,
        val: SopTissuepropertiesotisTissuetomusclestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "tissuetomusclestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissuetomusclestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomusclestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledampingmode(
        mut self,
        val: SopTissuepropertiesotisTissuetomuscledampingmode,
    ) -> Self {
        self.base.params.insert(
            "tissuetomuscledampingmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissuetomuscledampingmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledampingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledistancemode(
        mut self,
        val: SopTissuepropertiesotisTissuetomuscledistancemode,
    ) -> Self {
        self.base.params.insert(
            "tissuetomuscledistancemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissuetomuscledistancemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledistancemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonestiffnessmode(
        mut self,
        val: SopTissuepropertiesotisTissuetobonestiffnessmode,
    ) -> Self {
        self.base.params.insert(
            "tissuetobonestiffnessmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissuetobonestiffnessmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonestiffnessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedampingmode(
        mut self,
        val: SopTissuepropertiesotisTissuetobonedampingmode,
    ) -> Self {
        self.base.params.insert(
            "tissuetobonedampingmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissuetobonedampingmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedampingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedistancemode(
        mut self,
        val: SopTissuepropertiesotisTissuetobonedistancemode,
    ) -> Self {
        self.base.params.insert(
            "tissuetobonedistancemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tissuetobonedistancemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedistancemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscalemode(mut self, val: SopTissuepropertiesotisRestscalemode) -> Self {
        self.base.params.insert(
            "restscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_restscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corerestscalemode(mut self, val: SopTissuepropertiesotisCorerestscalemode) -> Self {
        self.base.params.insert(
            "corerestscalemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_corerestscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corerestscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopTissuepropertiesotisGrouptype) -> Self {
        self.base.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_presetproperties(mut self, val: SopTissuepropertiesotisPresetproperties) -> Self {
        self.base.params.insert(
            "presetproperties".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_presetproperties_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "presetproperties".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coresoliddampingratioexp(
        mut self,
        val: SopTissuepropertiesotisCoresoliddampingratioexp,
    ) -> Self {
        self.base.params.insert(
            "coresoliddampingratioexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_coresoliddampingratioexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresoliddampingratioexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratioexp(
        mut self,
        val: SopTissuepropertiesotisSoliddampingratioexp,
    ) -> Self {
        self.base.params.insert(
            "soliddampingratioexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_soliddampingratioexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratioexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetomuscledampingexp(
        mut self,
        val: SopTissuepropertiesotisTissuetomuscledampingexp,
    ) -> Self {
        self.base.params.insert(
            "tissuetomuscledampingexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tissuetomuscledampingexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetomuscledampingexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuetobonedampingexp(
        mut self,
        val: SopTissuepropertiesotisTissuetobonedampingexp,
    ) -> Self {
        self.base.params.insert(
            "tissuetobonedampingexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tissuetobonedampingexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuetobonedampingexp".to_string(),
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
    pub fn with_mask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invertmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invertmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoresolidshapestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecoresolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoresolidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecoresolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoresolidvolumestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecoresolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoresolidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecoresolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoresoliddampingratio(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecoresoliddampingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoresoliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecoresoliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoresolidmassdensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecoresolidmassdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoresolidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecoresolidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoresolidfiberstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecoresolidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoresolidfiberstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecoresolidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidshapestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidvolumestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesoliddampingratio(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesoliddampingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesoliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesoliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidmassdensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidmassdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolidfiberstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolidfiberstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolidfiberstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuetomusclestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuetomusclestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuetomusclestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuetomusclestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuetomuscledamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuetomuscledamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuetomuscledamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuetomuscledamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuetomuscledistance(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuetomuscledistance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuetomuscledistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuetomuscledistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuetobonestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuetobonestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuetobonestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuetobonestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuetobonedamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuetobonedamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuetobonedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuetobonedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuetobonedistance(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuetobonedistance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuetobonedistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuetobonedistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerestscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecorerestscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecorerestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecorerestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecorerestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTissuepropertiesotis {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tissuepropertiesotis"
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
pub enum SopTissuesolidifyTissuethicknessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolidifyGuidedisplay {
    Off = 0,
    CoreSurface = 1,
    Outliers = 2,
    Inversions = 3,
    LowQualityPrimitives = 4,
    CoreFalloff = 5,
    TissueInteriorSurface = 6,
}

#[derive(Debug, Clone)]
pub struct SopTissuesolidify {
    pub base: crate::core::types::NodeBase,
}

impl SopTissuesolidify {
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

    /// Connects to input 0: "Skin Surface Geometry"
    pub fn set_input_skin_surface_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Skin Surface Geometry" and specifies the output index of the target node.
    pub fn set_input_skin_surface_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Muscles and Bones Geometry"
    pub fn set_input_muscles_and_bones_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Muscles and Bones Geometry" and specifies the output index of the target node.
    pub fn set_input_muscles_and_bones_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Core Surface"
    pub fn set_input_core_surface(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Core Surface" and specifies the output index of the target node.
    pub fn set_input_core_surface_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Core Central Axis Polylines"
    pub fn set_input_core_central_axis_polylines(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Core Central Axis Polylines" and specifies the output index of the target node.
    pub fn set_input_core_central_axis_polylines_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_surfaceoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceoffsetlimit(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfaceoffsetlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceoffsetlimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceoffsetlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuethickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuethickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuethickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuemaxtetsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuemaxtetsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuemaxtetsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuemaxtetsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reldensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reldensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reldensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reldensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gradation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacevoxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacevoxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacevoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacevoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreoffsetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coreoffsetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coreoffsetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coreoffsetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coremaxtetsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coremaxtetsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coremaxtetsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coremaxtetsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divisionsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "divisionsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divisionsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divisionsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corefalloffdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corefalloffdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corefalloffdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corefalloffdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corefalloffwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corefalloffwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corefalloffwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corefalloffwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minsizecoresurface(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minsizecoresurface".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minsizecoresurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minsizecoresurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsizecoresurface(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxsizecoresurface".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxsizecoresurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsizecoresurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reldensitycoresurface(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reldensitycoresurface".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reldensitycoresurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reldensitycoresurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradationcoresurface(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gradationcoresurface".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradationcoresurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradationcoresurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minsizebackstop(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minsizebackstop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minsizebackstop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minsizebackstop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsizebackstop(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxsizebackstop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxsizebackstop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsizebackstop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reldensitybackstop(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reldensitybackstop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reldensitybackstop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reldensitybackstop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradationbackstop(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gradationbackstop".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradationbackstop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradationbackstop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlierscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "outlierscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outlierscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outlierscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_qualitythreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "qualitythreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_qualitythreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "qualitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corevoxelsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corevoxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corevoxelsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corevoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreparticlescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coreparticlescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coreparticlescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coreparticlescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.base.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_coresurfaceguidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "coresurfaceguidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_coresurfaceguidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coresurfaceguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorsurfaceguidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "interiorsurfaceguidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_interiorsurfaceguidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorsurfaceguidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleoutliercolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "muscleoutliercolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_muscleoutliercolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "muscleoutliercolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreoutliercolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "coreoutliercolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_coreoutliercolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coreoutliercolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreprimitivecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "coreprimitivecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_coreprimitivecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coreprimitivecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueprimitivecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "tissueprimitivecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tissueprimitivecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueprimitivecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_substepiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substepiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substepiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substepiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bluriterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bluriterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bluriterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bluriterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidedisplay(mut self, val: SopTissuesolidifyGuidedisplay) -> Self {
        self.base.params.insert(
            "guidedisplay".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_guidedisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidedisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_tissuethicknessscalemode(
        mut self,
        val: SopTissuesolidifyTissuethicknessscalemode,
    ) -> Self {
        self.base.params.insert(
            "tissuethicknessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tissuethicknessscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuethicknessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_tissuethicknessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tissuethicknessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissuethicknessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuethicknessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coreoffsetattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "coreoffsetattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coreoffsetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coreoffsetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablethicknessscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablethicknessscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablethicknessscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablethicknessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshexteriorsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remeshexteriorsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remeshexteriorsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexteriorsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoreoffset(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecoreoffset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoreoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecoreoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecorethicknessmatch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecorethicknessmatch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecorethicknessmatch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecorethicknessmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshcoresurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remeshcoresurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remeshcoresurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshcoresurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayexterior(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displayexterior".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayexterior_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayexterior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshbackstopsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remeshbackstopsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remeshbackstopsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshbackstopsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTissuesolidify {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tissuesolidify"
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
pub enum SopTissuesolidifyotisGrouptype {
    GuessFromGroup = 0,
    Breakpoints = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolidifyotisVisualization {
    ExteriorRemesh = 0,
    InnerSurface = 1,
    TissueMesh = 2,
    CoreTetrahedra = 3,
    ShellTetrahedra = 4,
    TinyTetrahedra = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolidifyotisMethod {
    FusedTissueAndCore = 0,
    SeparatedTissueAndCore = 1,
    CoreOnly = 2,
    Fascia = 3,
}

#[derive(Debug, Clone)]
pub struct SopTissuesolidifyotis {
    pub base: crate::core::types::NodeBase,
}

impl SopTissuesolidifyotis {
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

    /// Connects to input 0: "Skin Geometry"
    pub fn set_input_skin_geometry(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Skin Geometry" and specifies the output index of the target node.
    pub fn set_input_skin_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Muscle Geometry"
    pub fn set_input_muscle_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Muscle Geometry" and specifies the output index of the target node.
    pub fn set_input_muscle_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Bone Geometry"
    pub fn set_input_bone_geometry(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Bone Geometry" and specifies the output index of the target node.
    pub fn set_input_bone_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_remeshexterior_vis(mut self) -> Self {
        self.base.params.insert(
            "remeshexterior_vis".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_innershrinkwrap_vis(mut self) -> Self {
        self.base.params.insert(
            "innershrinkwrap_vis".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_tissue_meshvis(mut self) -> Self {
        self.base.params.insert(
            "tissue_meshvis".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_remeshexterior_minsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "remeshexterior_minsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_remeshexterior_minsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexterior_minsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshexterior_maxsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "remeshexterior_maxsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_remeshexterior_maxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexterior_maxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshexterior_density(mut self, val: f32) -> Self {
        self.base.params.insert(
            "remeshexterior_density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_remeshexterior_density_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexterior_density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshexterior_gradation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "remeshexterior_gradation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_remeshexterior_gradation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexterior_gradation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_slopelimit(mut self, val: f32) -> Self {
        self.base.params.insert(
            "innershrinkwrap_slopelimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_innershrinkwrap_slopelimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_slopelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_smoothing(mut self, val: f32) -> Self {
        self.base.params.insert(
            "innershrinkwrap_smoothing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_innershrinkwrap_smoothing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_smoothing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissue_minthickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissue_minthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissue_minthickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissue_minthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissue_tetsquashratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissue_tetsquashratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissue_tetsquashratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissue_tetsquashratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_smoothiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "smoothiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_smoothiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshexterior_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "remeshexterior_iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_remeshexterior_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexterior_iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_prebluriterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "innershrinkwrap_prebluriterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_innershrinkwrap_prebluriterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_prebluriterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "innershrinkwrap_iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_innershrinkwrap_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopTissuesolidifyotisGrouptype) -> Self {
        self.base.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualization(mut self, val: SopTissuesolidifyotisVisualization) -> Self {
        self.base.params.insert(
            "visualization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_visualization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: SopTissuesolidifyotisMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
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
    pub fn with_innershrinkwrap_preblurmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_preblurmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_preblurmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_preblurmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepbiggestpiece(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepbiggestpiece".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepbiggestpiece_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepbiggestpiece".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showblur(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "smoothgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_smoothgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshexterior_enable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remeshexterior_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remeshexterior_enable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshexterior_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_preblur(mut self, val: bool) -> Self {
        self.base.params.insert(
            "innershrinkwrap_preblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_innershrinkwrap_preblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_preblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_innershrinkwrap_finaldetangle(mut self, val: bool) -> Self {
        self.base.params.insert(
            "innershrinkwrap_finaldetangle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_innershrinkwrap_finaldetangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "innershrinkwrap_finaldetangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTissuesolidifyotis {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tissuesolidifyotis"
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
pub enum SopTissuesolverUsesurfacedensity {
    Uniform = 0,
    UseDensityAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverTissuelocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverUsevolumereduction {
    Off = 0,
    UseScale = 1,
    UseGeo = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverPrebakedlocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverBonelocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverBonecollisiondetection {
    UseSolverDefault = 0,
    UseVolumeCollisions = 1,
    UseSurfaceCollisions = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverCollisiondetection {
    UseSolverDefault = 0,
    UseVolumeCollisions = 1,
    UseSurfaceCollisions = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverIntegratortype {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverAuxoutput {
    SurfaceTriangles = 0,
    TetMesh = 1,
    AnimatedTetMesh = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverPreviewdisplay {
    Off = 0,
    Animated = 1,
    AnimatedWithShrinkage = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverUseremeshattrib {
    UniformTriangleScale = 0,
    UseScaleAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolverSolverbypass {
    SolverEnabled = 0,
    SolverBypassed = 1,
}

#[derive(Debug, Clone)]
pub struct SopTissuesolver {
    pub base: crate::core::types::NodeBase,
}

impl SopTissuesolver {
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

    /// Connects to input 0: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Animated Muscles"
    pub fn set_input_animated_muscles(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Animated Muscles" and specifies the output index of the target node.
    pub fn set_input_animated_muscles_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Static Muscles"
    pub fn set_input_static_muscles(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Static Muscles" and specifies the output index of the target node.
    pub fn set_input_static_muscles_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Animated Bones"
    pub fn set_input_animated_bones(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Animated Bones" and specifies the output index of the target node.
    pub fn set_input_animated_bones_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Static Bones"
    pub fn set_input_static_bones(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Static Bones" and specifies the output index of the target node.
    pub fn set_input_static_bones_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.base.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reload(mut self) -> Self {
        self.base
            .params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_execute(mut self) -> Self {
        self.base.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_stashgeo(mut self) -> Self {
        self.base.params.insert(
            "stashgeo".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_stashinfo(mut self) -> Self {
        self.base.params.insert(
            "stashinfo".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_clearstashedgeo(mut self) -> Self {
        self.base.params.insert(
            "clearstashedgeo".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_resimulatereduction(mut self) -> Self {
        self.base.params.insert(
            "resimulatereduction".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_stashedgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stashedgeo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_stashedgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stashedgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_shellshapestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shellshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shellshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strongbendstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strongbendstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shelldampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shelldampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shelldampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shelldampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellmassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shellmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shellmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellthickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellthickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdermaldepth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "subdermaldepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subdermaldepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subdermaldepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdermalmultiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "subdermalmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subdermalmultiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subdermalmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacedensityattribweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacedensityattribweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacedensityattribweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacedensityattribweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuemaxtetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuemaxtetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuemaxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuemaxtetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuescaleconst(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuescaleconst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuescaleconst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuescaleconst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuecalelocalfeature(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuecalelocalfeature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuecalelocalfeature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuecalelocalfeature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumereductionamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "volumereductionamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumereductionamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumereductionamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetstrength".to_string(),
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
    pub fn with_tissuecollisionradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuecollisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachmultiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissueattachmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissueattachmultiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleshapestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "muscleshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscleshapestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "muscleshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclevolumestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "musclevolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_musclevolumestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "musclevolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleattachslack(mut self, val: f32) -> Self {
        self.base.params.insert(
            "muscleattachslack".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscleattachslack_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "muscleattachslack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_muscleattachdepth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "muscleattachdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_muscleattachdepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "muscleattachdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedmassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "prebakedmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prebakedmassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedbasesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "prebakedbasesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prebakedbasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedbasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedmaxtetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "prebakedmaxtetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prebakedmaxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedmaxtetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedsurftriscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "prebakedsurftriscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prebakedsurftriscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedsurftriscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedscalelocalfeature(mut self, val: f32) -> Self {
        self.base.params.insert(
            "prebakedscalelocalfeature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prebakedscalelocalfeature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedscalelocalfeature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedscaleconst(mut self, val: f32) -> Self {
        self.base.params.insert(
            "prebakedscaleconst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prebakedscaleconst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedscaleconst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetrianglesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonetrianglesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonetrianglesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetrianglesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetetsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonetetsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonetetsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetetsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonebasesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonebasesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonemaxtetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonemaxtetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonemaxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonemaxtetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonesurftriscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonesurftriscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonesurftriscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonesurftriscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonescalelocalfeature(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonescalelocalfeature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonescalelocalfeature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonescalelocalfeature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonescaleconst(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonescaleconst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonescaleconst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonescaleconst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonecollisionradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonecollisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonecollisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonecollisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonevolumecollisiondivsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonevolumecollisiondivsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonevolumecollisiondivsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonevolumecollisiondivsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boneattachslack(mut self, val: f32) -> Self {
        self.base.params.insert(
            "boneattachslack".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_boneattachslack_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boneattachslack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonemassdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonemassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonemassdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonemassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepalpha(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sweepalpha".to_string(),
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
    pub fn with_sdftol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sdftol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdftol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sdftol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitlength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitmass(mut self, val: f32) -> Self {
        self.base.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitmass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonefriction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bonefriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bonefriction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonefriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuebasesize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuebasesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuesurftriscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuesurftriscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuesurftriscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuesurftriscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "remeshscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_remeshscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdermalstrength2(mut self, val: f32) -> Self {
        self.base.params.insert(
            "subdermalstrength2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subdermalstrength2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subdermalstrength2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_previewdeformedcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "previewdeformedcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_previewdeformedcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "previewdeformedcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumereductions(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "volumereductions".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_volumereductions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumereductions".to_string(),
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
    pub fn with_bypasscolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "bypasscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bypasscolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bypasscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacetrianglecount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "surfacetrianglecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_surfacetrianglecount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacetrianglecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacesubdivisions(mut self, val: i32) -> Self {
        self.base.params.insert(
            "surfacesubdivisions".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_surfacesubdivisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacesubdivisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reductionsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "reductionsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reductionsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reductionsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usevolumereduction(mut self, val: SopTissuesolverUsevolumereduction) -> Self {
        self.base.params.insert(
            "usevolumereduction".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usevolumereduction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usevolumereduction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumereductionblur(mut self, val: i32) -> Self {
        self.base.params.insert(
            "volumereductionblur".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_volumereductionblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumereductionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minpt(mut self, val: i32) -> Self {
        self.base.params.insert(
            "minpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxpt(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.base.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepcount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sweepcount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_usesurfacedensity(mut self, val: SopTissuesolverUsesurfacedensity) -> Self {
        self.base.params.insert(
            "usesurfacedensity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usesurfacedensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesurfacedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuelocalscaling(mut self, val: SopTissuesolverTissuelocalscaling) -> Self {
        self.base.params.insert(
            "tissuelocalscaling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tissuelocalscaling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuelocalscaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedlocalscaling(mut self, val: SopTissuesolverPrebakedlocalscaling) -> Self {
        self.base.params.insert(
            "prebakedlocalscaling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_prebakedlocalscaling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedlocalscaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonelocalscaling(mut self, val: SopTissuesolverBonelocalscaling) -> Self {
        self.base.params.insert(
            "bonelocalscaling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bonelocalscaling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonelocalscaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonecollisiondetection(
        mut self,
        val: SopTissuesolverBonecollisiondetection,
    ) -> Self {
        self.base.params.insert(
            "bonecollisiondetection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bonecollisiondetection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonecollisiondetection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisiondetection(mut self, val: SopTissuesolverCollisiondetection) -> Self {
        self.base.params.insert(
            "collisiondetection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisiondetection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisiondetection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_integratortype(mut self, val: SopTissuesolverIntegratortype) -> Self {
        self.base.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_integratortype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_auxoutput(mut self, val: SopTissuesolverAuxoutput) -> Self {
        self.base.params.insert(
            "auxoutput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_auxoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "auxoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewdisplay(mut self, val: SopTissuesolverPreviewdisplay) -> Self {
        self.base.params.insert(
            "previewdisplay".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_previewdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "previewdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useremeshattrib(mut self, val: SopTissuesolverUseremeshattrib) -> Self {
        self.base.params.insert(
            "useremeshattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_useremeshattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useremeshattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverbypass(mut self, val: SopTissuesolverSolverbypass) -> Self {
        self.base.params.insert(
            "solverbypass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solverbypass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solverbypass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacedensityattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "surfacedensityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surfacedensityattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacedensityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuescaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tissuescaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissuescaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuescaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumereducedgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "volumereducedgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumereducedgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumereducedgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumereduceattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "volumereduceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumereduceattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumereduceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueattachattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tissueattachattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissueattachattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueattachattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedscaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "prebakedscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prebakedscaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonescaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonescaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonescaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonescaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidersoppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collidersoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collidersoppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidersoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidergroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collidergroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collidergroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidergroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectivetissuegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "connectivetissuegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_connectivetissuegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "connectivetissuegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_musclesgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "musclesgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_musclesgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "musclesgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonesgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonesgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonesgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonesgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacetrianglesgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "surfacetrianglesgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surfacetrianglesgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacetrianglesgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometrygroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "embeddedgeometrygroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometrygroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "embeddedgeometrygroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshscaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "remeshscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remeshscaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skelrootpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skelrootpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skelrootpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skelrootpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_havestash(mut self, val: bool) -> Self {
        self.base.params.insert(
            "havestash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_havestash_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "havestash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useanimatedskin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useanimatedskin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useanimatedskin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useanimatedskin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletissuecollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletissuecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletissuecollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletissuecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableskinselfcollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableskinselfcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableskinselfcollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableskinselfcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useprebakedmuscles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useprebakedmuscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useprebakedmuscles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useprebakedmuscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemusclecollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemusclecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemusclecollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemusclecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidifyprebakedmuscles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solidifyprebakedmuscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solidifyprebakedmuscles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidifyprebakedmuscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedusebasesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prebakedusebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prebakedusebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedusebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prebakedcoverinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prebakedcoverinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prebakedcoverinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prebakedcoverinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidifybones(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solidifybones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solidifybones_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidifybones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boneusebasesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "boneusebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boneusebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boneusebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonecoverinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "bonecoverinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bonecoverinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonecoverinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boneanimationtype(mut self, val: bool) -> Self {
        self.base.params.insert(
            "boneanimationtype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boneanimationtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boneanimationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablebonecollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablebonecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablebonecollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablebonecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecollider(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecollider".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecollider_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecollider".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animategeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "animategeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animategeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_laserscan(mut self, val: bool) -> Self {
        self.base.params.insert(
            "laserscan".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_laserscan_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "laserscan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fixsigns(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fixsigns".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixsigns_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fixsigns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcebounds(mut self, val: bool) -> Self {
        self.base.params.insert(
            "forcebounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcebounds_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "forcebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invert(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachetodisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cachetodisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetodisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachetodisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvetissue(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solvetissue".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvetissue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvetissue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvemuscles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solvemuscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvemuscles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvemuscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvebones(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solvebones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvebones_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvebones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solveembedded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "solveembedded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solveembedded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solveembedded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtissue(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputtissue".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtissue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtissue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputmuscles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputmuscles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputmuscles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputmuscles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputbones(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputbones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputbones_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputbones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputembedded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputembedded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputembedded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputembedded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidebonecollisionradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidebonecollisionradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidebonecollisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidebonecollisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidetissuecollisionradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidetissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidetissuecollisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidetissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidebones(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidebones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidebones_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidebones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissueusebasesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tissueusebasesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tissueusebasesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueusebasesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuecoverinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tissuecoverinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tissuecoverinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuecoverinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideskelrootpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "overrideskelrootpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideskelrootpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "overrideskelrootpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTissuesolver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tissuesolver"
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
pub enum SopTissuesolvervellumVellumintegratortype {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTissuesolvervellumSlidingmethod {
    ClosestPoint = 0,
    TraversePolygons = 1,
    /// Traverse Triangles (Optimized)
    TraverseTrianglesOptimized = 2,
}

#[derive(Debug, Clone)]
pub struct SopTissuesolvervellum {
    pub base: crate::core::types::NodeBase,
}

impl SopTissuesolvervellum {
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

    /// Connects to input 0: "Tissue"
    pub fn set_input_tissue(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Tissue" and specifies the output index of the target node.
    pub fn set_input_tissue_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Internal Attach Geometry"
    pub fn set_input_internal_attach_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Internal Attach Geometry" and specifies the output index of the target node.
    pub fn set_input_internal_attach_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.base.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_initframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "initframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumtimescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumtimescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumtimescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumtimescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitlength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitmass(mut self, val: f32) -> Self {
        self.base.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitmass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tissuecollisionradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tissuecollisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_drag(mut self, val: f32) -> Self {
        self.base.params.insert(
            "drag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_drag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "drag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rigidstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rigidstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigiddampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rigiddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rigiddampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigiddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_internalsolidstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "internalsolidstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_internalsolidstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "internalsolidstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_internalsoliddamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "internalsoliddamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_internalsoliddamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "internalsoliddamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_internalsurfacestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "internalsurfacestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_internalsurfacestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "internalsurfacestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_internalsurfacedamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "internalsurfacedamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_internalsurfacedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "internalsurfacedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disablestretchratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "disablestretchratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disablestretchratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "disablestretchratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_groundpos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "groundpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "groundpos".to_string(),
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

    // --- Float4 parameters ---
    pub fn with_tissuecolor(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "tissuecolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_tissuecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissuecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corecolor(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "corecolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_corecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceattachcolor(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "surfaceattachcolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_surfaceattachcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceattachcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidattachcolor(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "solidattachcolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_solidattachcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidattachcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coretissueattachcolor(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "coretissueattachcolor".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_coretissueattachcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coretissueattachcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_vellumsubsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellumsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellumsubsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellumiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellumiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumsmoothiter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellumsmoothiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellumsmoothiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumsmoothiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumcollisionsiter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vellumcollisionsiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vellumcollisionsiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumcollisionsiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postcollisioniter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "postcollisioniter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_postcollisioniter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postcollisioniter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolveallmax(mut self, val: i32) -> Self {
        self.base.params.insert(
            "resolveallmax".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolveallmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resolveallmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxmultipass(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxmultipass".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxmultipass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxmultipass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vellumintegratortype(
        mut self,
        val: SopTissuesolvervellumVellumintegratortype,
    ) -> Self {
        self.base.params.insert(
            "vellumintegratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumintegratortype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumintegratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingmethod(mut self, val: SopTissuesolvervellumSlidingmethod) -> Self {
        self.base.params.insert(
            "slidingmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slidingmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_collidersoppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collidersoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collidersoppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidersoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidergroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collidergroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collidergroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collidergroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tposeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tposeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tposeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachgeotposeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attachgeotposeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attachgeotposeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attachgeotposeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidpoints(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rigidpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rigidpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingmethodmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "slidingmethodmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_slidingmethodmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slidingmethodmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinstiffnessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pinstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pinstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pinstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablecollisions(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableinternalcollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableinternalcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableinternalcollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableinternalcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablegroundplane(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablegroundplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablegroundplane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablegroundplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolveall(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resolveall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolveall_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resolveall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecollider(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecollider".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecollider_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecollider".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachetodisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cachetodisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetodisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachetodisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerigidgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerigidgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerigidgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerigidgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useslidingmethodmask(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useslidingmethodmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useslidingmethodmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useslidingmethodmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablevolumeoptimization(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablevolumeoptimization".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevolumeoptimization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablevolumeoptimization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepinstiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablepinstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepinstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablepinstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domultipass(mut self, val: bool) -> Self {
        self.base.params.insert(
            "domultipass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domultipass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "domultipass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecollisiongeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidecollisiongeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidecollisiongeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecollisiongeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidetissuecollisionradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidetissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidetissuecollisionradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidetissuecollisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideinternalattach(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guideinternalattach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideinternalattach_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideinternalattach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidesubdermalattach(mut self, val: bool) -> Self {
        self.base.params.insert(
            "guidesubdermalattach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidesubdermalattach_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidesubdermalattach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viztissue(mut self, val: bool) -> Self {
        self.base.params.insert(
            "viztissue".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viztissue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viztissue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vizcore(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vizcore".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vizcore_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vizcore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vizsurfaceattach(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vizsurfaceattach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vizsurfaceattach_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vizsurfaceattach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vizsolidattach(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vizsolidattach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vizsolidattach_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vizsolidattach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vizcoretissueattach(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vizcoretissueattach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vizcoretissueattach_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vizcoretissueattach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTissuesolvervellum {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tissuesolvervellum"
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
pub struct SopToonshaderattribs {
    pub base: crate::core::types::NodeBase,
}

impl SopToonshaderattribs {
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

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_outlinescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "outlinescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outlinescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outlinescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_colorhigh(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "colorhigh".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_colorhigh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "colorhigh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colormid(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "colormid".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_colormid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "colormid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlow(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "colorlow".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_colorlow_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "colorlow".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopToonshaderattribs {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "toonshaderattribs"
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
pub enum SopTopnetOutputtype {
    File = 0,
    GeometryAttribute = 1,
    None = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopnetCreategroups {
    None = 0,
    PointGroups = 1,
    PrimitiveGroups = 2,
    PointAttribute = 3,
    PrimitiveAttribute = 4,
}

#[derive(Debug, Clone)]
pub struct SopTopnet {
    pub base: crate::core::types::NodeBase,
}

impl SopTopnet {
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

    // --- Button parameters ---
    pub fn trigger_forcerecook(mut self) -> Self {
        self.base.params.insert(
            "forcerecook".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_outputtype(mut self, val: SopTopnetOutputtype) -> Self {
        self.base.params.insert(
            "outputtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_creategroups(mut self, val: SopTopnetCreategroups) -> Self {
        self.base.params.insert(
            "creategroups".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_creategroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "creategroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_externaltop(mut self, val: &str) -> Self {
        self.base.params.insert(
            "externaltop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externaltop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externaltop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtag(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputtag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pieceattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pieceattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pieceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groupname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "groupname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_groupname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "groupname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taskgraphfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_taskgraphfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useexternaltop(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useexternaltop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexternaltop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useexternaltop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirtyoncook(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dirtyoncook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dirtyoncook_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dirtyoncook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputmerge(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputmerge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputmerge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputmerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachetaskgraph(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cachetaskgraph".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetaskgraph_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachetaskgraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topnet"
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
pub trait SopTopnetInnerExt {
    fn geometryimport(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopTopnetInnerExt for crate::core::graph::InnerGraph<'a, SopTopnet> {
    fn geometryimport(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("geometryimport")
    }
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("localscheduler")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopnetmgrCheckpointformat {
    /// Python (Deprecated)
    PythonDeprecated = 0,
    Json = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopnetmgrCheckpointload {
    Never = 0,
    OnSceneLoad = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopnetmgrRegenerationtype {
    UpdateWorkItemsAndInvalidateCaches = 0,
    UpdateWorkItemsOnly = 1,
    IgnoreChanges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopnetmgrEvaluationtime {
    NetworkCookTime = 0,
    GlobalStartTime = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct SopTopnetmgr {
    pub base: crate::core::types::NodeBase,
}

impl SopTopnetmgr {
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
    pub fn trigger_generatestatic(mut self) -> Self {
        self.base.params.insert(
            "generatestatic".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookbutton(mut self) -> Self {
        self.base.params.insert(
            "cookbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtybutton(mut self) -> Self {
        self.base.params.insert(
            "dirtybutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cancelbutton(mut self) -> Self {
        self.base.params.insert(
            "cancelbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savetaskgraph(mut self) -> Self {
        self.base.params.insert(
            "savetaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadtaskgraph(mut self) -> Self {
        self.base.params.insert(
            "loadtaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadcheckpoint(mut self) -> Self {
        self.base.params.insert(
            "loadcheckpoint".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_taskgraphsaverate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskgraphsaverate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customtime(mut self, val: i32) -> Self {
        self.base.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_checkpointformat(mut self, val: SopTopnetmgrCheckpointformat) -> Self {
        self.base.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointformat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointload(mut self, val: SopTopnetmgrCheckpointload) -> Self {
        self.base.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regenerationtype(mut self, val: SopTopnetmgrRegenerationtype) -> Self {
        self.base.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_regenerationtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_evaluationtime(mut self, val: SopTopnetmgrEvaluationtime) -> Self {
        self.base.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_evaluationtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_taskgraphfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_taskgraphfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpointfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultlabel(mut self, val: &str) -> Self {
        self.base.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_taskgraphautosave(mut self, val: bool) -> Self {
        self.base.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_taskgraphautosave_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointenabled(mut self, val: bool) -> Self {
        self.base.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointenabled_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savegraphattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savegraphattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedefaultlabel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedefaultlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savescenefile(mut self, val: bool) -> Self {
        self.base.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savescenefile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopnetmgr {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topnetmgr"
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
pub trait SopTopnetmgrInnerExt {
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopTopnetmgrInnerExt for crate::core::graph::InnerGraph<'a, SopTopnetmgr> {
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("localscheduler")
    }
}

#[derive(Debug, Clone)]
pub struct SopTopobuild {
    pub base: crate::core::types::NodeBase,
}

impl SopTopobuild {
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

    /// Connects to input 0: "Mesh Geometry"
    pub fn set_input_mesh_geometry(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Mesh Geometry" and specifies the output index of the target node.
    pub fn set_input_mesh_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Template Geometry"
    pub fn set_input_template_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Template Geometry" and specifies the output index of the target node.
    pub fn set_input_template_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_topobuild_preedit(mut self) -> Self {
        self.base.params.insert(
            "topobuild_preedit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_clearall(mut self) -> Self {
        self.base.params.insert(
            "clearall".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_feedback(mut self, val: &str) -> Self {
        self.base.params.insert(
            "feedback".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_feedback_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "feedback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_edit_tool_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("edit{}_tool", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_edit_tool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("edit{}_tool", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edit_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("edit{}_data", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_edit_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("edit{}_data", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_edit_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("edit{}_enable", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_edit_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("edit{}_enable", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopobuild {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topobuild"
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
pub enum SopTopoflowConstraintselection {
    Auto = 0,
    Manual = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopoflowMaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopoflowFlowmethod {
    Farneback = 0,
    Dis = 1,
}

#[derive(Debug, Clone)]
pub struct SopTopoflow {
    pub base: crate::core::types::NodeBase,
}

impl SopTopoflow {
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

    /// Connects to input 0: "Template Mesh"
    pub fn set_input_template_mesh(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Template Mesh" and specifies the output index of the target node.
    pub fn set_input_template_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Template Texture Constraints"
    pub fn set_input_template_texture_constraints(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Template Texture Constraints" and specifies the output index of the target node.
    pub fn set_input_template_texture_constraints_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Template Mesh Landmarks"
    pub fn set_input_template_mesh_landmarks(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Template Mesh Landmarks" and specifies the output index of the target node.
    pub fn set_input_template_mesh_landmarks_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Target Mesh Landmarks"
    pub fn set_input_target_mesh_landmarks(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Target Mesh Landmarks" and specifies the output index of the target node.
    pub fn set_input_target_mesh_landmarks_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_respercentage(mut self, val: f32) -> Self {
        self.base.params.insert(
            "respercentage".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_respercentage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "respercentage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alphacutoff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "alphacutoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alphacutoff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alphacutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windowradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "windowradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_constraintselection(mut self, val: SopTopoflowConstraintselection) -> Self {
        self.base.params.insert(
            "constraintselection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_constraintselection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskmode(mut self, val: SopTopoflowMaskmode) -> Self {
        self.base.params.insert(
            "maskmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flowmethod(mut self, val: SopTopoflowFlowmethod) -> Self {
        self.base.params.insert(
            "flowmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_flowmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "flowmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcegeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcegeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srctexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "srctexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srctexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srctexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tgttexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tgttexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tgttexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tgttexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintsource(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraintsource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraintsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_landmarkattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "landmarkattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_landmarkattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "landmarkattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidprimitives(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rigidprimitives".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rigidprimitives_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidprimitives".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rigidmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rigidmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablegeometryconstraints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablegeometryconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablegeometryconstraints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablegeometryconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopoflow {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topoflow"
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
pub struct SopTopoflowbake {
    pub base: crate::core::types::NodeBase,
}

impl SopTopoflowbake {
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

    /// Connects to input 0: "Source Geometry"
    pub fn set_input_source_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source Geometry" and specifies the output index of the target node.
    pub fn set_input_source_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Target Geometry"
    pub fn set_input_target_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Target Geometry" and specifies the output index of the target node.
    pub fn set_input_target_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_alphacutoff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "alphacutoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alphacutoff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alphacutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_inres(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "inres".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_inres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outres(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "outres".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_outres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcetexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcetexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcetexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_resample(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputgrayscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputgrayscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputgrayscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputgrayscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopoflowbake {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topoflowbake"
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
pub enum SopTopoflowsampleMethod {
    CornerDetection = 0,
    MinimumEigenValue = 1,
}

#[derive(Debug, Clone)]
pub struct SopTopoflowsample {
    pub base: crate::core::types::NodeBase,
}

impl SopTopoflowsample {
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
    pub fn with_qualitytolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "qualitytolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_qualitytolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "qualitytolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cornerweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cornerweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cornerweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cornerweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxfeaturesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxfeaturesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxfeaturesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxfeaturesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimumspacing(mut self, val: i32) -> Self {
        self.base.params.insert(
            "minimumspacing".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumspacing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minimumspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blurringwindowradius(mut self, val: i32) -> Self {
        self.base.params.insert(
            "blurringwindowradius".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blurringwindowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blurringwindowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradientwindowradius(mut self, val: i32) -> Self {
        self.base.params.insert(
            "gradientwindowradius".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_gradientwindowradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradientwindowradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopTopoflowsampleMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_texture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopoflowsample {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topoflowsample"
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
pub struct SopTopolandmark {
    pub base: crate::core::types::NodeBase,
}

impl SopTopolandmark {
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

    /// Connects to input 0: "Template Mesh"
    pub fn set_input_template_mesh(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Template Mesh" and specifies the output index of the target node.
    pub fn set_input_template_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Target Mesh"
    pub fn set_input_target_mesh(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Target Mesh" and specifies the output index of the target node.
    pub fn set_input_target_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Template Points"
    pub fn set_input_template_points(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Template Points" and specifies the output index of the target node.
    pub fn set_input_template_points_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Target Points"
    pub fn set_input_target_points(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Target Points" and specifies the output index of the target node.
    pub fn set_input_target_points_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Data parameters ---
    pub fn with_sourcestash(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourcestash".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_sourcestash_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourcestash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstash(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetstash".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_targetstash_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetstash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sourceoutput(mut self, val: i32) -> Self {
        self.base.params.insert(
            "sourceoutput".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sourceoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourceoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetoutput(mut self, val: i32) -> Self {
        self.base.params.insert(
            "targetoutput".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_targetoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_labelattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "labelattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_labelattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "labelattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_labelprefix(mut self, val: &str) -> Self {
        self.base.params.insert(
            "labelprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_labelprefix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "labelprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopolandmark {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topolandmark"
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
pub enum SopToposlidebycurverefsSolvemode {
    /// Topo Transfer (accurate)
    TopoTransferAccurate = 0,
    /// Slide (fast)
    SlideFast = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopToposlidebycurverefsVismode {
    Source = 0,
    Target = 1,
}

#[derive(Debug, Clone)]
pub struct SopToposlidebycurverefs {
    pub base: crate::core::types::NodeBase,
}

impl SopToposlidebycurverefs {
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

    /// Connects to input 0: "Target Geometry"
    pub fn set_input_target_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Target Geometry" and specifies the output index of the target node.
    pub fn set_input_target_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Reference Geometry"
    pub fn set_input_reference_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Reference Geometry" and specifies the output index of the target node.
    pub fn set_input_reference_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Reference Curves"
    pub fn set_input_reference_curves(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Reference Curves" and specifies the output index of the target node.
    pub fn set_input_reference_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Target Curves"
    pub fn set_input_target_curves(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Target Curves" and specifies the output index of the target node.
    pub fn set_input_target_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_editsourcecurves(mut self) -> Self {
        self.base.params.insert(
            "editsourcecurves".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_edittargetcurves(mut self) -> Self {
        self.base.params.insert(
            "edittargetcurves".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blurstepsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blurstepsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blurstepsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blurstepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bluriterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bluriterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bluriterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bluriterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_solvemode(mut self, val: SopToposlidebycurverefsSolvemode) -> Self {
        self.base.params.insert(
            "solvemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solvemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vismode(mut self, val: SopToposlidebycurverefsVismode) -> Self {
        self.base.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vismode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "maskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiusattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "radiusattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_radiusattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radiusattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_matchcurves(mut self, val: bool) -> Self {
        self.base.params.insert(
            "matchcurves".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchcurves_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchcurves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_editsetflags(mut self, val: bool) -> Self {
        self.base.params.insert(
            "editsetflags".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_editsetflags_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "editsetflags".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaskattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaskattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaskattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskbyradius(mut self, val: bool) -> Self {
        self.base.params.insert(
            "maskbyradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskbyradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskbyradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useradiusattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useradiusattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useradiusattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useradiusattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablevis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablevis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablevis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showvisarrows(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showvisarrows".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showvisarrows_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showvisarrows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopToposlidebycurverefs {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "toposlidebycurverefs"
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
pub enum SopTopotransferConstraintselection {
    Auto = 0,
    Manual = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopotransferMaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopotransferSolvertype {
    NonLinear = 0,
    Linear = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTopotransferDebugMenu {
    None = 0,
    Coarse = 1,
    Dense = 2,
}

#[derive(Debug, Clone)]
pub struct SopTopotransfer {
    pub base: crate::core::types::NodeBase,
}

impl SopTopotransfer {
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

    /// Connects to input 0: "Template Mesh"
    pub fn set_input_template_mesh(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Template Mesh" and specifies the output index of the target node.
    pub fn set_input_template_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Target Mesh"
    pub fn set_input_target_mesh(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Target Mesh" and specifies the output index of the target node.
    pub fn set_input_target_mesh_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Template Mesh Landmarks"
    pub fn set_input_template_mesh_landmarks(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Template Mesh Landmarks" and specifies the output index of the target node.
    pub fn set_input_template_mesh_landmarks_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Target Mesh Landmarks"
    pub fn set_input_target_mesh_landmarks(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Target Mesh Landmarks" and specifies the output index of the target node.
    pub fn set_input_target_mesh_landmarks_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_pstatebutton_inst(mut self, index1: usize) -> Self {
        self.base.params.insert(
            format!("pstatebutton{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_reductionpercentage_single(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reductionpercentage_single".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reductionpercentage_single_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reductionpercentage_single".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialreductionpercentage(mut self, val: f32) -> Self {
        self.base.params.insert(
            "initialreductionpercentage".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initialreductionpercentage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialreductionpercentage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_masktolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "masktolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_masktolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "masktolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disttolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "disttolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disttolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "disttolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normtolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "normtolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normtolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normtolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parameter_tolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "parameter_tolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_parameter_tolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parameter_tolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tau(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tau".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tau_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tau".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initial_damping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "initial_damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initial_damping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initial_damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_reductionpercentage(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "reductionpercentage".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_reductionpercentage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reductionpercentage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidweights(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "rigidweights".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_rigidweights_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_landmarkweights(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "landmarkweights".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_landmarkweights_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "landmarkweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradienttolerance(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "gradienttolerance".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_gradienttolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradienttolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pballcolor_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("pballcolor{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pballcolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pballcolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reducedlevels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "reducedlevels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reducedlevels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reducedlevels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solveriterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "solveriterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_solveriterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solveriterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_coarse_lvl(mut self, val: i32) -> Self {
        self.base.params.insert(
            "debug_coarse_lvl".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_debug_coarse_lvl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_coarse_lvl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_coarse_iteration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "debug_coarse_iteration".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_debug_coarse_iteration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_coarse_iteration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_dense_lvl(mut self, val: i32) -> Self {
        self.base.params.insert(
            "debug_dense_lvl".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_debug_dense_lvl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_dense_lvl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_dense_iteration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "debug_dense_iteration".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_debug_dense_iteration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_dense_iteration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_constraintselection(mut self, val: SopTopotransferConstraintselection) -> Self {
        self.base.params.insert(
            "constraintselection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_constraintselection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskmode(mut self, val: SopTopotransferMaskmode) -> Self {
        self.base.params.insert(
            "maskmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvertype(mut self, val: SopTopotransferSolvertype) -> Self {
        self.base.params.insert(
            "solvertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solvertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solvertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_menu(mut self, val: SopTopotransferDebugMenu) -> Self {
        self.base.params.insert(
            "debug_menu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_debug_menu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_menu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constraintsource(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraintsource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraintsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraintsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rigidmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rigidmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidprimitives(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rigidprimitives".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rigidprimitives_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidprimitives".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_landmarkattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "landmarkattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_landmarkattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "landmarkattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_psourcegroup_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("psourcegroup{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_psourcegroup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("psourcegroup{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcepos_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("sourcepos{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcepos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("sourcepos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptargetgroup_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("ptargetgroup{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptargetgroup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("ptargetgroup{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpos_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("targetpos{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("targetpos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablesolve(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesolve".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesolve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablegeometryconstraints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablegeometryconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablegeometryconstraints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablegeometryconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_hessian_scaling(mut self, val: bool) -> Self {
        self.base.params.insert(
            "debug_hessian_scaling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_debug_hessian_scaling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_hessian_scaling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_use_marquadt(mut self, val: bool) -> Self {
        self.base.params.insert(
            "debug_use_marquadt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_debug_use_marquadt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_use_marquadt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_tau(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_tau".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_tau_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_tau".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug_save_meshes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "debug_save_meshes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_debug_save_meshes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debug_save_meshes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselandmarklabels(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uselandmarklabels".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselandmarklabels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uselandmarklabels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablelandmarks(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablelandmarks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablelandmarks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablelandmarks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablegroup_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("enablegroup{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablegroup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("enablegroup{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesourcepos_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("usesourcepos{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesourcepos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("usesourcepos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetargetpos_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("usetargetpos{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetpos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("usetargetpos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTopotransfer {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "topotransfer"
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
pub enum SopTorusType {
    Polygon = 0,
    Mesh = 1,
    Nurbs = 2,
    Bezier = 3,
    PolygonSoup = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTorusSurftype {
    Rows = 0,
    Columns = 1,
    RowsAndColumns = 2,
    Triangles = 3,
    Quadrilaterals = 4,
    AlternatingTriangles = 5,
    ReverseTriangles = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTorusOrient {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone)]
pub struct SopTorus {
    pub base: crate::core::types::NodeBase,
}

impl SopTorus {
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

    // --- Float2 parameters ---
    pub fn with_rad(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angleu(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "angleu".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_angleu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angleu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anglev(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "anglev".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_anglev_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "anglev".to_string(),
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

    // --- Int parameters ---
    pub fn with_rows(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("rows".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_rows_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cols(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("cols".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_cols_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderu(mut self, val: i32) -> Self {
        self.base.params.insert(
            "orderu".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_orderu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orderu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderv(mut self, val: i32) -> Self {
        self.base.params.insert(
            "orderv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_orderv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orderv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopTorusType) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_surftype(mut self, val: SopTorusSurftype) -> Self {
        self.base.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surftype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orient(mut self, val: SopTorusOrient) -> Self {
        self.base.params.insert(
            "orient".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_imperfect(mut self, val: bool) -> Self {
        self.base.params.insert(
            "imperfect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_imperfect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "imperfect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeu(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closeu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closeu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closev(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closev".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closev_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capu(mut self, val: bool) -> Self {
        self.base.params.insert(
            "capu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_capu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "capu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "capv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_capv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "capv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTorus {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "torus"
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
pub enum SopTraceChannel {
    Mono = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Alpha = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTraceBoundary {
    Black = 0,
    White = 1,
    /// Same as (0,0)
    SameAs00 = 2,
    Custom = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTraceMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone)]
pub struct SopTrace {
    pub base: crate::core::types::NodeBase,
}

impl SopTrace {
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

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.base
            .params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_thresh(mut self, val: f32) -> Self {
        self.base.params.insert(
            "thresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thresh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "copframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_copframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_step(mut self, val: f32) -> Self {
        self.base.params.insert(
            "step".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_step_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "step".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corner(mut self, val: f32) -> Self {
        self.base.params.insert(
            "corner".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_corner_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_error(mut self, val: f32) -> Self {
        self.base.params.insert(
            "error".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_error_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "error".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lod(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundaryvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "boundaryvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_boundaryvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundaryvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
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

    // --- Int parameters ---
    pub fn with_bordwidth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bordwidth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bordwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bordwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_imagesize(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "imagesize".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_imagesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "imagesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_channel(mut self, val: SopTraceChannel) -> Self {
        self.base.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary(mut self, val: SopTraceBoundary) -> Self {
        self.base.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingframe(mut self, val: SopTraceMissingframe) -> Self {
        self.base.params.insert(
            "missingframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "missingframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_tracelayer(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tracelayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tracelayer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tracelayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "coppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copcolor(mut self, val: &str) -> Self {
        self.base.params.insert(
            "copcolor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copalpha(mut self, val: &str) -> Self {
        self.base.params.insert(
            "copalpha".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copalpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addtexture(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addtexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delborder(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delborder".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delborder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delborder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doresample(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doresample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doresample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doresample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosmooth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dosmooth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosmooth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dosmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fitcurve(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fitcurve".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fitcurve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fitcurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convpoly(mut self, val: bool) -> Self {
        self.base.params.insert(
            "convpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convpoly_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "convpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hole(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hole".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hole_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hole".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTrace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "trace"
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
pub enum SopTrailResult {
    PreserveOriginal = 0,
    ConnectAsMesh = 1,
    ConnectAsPolygons = 2,
    ComputeVelocity = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTrailSurftype {
    Rows = 0,
    Columns = 1,
    RowsAndColumns = 2,
    Triangles = 3,
    Quadrilaterals = 4,
    AlternatingTriangles = 5,
    ReverseTriangles = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTrailVelapproximation {
    BackwardDifference = 0,
    CentralDifference = 1,
    ForwardDifference = 2,
}

#[derive(Debug, Clone)]
pub struct SopTrail {
    pub base: crate::core::types::NodeBase,
}

impl SopTrail {
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

    // --- Button parameters ---
    pub fn trigger_clear(mut self) -> Self {
        self.base
            .params
            .insert("clear".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_inc(mut self, val: f32) -> Self {
        self.base.params.insert(
            "inc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inc".to_string(),
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

    // --- Int parameters ---
    pub fn with_length(mut self, val: i32) -> Self {
        self.base.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_cache(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cache".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_result(mut self, val: SopTrailResult) -> Self {
        self.base.params.insert(
            "result".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_result_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "result".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surftype(mut self, val: SopTrailSurftype) -> Self {
        self.base.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surftype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velapproximation(mut self, val: SopTrailVelapproximation) -> Self {
        self.base.params.insert(
            "velapproximation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velapproximation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velapproximation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_accelattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "accelattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_accelattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "accelattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributetomatch(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attributetomatch".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributetomatch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attributetomatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_evalframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "evalframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_evalframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "evalframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_close(mut self, val: bool) -> Self {
        self.base.params.insert(
            "close".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_close_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "close".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeaccel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computeaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeaccel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computeaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeangular(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computeangular".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeangular_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computeangular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchbyattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "matchbyattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchbyattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTrail {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "trail"
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
pub enum SopTriangulate2dPlanepossrc {
    FitPlane = 0,
    SelectProjectionPlane = 1,
    UseAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTriangulate2dTrianglesize {
    Unrestricted = 0,
    CapMaximumArea = 1,
    SetTargetEdgeLength = 2,
}

#[derive(Debug, Clone)]
pub struct SopTriangulate2d {
    pub base: crate::core::types::NodeBase,
}

impl SopTriangulate2d {
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

    // --- Float parameters ---
    pub fn with_dist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_encroachangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "encroachangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_encroachangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "encroachangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxarea(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxarea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxarea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetedgelength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "targetedgelength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetedgelength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetedgelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minedgelength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minedgelength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minedgelength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minedgelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_origin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "origin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_origin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "origin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxnewpts(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxnewpts".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxnewpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxnewpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lloydsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "lloydsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lloydsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lloydsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "randseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_randseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_planepossrc(mut self, val: SopTriangulate2dPlanepossrc) -> Self {
        self.base.params.insert(
            "planepossrc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planepossrc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "planepossrc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trianglesize(mut self, val: SopTriangulate2dTrianglesize) -> Self {
        self.base.params.insert(
            "trianglesize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trianglesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trianglesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_points(mut self, val: &str) -> Self {
        self.base.params.insert(
            "points".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_points_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "points".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos2attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pos2attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pos2attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos2attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constredges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constredges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constredges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constredges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrpolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constrpolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constrpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_silhouettepolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "silhouettepolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_silhouettepolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "silhouettepolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrsplitptgrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constrsplitptgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrsplitptgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constrsplitptgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrdedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constrdedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrdedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constrdedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useconstredges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useconstredges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useconstredges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useconstredges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useconstrpolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useconstrpolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useconstrpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useconstrpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignorepolybridges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ignorepolybridges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignorepolybridges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ignorepolybridges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesilhouettepolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesilhouettepolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesilhouettepolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesilhouettepolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowconstrsplit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allowconstrsplit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowconstrsplit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allowconstrsplit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexactconstruction(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useexactconstruction".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexactconstruction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useexactconstruction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignorenonconstrpts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ignorenonconstrpts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignorenonconstrpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ignorenonconstrpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removefromconvexhull(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removefromconvexhull".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removefromconvexhull_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removefromconvexhull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removefromconstrpolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removefromconstrpolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removefromconstrpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removefromconstrpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeoutsidesilhouette(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeoutsidesilhouette".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeoutsidesilhouette_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeoutsidesilhouette".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refine(mut self, val: bool) -> Self {
        self.base.params.insert(
            "refine".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_refine_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowrefineonstrsplit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allowrefineonstrsplit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowrefineonstrsplit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allowrefineonstrsplit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowmovinginteriorpts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allowmovinginteriorpts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowmovinginteriorpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allowmovinginteriorpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restorepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "restorepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restorepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restorepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepprims(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepprims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatenmls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updatenmls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeunusedpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeunusedpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeunusedpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeunusedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeduplicatepoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeduplicatepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeduplicatepoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeduplicatepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecontrsplitptgrp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecontrsplitptgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecontrsplitptgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecontrsplitptgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useconstrdedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useconstrdedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useconstrdedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useconstrdedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTriangulate2d {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "triangulate2d"
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
pub struct SopTribez {
    pub base: crate::core::types::NodeBase,
}

impl SopTribez {
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

    // --- Int parameters ---
    pub fn with_order(mut self, val: i32) -> Self {
        self.base.params.insert(
            "order".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_order_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "order".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopTribez {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tribez"
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
pub struct SopTridivide {
    pub base: crate::core::types::NodeBase,
}

impl SopTridivide {
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

    /// Connects to input 0: "Geometry to Divide"
    pub fn set_input_geometry_to_divide(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Divide" and specifies the output index of the target node.
    pub fn set_input_geometry_to_divide_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Reference For Edge Lengths"
    pub fn set_input_reference_for_edge_lengths(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Reference For Edge Lengths" and specifies the output index of the target node.
    pub fn set_input_reference_for_edge_lengths_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_minarea(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minarea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minarea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minedge(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minedge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "size".to_string(),
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

    // --- FloatArray parameters ---
    pub fn with_projmatrix(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "projmatrix".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_projmatrix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projmatrix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_root3iter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "root3iter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_root3iter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "root3iter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numsplits(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numsplits".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numsplits_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numsplits".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_convex(mut self, val: bool) -> Self {
        self.base.params.insert(
            "convex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "convex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dominarea(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dominarea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dominarea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dominarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doprojmatrix(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doprojmatrix".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doprojmatrix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doprojmatrix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doboundingbox(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doboundingbox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doboundingbox_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doboundingbox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doexpand(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doexpand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doexpand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doexpand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dominedge(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dominedge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dominedge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dominedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_donumsplits(mut self, val: bool) -> Self {
        self.base.params.insert(
            "donumsplits".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donumsplits_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "donumsplits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTridivide {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tridivide"
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
pub enum SopTrimOptype {
    KeepOutside = 0,
    KeepInside = 1,
    KeepNatural = 2,
    Untrim = 3,
    ChangeAltitude = 4,
}

#[derive(Debug, Clone)]
pub struct SopTrim {
    pub base: crate::core::types::NodeBase,
}

impl SopTrim {
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

    /// Connects to input 0: "Spline Surfaces"
    pub fn set_input_spline_surfaces(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Spline Surfaces" and specifies the output index of the target node.
    pub fn set_input_spline_surfaces_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_trimtol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "trimtol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trimtol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trimtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_altitude(mut self, val: i32) -> Self {
        self.base.params.insert(
            "altitude".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_altitude_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "altitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_optype(mut self, val: SopTrimOptype) -> Self {
        self.base.params.insert(
            "optype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_optype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optype".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_individual(mut self, val: bool) -> Self {
        self.base.params.insert(
            "individual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_individual_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "individual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bigloop(mut self, val: bool) -> Self {
        self.base.params.insert(
            "bigloop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bigloop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bigloop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTrim {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "trim"
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
pub struct SopTristrip {
    pub base: crate::core::types::NodeBase,
}

impl SopTristrip {
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

    /// Connects to input 0: "Geometry to tristrip"
    pub fn set_input_geometry_to_tristrip(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to tristrip" and specifies the output index of the target node.
    pub fn set_input_geometry_to_tristrip_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_maxstriplength(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxstriplength".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxstriplength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxstriplength".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_constrainstriplength(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constrainstriplength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainstriplength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constrainstriplength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTristrip {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tristrip"
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
pub enum SopTubeType {
    Primitive = 0,
    Polygon = 1,
    Mesh = 2,
    Nurbs = 3,
    Bezier = 4,
    PolygonSoup = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTubeSurftype {
    Rows = 0,
    Columns = 1,
    RowsAndColumns = 2,
    Triangles = 3,
    Quadrilaterals = 4,
    AlternatingTriangles = 5,
    ReverseTriangles = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTubeOrient {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone)]
pub struct SopTube {
    pub base: crate::core::types::NodeBase,
}

impl SopTube {
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

    /// Connects to input 0: "Bounding Source"
    pub fn set_input_bounding_source(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Bounding Source" and specifies the output index of the target node.
    pub fn set_input_bounding_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_radscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_height(mut self, val: f32) -> Self {
        self.base.params.insert(
            "height".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "height".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_rad(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad".to_string(),
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

    // --- Int parameters ---
    pub fn with_rows(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("rows".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_rows_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cols(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("cols".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_cols_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cols".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderu(mut self, val: i32) -> Self {
        self.base.params.insert(
            "orderu".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_orderu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orderu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderv(mut self, val: i32) -> Self {
        self.base.params.insert(
            "orderv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_orderv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orderv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopTubeType) -> Self {
        self.base.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_surftype(mut self, val: SopTubeSurftype) -> Self {
        self.base.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surftype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orient(mut self, val: SopTubeOrient) -> Self {
        self.base.params.insert(
            "orient".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_cap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_consolidatepts(mut self, val: bool) -> Self {
        self.base.params.insert(
            "consolidatepts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_consolidatepts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "consolidatepts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vertexnormals(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vertexnormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vertexnormals_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertexnormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imperfect(mut self, val: bool) -> Self {
        self.base.params.insert(
            "imperfect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_imperfect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "imperfect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopTube {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "tube"
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
pub enum SopTwistOp {
    Twist = 0,
    Bend = 1,
    Shear = 2,
    Taper = 3,
    LinearTaper = 4,
    /// Squash & Stretch
    SquashStretch = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTwistPaxis {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopTwistSaxis {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone)]
pub struct SopTwist {
    pub base: crate::core::types::NodeBase,
}

impl SopTwist {
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

    /// Connects to input 0: "Twist source"
    pub fn set_input_twist_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Twist source" and specifies the output index of the target node.
    pub fn set_input_twist_source_from(
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

    // --- Float3 parameters ---
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

    // --- Menu parameters ---
    pub fn with_op(mut self, val: SopTwistOp) -> Self {
        self.base.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_paxis(mut self, val: SopTwistPaxis) -> Self {
        self.base.params.insert(
            "paxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_paxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "paxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_saxis(mut self, val: SopTwistSaxis) -> Self {
        self.base.params.insert(
            "saxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_saxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "saxis".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopTwist {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "twist"
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
